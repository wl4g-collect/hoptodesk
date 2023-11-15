#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::two_factor_auth::sockets::{AuthAnswer};

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
use std::iter::FromIterator;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use std::sync::Arc;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicI64, Ordering},
        RwLock,
    },
};

use tokio::task;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::ipc::Connection;
#[cfg(not(any(target_os = "ios")))]
use crate::ipc::{self, Data};
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use clipboard::ContextSend;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use hbb_common::tokio::sync::mpsc::unbounded_channel;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use hbb_common::tokio::sync::Mutex as TokioMutex;
use hbb_common::{
    allow_err,
    config::Config,
    fs::is_write_need_confirmation,
    fs::{self, get_string, new_send_confirm, DigestCheckResult},
    log,
    message_proto::*,
    protobuf::Message as _,
    tokio::{
        self,
        sync::mpsc::{self, UnboundedSender},
        task::spawn_blocking,
    },
    ResultType,
};
use serde_derive::Serialize;

#[derive(Serialize, Clone)]
pub struct Client {
    pub id: i32,
    pub authorized: bool,
    pub disconnected: bool,
    pub is_file_transfer: bool,
    pub port_forward: String,
    pub name: String,
    pub peer_id: String,
    pub keyboard: bool,
    pub clipboard: bool,
    pub audio: bool,
    pub file: bool,
    pub restart: bool,
    pub recording: bool,
    pub block_input: bool,
    pub from_switch: bool,
    pub in_voice_call: bool,
    pub incoming_voice_call: bool,
    #[serde(skip)]
    #[cfg(not(any(target_os = "ios")))]
    tx: UnboundedSender<Data>,
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
struct IpcTaskRunner<T: InvokeUiCM> {
    stream: Connection,
    cm: ConnectionManager<T>,
    tx: mpsc::UnboundedSender<Data>,
    rx: mpsc::UnboundedReceiver<Data>,
    close: bool,
    running: bool,
    conn_id: i32,
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    file_transfer_enabled: bool,
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    file_transfer_enabled_peer: bool,
}

lazy_static::lazy_static! {
    static ref CLIENTS: RwLock<HashMap<i32, Client>> = Default::default();
    static ref CLICK_TIME: AtomicI64 = AtomicI64::new(0);
}

#[derive(Clone)]
pub struct ConnectionManager<T: InvokeUiCM> {
    pub ui_handler: T,
}

pub trait InvokeUiCM: Send + Clone + 'static + Sized {
    fn add_connection(&self, client: &Client, security_numbers: String, avatar_image: String);

    fn remove_connection(&self, id: i32, close: bool);

    fn new_message(&self, id: i32, text: String);

    fn change_theme(&self, dark: String);

    fn change_language(&self);
	
	#[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn update_2fa_answer(&self, answer: AuthAnswer);
    fn show_elevation(&self, show: bool);    
    
    fn update_voice_call_state(&self, client: &Client);

    fn file_transfer_log(&self, action: &str, log: &str);
}

impl<T: InvokeUiCM> Deref for ConnectionManager<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ui_handler
    }
}

impl<T: InvokeUiCM> DerefMut for ConnectionManager<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui_handler
    }
}

impl<T: InvokeUiCM> ConnectionManager<T> {
    fn add_connection(
        &self,
        id: i32,
        is_file_transfer: bool,
        port_forward: String,
        peer_id: String,
        name: String,
        authorized: bool,
        keyboard: bool,
        clipboard: bool,
        audio: bool,
        file: bool,
        restart: bool,
        recording: bool,
        block_input: bool,
        from_switch: bool,
        #[cfg(not(any(target_os = "ios")))] tx: mpsc::UnboundedSender<Data>,
        security_numbers: String,
        avatar_image: String
    ) {
        let client = Client {
            id,
            authorized,
            disconnected: false,
            is_file_transfer,
            port_forward,
            name: name.clone(),
            peer_id: peer_id.clone(),
            keyboard,
            clipboard,
            audio,
            file,
            restart,
            recording,
            block_input,
            from_switch,
        	#[cfg(not(any(target_os = "ios")))]
            tx,
            in_voice_call: false,
            incoming_voice_call: false,
        };
        CLIENTS
            .write()
            .unwrap()
            .retain(|_, c| !(c.disconnected && c.peer_id == client.peer_id));
        CLIENTS.write().unwrap().insert(id, client.clone());
        self.ui_handler.add_connection(&client, security_numbers, avatar_image);
    }

    #[inline]
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    fn is_authorized(&self, id: i32) -> bool {
        CLIENTS
            .read()
            .unwrap()
            .get(&id)
            .map(|c| c.authorized)
            .unwrap_or(false)
    }
    
    fn remove_connection(&self, id: i32, close: bool) {
        if close {
            CLIENTS.write().unwrap().remove(&id);
        } else {
            CLIENTS
                .write()
                .unwrap()
                .get_mut(&id)
                .map(|c| c.disconnected = true);
        }

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            let _ = ContextSend::proc(|context| -> ResultType<()> {
                context.empty_clipboard(id)?;
                Ok(())
            });
        }

        #[cfg(any(target_os = "android"))]
        if CLIENTS
            .read()
            .unwrap()
            .iter()
            .filter(|(_k, v)| !v.is_file_transfer)
            .next()
            .is_none()
        {
            if let Err(e) =
                scrap::android::call_main_service_set_by_name("stop_capture", None, None)
            {
                log::debug!("stop_capture err:{}", e);
            }
        }

        self.ui_handler.remove_connection(id, close);
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn show_elevation(&self, show: bool) {
        self.ui_handler.show_elevation(show);
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn voice_call_started(&self, id: i32) {
        if let Some(client) = CLIENTS.write().unwrap().get_mut(&id) {
            client.incoming_voice_call = false;
            client.in_voice_call = true;
            self.ui_handler.update_voice_call_state(client);
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn voice_call_incoming(&self, id: i32) {
        if let Some(client) = CLIENTS.write().unwrap().get_mut(&id) {
            client.incoming_voice_call = true;
            client.in_voice_call = false;
            self.ui_handler.update_voice_call_state(client);
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn voice_call_closed(&self, id: i32, _reason: &str) {
        if let Some(client) = CLIENTS.write().unwrap().get_mut(&id) {
            client.incoming_voice_call = false;
            client.in_voice_call = false;
            self.ui_handler.update_voice_call_state(client);
        }
    }
}

#[inline]
#[cfg(not(any(target_os = "ios")))]
pub fn check_click_time(id: i32) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        allow_err!(client.tx.send(Data::ClickTime(0)));
    };
}

#[inline]
pub fn get_click_time() -> i64 {
    CLICK_TIME.load(Ordering::SeqCst)
}

#[inline]
#[cfg(not(any(target_os = "ios")))]
pub fn authorize(id: i32) {
    if let Some(client) = CLIENTS.write().unwrap().get_mut(&id) {
        client.authorized = true;
        allow_err!(client.tx.send(Data::Authorize));
    };
}

#[inline]
#[cfg(not(any(target_os = "ios")))]
pub fn close(id: i32) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        allow_err!(client.tx.send(Data::Close));
    };
}

#[inline]
pub fn remove(id: i32) {
	CLIENTS.write().unwrap().remove(&id);
}

fn close_incoming(id: &str) {
    CLIENTS.write().unwrap().retain(|_, client| {
        if client.peer_id == id {
            allow_err!(client.tx.send(Data::Close));
            return false;
        }
        true
    });
}


async fn tell_sessions(body: String, myid: String, client_ids: String) {
	let url = format!(
        "https://api.hoptodesk.com/?teamid={}&id={}&clientidslist={}", body, myid, client_ids
    );
	let response = reqwest::get(&url)
        .await
        .expect("Error making HTTP GET request");
    let response_text = response.text().await.expect("Error reading API response.");

    log::info!("Reported Sessions Response for {}, Sessions: {}, Response: {}", myid, client_ids, response_text);
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn list_sessions(_id: &str) {
    let clients = CLIENTS.read().unwrap();
    let mut client_ids = String::new();

    for (_, client) in clients.iter() {
        if !client_ids.is_empty() {
            client_ids.push(',');
        }
        client_ids.push_str(&client.peer_id);
    }

    if !clients.is_empty() {
        let mut client_ids = String::new();

        for (_, client) in clients.iter() {
            if !client_ids.is_empty() {
                client_ids.push(',');
            }
            client_ids.push_str(&client.peer_id);
        }

        let myid = Config::get_id();
        let body = std::fs::read_to_string(Config::path("TeamID.toml"))
            .unwrap_or_else(|err| {
                eprintln!("Error reading file: {}", err);
                String::new()
            });
		if !client_ids.is_empty() {
			task::spawn(async move {
				tell_sessions(body, myid, client_ids).await;
			});
		}
    }
}



// server mode send chat to peer
#[inline]
#[cfg(not(any(target_os = "ios")))]
pub fn send_chat(id: i32, text: String) {
	let clients = CLIENTS.read().unwrap();
    if let Some(client) = clients.get(&id) {
		allow_err!(client.tx.send(Data::ChatMessage { text }));
    }
}

#[inline]
#[cfg(not(any(target_os = "ios")))]
pub fn switch_permission(id: i32, name: String, enabled: bool) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        allow_err!(client.tx.send(Data::SwitchPermission { name, enabled }));
    };
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
#[inline]
pub fn get_clients_state() -> String {
    let clients = CLIENTS.read().unwrap();
    let res = Vec::from_iter(clients.values().cloned());
    serde_json::to_string(&res).unwrap_or("".into())
}

#[inline]
pub fn get_clients_length() -> usize {
    let clients = CLIENTS.read().unwrap();
    clients.len()
}

#[inline]
#[cfg(feature = "flutter")]
#[cfg(not(any(target_os = "ios")))]
pub fn switch_back(id: i32) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        allow_err!(client.tx.send(Data::SwitchSidesBack));
    };
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
impl<T: InvokeUiCM> IpcTaskRunner<T> {
    async fn run(&mut self) {
        //use hbb_common::config::LocalConfig;

        // for tmp use, without real conn id
        let mut write_jobs: Vec<fs::TransferJob> = Vec::new();
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let is_authorized = self.cm.is_authorized(self.conn_id);

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        let rx_clip1;
        let mut rx_clip;
        let _tx_clip;
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        if self.conn_id > 0 && is_authorized {
            //log::debug!("Clipboard is enabled from client peer: type 1");
            rx_clip1 = clipboard::get_rx_cliprdr_server(self.conn_id);
            rx_clip = rx_clip1.lock().await;
        } else {
            //log::debug!("Clipboard is enabled from client peer, actually useless: type 2");
            let rx_clip2;
            (_tx_clip, rx_clip2) = unbounded_channel::<clipboard::ClipboardFile>();
            rx_clip1 = Arc::new(TokioMutex::new(rx_clip2));
            rx_clip = rx_clip1.lock().await;
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            (_tx_clip, rx_clip) = unbounded_channel::<i32>();
        }

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            if ContextSend::is_enabled() {
                allow_err!(
                    self.stream
                        .send(&Data::ClipboardFile(clipboard::ClipboardFile::MonitorReady))
                        .await
                );
            }
        }
        let (tx_log, _rx_log) = mpsc::unbounded_channel::<String>();

        self.running = false;
        loop {
            tokio::select! {
                res = self.stream.next() => {
                    match res {
                        Err(err) => {
                            log::info!("cm ipc connection closed: {}", err);
                            break;
                        }
                        Ok(Some(data)) => {
                            match data {
								Data::TFA { id, answer } => {
									self.cm.update_2fa_answer(answer);
								}
                                Data::Login{id, is_file_transfer, port_forward, peer_id, name, authorized, keyboard, clipboard, audio, file, file_transfer_enabled: _file_transfer_enabled, restart, recording, block_input, from_switch, security_numbers, avatar_image} => {
                                    log::debug!("conn_id: {}", id);
                                    self.cm.add_connection(id, is_file_transfer, port_forward, peer_id, name, authorized, keyboard, clipboard, audio, file, restart, recording, block_input, from_switch, self.tx.clone(), security_numbers, avatar_image);
                                    self.conn_id = id;
                                    #[cfg(windows)]
                                    {
                                        self.file_transfer_enabled = _file_transfer_enabled;
                                    }
                                    self.running = true;
                                    break;
                                }
                                Data::Close => {
                                    log::info!("cm ipc connection closed from connection request");
                                    break;
                                }
                                Data::CloseIncoming { id } => {
                                    close_incoming(&id);
                                }
                                #[cfg(not(any(target_os = "android", target_os = "ios")))]
								Data::ListSessions { id } => {
									list_sessions(&id);
                                }								
                                Data::Disconnected => {
                                    self.close = false;
                                    log::info!("cm ipc connection disconnect");
                                    break;
                                }
                                Data::PrivacyModeState((_id, _)) => {
                                    #[cfg(windows)]
                                    cm_inner_send(_id, data);
                                }
                                Data::ClickTime(ms) => {
                                    CLICK_TIME.store(ms, Ordering::SeqCst);
                                }
                                Data::ChatMessage { text } => {
                                    self.cm.new_message(self.conn_id, text);
                                }
                                Data::FS(mut fs) => {
                                    if let ipc::FS::WriteBlock { id, file_num, data: _, compressed } = fs {
                                        if let Ok(bytes) = self.stream.next_raw().await {
                                            fs = ipc::FS::WriteBlock{id, file_num, data:bytes.into(), compressed};
                                            handle_fs(fs, &mut write_jobs, &self.tx, Some(&tx_log)).await;
                                        }
                                    } else {
                                        handle_fs(fs, &mut write_jobs, &self.tx, Some(&tx_log)).await;
                                    }
                                    let log = fs::serialize_transfer_jobs(&write_jobs);
                                    self.cm.ui_handler.file_transfer_log("transfer", &log);
                                }
                                Data::FileTransferLog((action, log)) => {
                                    self.cm.ui_handler.file_transfer_log(&action, &log);
                                }                                
                                #[cfg(not(any(target_os = "android", target_os = "ios")))]
                                Data::ClipboardFile(_clip) => {
                                    #[cfg(any(target_os = "windows", target_os="linux", target_os = "macos"))]
                                    {
                                        let is_stopping_allowed = _clip.is_stopping_allowed_from_peer();
                                        let is_clipboard_enabled = ContextSend::is_enabled();
                                        let file_transfer_enabled = self.file_transfer_enabled;
                                        let stop = !is_stopping_allowed && !(is_clipboard_enabled && file_transfer_enabled);
                                        log::debug!(
                                            "Process clipboard message from client peer, stop: {}, is_stopping_allowed: {}, is_clipboard_enabled: {}, file_transfer_enabled: {}",
                                            stop, is_stopping_allowed, is_clipboard_enabled, file_transfer_enabled);
                                        if stop {
                                            ContextSend::set_is_stopped();
                                        } else {
                                            if !is_authorized {
                                                log::debug!("Clipboard message from client peer, but not authorized");
                                                continue;
                                            }
                                            let conn_id = self.conn_id;
                                            let _ = ContextSend::proc(|context| -> ResultType<()> {
                                                context.server_clip_file(conn_id, _clip)
                                                    .map_err(|e| e.into())
                                            });
                                        }
                                    }
                                }
                                Data::ClipboardFileEnabled(_enabled) => {
                                    #[cfg(any(target_os= "windows",target_os ="linux", target_os = "macos"))]
                                    {
                                        self.file_transfer_enabled_peer = _enabled;
                                    }
                                }
                                /*
                                Data::Theme(dark) => {
                                    self.cm.change_theme(dark);
                                }
                                Data::Language(lang) => {
                                    LocalConfig::set_option("lang".to_owned(), lang);
                                    self.cm.change_language();
                                }
                                */
                                Data::DataPortableService(ipc::DataPortableService::CmShowElevation(show)) => {
                                    self.cm.show_elevation(show);
                                }
                                Data::StartVoiceCall => {
                                    self.cm.voice_call_started(self.conn_id);
                                }
                                Data::VoiceCallIncoming => {
                                    self.cm.voice_call_incoming(self.conn_id);
                                }
                                Data::CloseVoiceCall(reason) => {
                                    self.cm.voice_call_closed(self.conn_id, reason.as_str());
                                }
                                _ => {

                                }
                            }
                        }
                        _ => {}
                    }
                }
                Some(data) = self.rx.recv() => {
                    if self.stream.send(&data).await.is_err() {
                        break;
                    }
                    match &data {
                        Data::SwitchPermission{name: _name, enabled: _enabled} => {
                            #[cfg(any(target_os="linux", target_os="windows", target_os = "macos"))]
                            if _name == "file" {
                                self.file_transfer_enabled = *_enabled;
                            }
                        }
                        Data::Authorize => {
                            self.running = true;
                            break;
                        }
                        _ => {
                        }
                    }
                },
                clip_file = rx_clip.recv() => match clip_file {
                    Some(_clip) => {
                        #[cfg(any(target_os = "windows", target_os ="linux", target_os = "macos"))]
                        {
                            let is_stopping_allowed = _clip.is_stopping_allowed();
                            let is_clipboard_enabled = ContextSend::is_enabled();
                            let file_transfer_enabled = self.file_transfer_enabled;
                            let file_transfer_enabled_peer = self.file_transfer_enabled_peer;
                            let stop = is_stopping_allowed && !(is_clipboard_enabled && file_transfer_enabled && file_transfer_enabled_peer);
                            log::debug!(
                                "Process clipboard message from clip, stop: {}, is_stopping_allowed: {}, is_clipboard_enabled: {}, file_transfer_enabled: {}, file_transfer_enabled_peer: {}",
                                stop, is_stopping_allowed, is_clipboard_enabled, file_transfer_enabled, file_transfer_enabled_peer);
                            if stop {
                                ContextSend::set_is_stopped();
                            } else {
                                allow_err!(self.tx.send(Data::ClipboardFile(_clip)));
                            }
                        }
                    }
                    None => {
                        //
                    }
                },
                /*
                Some(job_log) = rx_log.recv() => {
                    self.cm.ui_handler.file_transfer_log("transfer", &job_log);
                }
                */
            }
        }
    }

    async fn ipc_task(stream: Connection, cm: ConnectionManager<T>) {
        log::debug!("ipc task begin");
        let (tx, rx) = mpsc::unbounded_channel::<Data>();
        let mut task_runner = Self {
            stream,
            cm,
            tx,
            rx,
            close: true,
            running: true,
            conn_id: 0,
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            file_transfer_enabled: false,
            #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
            file_transfer_enabled_peer: false,
        };

        while task_runner.running {
            task_runner.run().await;
        }
        if task_runner.conn_id > 0 {
            task_runner
                .cm
                .remove_connection(task_runner.conn_id, task_runner.close);
        }
        log::debug!("ipc task end");
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tokio::main(flavor = "current_thread")]
pub async fn start_ipc<T: InvokeUiCM>(cm: ConnectionManager<T>) {
    #[cfg(windows)]
    std::thread::spawn(move || {
        log::info!("try create privacy mode window");
        if let Err(e) = crate::platform::windows::check_update_broker_process() {
            log::warn!(
                "Failed to check update broker process. Privacy mode may not work properly. {}",
                e
            );
        }
    });

    #[cfg(any(
        target_os = "windows",
        all(
            any(target_os = "linux", target_os = "macos"),
            feature = "unix-file-copy-paste"
        ),
    ))]
    ContextSend::enable(Config::get_option("enable-file-transfer").is_empty());
    
    match ipc::new_listener("_cm").await {
        Ok(mut incoming) => {
            while let Some(result) = incoming.next().await {
                match result {
                    Ok(stream) => {
                        log::debug!("Got new connection");
                        tokio::spawn(IpcTaskRunner::<T>::ipc_task(
                            Connection::new(stream),
                            cm.clone(),
                        ));
                    }
                    Err(err) => {
                        log::error!("Couldn't get cm client: {:?}", err);
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Failed to start cm ipc server: {}", err);
        }
    }
    crate::platform::quit_gui();
}

#[cfg(target_os = "android")]
#[tokio::main(flavor = "current_thread")]
pub async fn start_listen<T: InvokeUiCM>(
    cm: ConnectionManager<T>,
    mut rx: mpsc::UnboundedReceiver<Data>,
    tx: mpsc::UnboundedSender<Data>,
) {
    let mut current_id = 0;
    let mut write_jobs: Vec<fs::TransferJob> = Vec::new();
    loop {
        match rx.recv().await {
            Some(Data::Login {
                id,
                is_file_transfer,
                port_forward,
                peer_id,
                name,
                authorized,
                keyboard,
                clipboard,
                audio,
                file,
                restart,
                recording,
                block_input,
                from_switch,                
                security_numbers,
                avatar_image,
                ..
            }) => {
                current_id = id;
                cm.add_connection(
                    id,
                    is_file_transfer,
                    port_forward,
                    peer_id,
                    name,
                    authorized,
                    keyboard,
                    clipboard,
                    audio,
                    file,
                    restart,
                    recording,
                    block_input,
                    from_switch,
                    tx.clone(),
                    security_numbers,
                    avatar_image,
                );
            }
            Some(Data::ChatMessage { text }) => {
                cm.new_message(current_id, text);
            }
            Some(Data::FS(fs)) => {
                handle_fs(fs, &mut write_jobs, &tx, None).await;
            }
            Some(Data::Close) => {
                break;
            }
            None => {
                break;
            }
            _ => {}
        }
    }
    cm.remove_connection(current_id, true);
}

#[cfg(not(any(target_os = "ios")))]
async fn handle_fs(
    fs: ipc::FS,
    write_jobs: &mut Vec<fs::TransferJob>,
    tx: &UnboundedSender<Data>,
    tx_log: Option<&UnboundedSender<String>>,
) {
    use hbb_common::fs::serialize_transfer_job;
    match fs {
        ipc::FS::ReadDir {
            dir,
            include_hidden,
        } => {
            read_dir(&dir, include_hidden, tx).await;
        }
        ipc::FS::RemoveDir {
            path,
            id,
            recursive,
        } => {
            remove_dir(path, id, recursive, tx).await;
        }
        ipc::FS::RemoveFile { path, id, file_num } => {
            remove_file(path, id, file_num, tx).await;
        }
        ipc::FS::CreateDir { path, id } => {
            create_dir(path, id, tx).await;
        }
        ipc::FS::NewWrite {
            path,
            id,
            file_num,
            mut files,
            overwrite_detection,
            total_size,
            conn_id,
        } => {
            // cm has no show_hidden context
            // dummy remote, show_hidden, is_remote
            let mut job = fs::TransferJob::new_write(
                id,
                "".to_string(),
                path,
                file_num,
                false,
                false,
                files
                    .drain(..)
                    .map(|f| FileEntry {
                        name: f.0,
                        modified_time: f.1,
                        ..Default::default()
                    })
                    .collect(),
                overwrite_detection,
            );
            job.total_size = total_size;
            job.conn_id = conn_id;
            //write_jobs.push(job);
        }
        ipc::FS::CancelWrite { id } => {
            if let Some(job) = fs::get_job(id, write_jobs) {
                job.remove_download_file();
                tx_log.map(|tx: &UnboundedSender<String>| {
                    tx.send(serialize_transfer_job(job, false, true, ""))
                });
                fs::remove_job(id, write_jobs);
            }
        }
        ipc::FS::WriteDone { id, file_num } => {
            if let Some(job) = fs::get_job(id, write_jobs) {
                job.modify_time();
                send_raw(fs::new_done(id, file_num), tx);
                tx_log.map(|tx| tx.send(serialize_transfer_job(job, true, false, "")));
                fs::remove_job(id, write_jobs);
            }
        }
        ipc::FS::WriteError { id, file_num, err } => {
            if let Some(job) = fs::get_job(id, write_jobs) {
                tx_log.map(|tx| tx.send(serialize_transfer_job(job, false, false, &err)));
                send_raw(fs::new_error(job.id(), err, file_num), tx);
                fs::remove_job(job.id(), write_jobs);
            }
        }
       ipc::FS::WriteBlock {
            id,
            file_num,
            data,
            compressed,
        } => {
            if let Some(job) = fs::get_job(id, write_jobs) {
                if let Err(err) = job
                    .write(FileTransferBlock {
                        id,
                        file_num,
                        data,
                        compressed,
                        ..Default::default()
                    })
                    .await
                {
                    send_raw(fs::new_error(id, err, file_num), &tx);
                }
            }
        }
        ipc::FS::CheckDigest {
            id,
            file_num,
            file_size,
            last_modified,
            is_upload,
        } => {
            if let Some(job) = fs::get_job(id, write_jobs) {
                let mut req = FileTransferSendConfirmRequest {
                    id,
                    file_num,
                    union: Some(file_transfer_send_confirm_request::Union::OffsetBlk(0)),
                    ..Default::default()
                };
                let digest = FileTransferDigest {
                    id,
                    file_num,
                    last_modified,
                    file_size,
                    ..Default::default()
                };
                if let Some(file) = job.files().get(file_num as usize) {
                    let path = get_string(&job.join(&file.name));
                    match is_write_need_confirmation(&path, &digest) {
                        Ok(digest_result) => {
                            match digest_result {
                                DigestCheckResult::IsSame => {
                                    req.set_skip(true);
                                    let msg_out = new_send_confirm(req);
                                    send_raw(msg_out, &tx);
                                }
                                DigestCheckResult::NeedConfirm(mut digest) => {
                                    // upload to server, but server has the same file, request
                                    digest.is_upload = is_upload;
                                    let mut msg_out = Message::new();
                                    let mut fr = FileResponse::new();
                                    fr.set_digest(digest);
                                    msg_out.set_file_response(fr);
                                    send_raw(msg_out, &tx);
                                }
                                DigestCheckResult::NoSuchFile => {
                                    let msg_out = new_send_confirm(req);
                                    send_raw(msg_out, &tx);
                                }
                            }
                        }
                        Err(err) => {
                            send_raw(fs::new_error(id, err, file_num), &tx);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

#[cfg(not(any(target_os = "ios")))]
async fn read_dir(dir: &str, include_hidden: bool, tx: &UnboundedSender<Data>) {
    let path = {
        if dir.is_empty() {
            Config::get_home()
        } else {
            fs::get_path(dir)
        }
    };
    if let Ok(Ok(fd)) = spawn_blocking(move || fs::read_dir(&path, include_hidden)).await {
        let mut msg_out = Message::new();
        let mut file_response = FileResponse::new();
        file_response.set_dir(fd);
        msg_out.set_file_response(file_response);
        send_raw(msg_out, tx);
    }
}

#[cfg(not(any(target_os = "ios")))]
async fn handle_result<F: std::fmt::Display, S: std::fmt::Display>(
    res: std::result::Result<std::result::Result<(), F>, S>,
    id: i32,
    file_num: i32,
    tx: &UnboundedSender<Data>,
) {
    match res {
        Err(err) => {
            send_raw(fs::new_error(id, err, file_num), tx);
        }
        Ok(Err(err)) => {
            send_raw(fs::new_error(id, err, file_num), tx);
        }
        Ok(Ok(())) => {
            send_raw(fs::new_done(id, file_num), tx);
        }
    }
}

#[cfg(not(any(target_os = "ios")))]
async fn remove_file(path: String, id: i32, file_num: i32, tx: &UnboundedSender<Data>) {
    handle_result(
        spawn_blocking(move || fs::remove_file(&path)).await,
        id,
        file_num,
        tx,
    )
    .await;
}

#[cfg(not(any(target_os = "ios")))]
async fn create_dir(path: String, id: i32, tx: &UnboundedSender<Data>) {
    handle_result(
        spawn_blocking(move || fs::create_dir(&path)).await,
        id,
        0,
        tx,
    )
    .await;
}

#[cfg(not(any(target_os = "ios")))]
async fn remove_dir(path: String, id: i32, recursive: bool, tx: &UnboundedSender<Data>) {
    let path = fs::get_path(&path);
    handle_result(
        spawn_blocking(move || {
            if recursive {
                fs::remove_all_empty_dir(&path)
            } else {
                std::fs::remove_dir(&path).map_err(|err| err.into())
            }
        })
        .await,
        id,
        0,
        tx,
    )
    .await;
}

#[cfg(not(any(target_os = "ios")))]
fn send_raw(msg: Message, tx: &UnboundedSender<Data>) {
    match msg.write_to_bytes() {
        Ok(bytes) => {
            allow_err!(tx.send(Data::RawMessage(bytes)));
        }
        err => allow_err!(err),
    }
}

#[cfg(windows)]
fn cm_inner_send(id: i32, data: Data) {
    let lock = CLIENTS.read().unwrap();
    if id != 0 {
        if let Some(s) = lock.get(&id) {
            allow_err!(s.tx.send(data));
        }
    } else {
        for s in lock.values() {
            allow_err!(s.tx.send(data.clone()));
        }
    }
}

pub fn can_elevate() -> bool {
    #[cfg(windows)]
    return !crate::platform::is_installed();
    #[cfg(not(windows))]
    return false;
}

pub fn elevate_portable(_id: i32) {
    #[cfg(windows)]
    {
        let lock = CLIENTS.read().unwrap();
        if let Some(s) = lock.get(&_id) {
            allow_err!(s.tx.send(ipc::Data::DataPortableService(
                ipc::DataPortableService::RequestStart
            )));
        }
    }
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
#[inline]
pub fn handle_incoming_voice_call(id: i32, accept: bool) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        // Not handled in iOS yet.
        #[cfg(not(any(target_os = "ios")))]
        allow_err!(client.tx.send(Data::VoiceCallResponse(accept)));
    };
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
#[inline]
pub fn close_voice_call(id: i32) {
    if let Some(client) = CLIENTS.read().unwrap().get(&id) {
        // Not handled in iOS yet.
        #[cfg(not(any(target_os = "ios")))]
        allow_err!(client.tx.send(Data::CloseVoiceCall("".to_owned())));
    };
}
