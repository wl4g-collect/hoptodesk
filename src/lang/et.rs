lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Sinu töölauale saab selle ID ja parooliga ligi pääseda."),
        ("connecting_status", "HopToDeski võrguga ühendumine..."),
        ("connecting_status_short", "Ühendamine..."),		
        ("not_ready_status", "Pole valmis. Palun kontrolli oma ühendust"),
        ("not_ready_status_short", "Pole valmis."),
        ("ID/Relay Server", "ID-/releeserver"),
        ("id_change_tip", "Lubatud on vaid a-z, A-Z, 0-9 ja _ (alakriips) tähemärgid. Esimene täht peab olema a-z või A-Z. Pikkus vahemikus 6-16."),
        ("Slogan_tip", ""),
        ("Build Date", "Ehituskuupäev"),
        ("Audio Input", "Helisisend"),
        ("Hardware Codec", "Riistvarakoodek"),
        ("ID Server", "ID-server"),
        ("Relay Server", "Releeserver"),
        ("API Server", "Rakendusliidese server"),
        ("invalid_http", "peab algama: http:// või https://"),
        ("server_not_support", "Pole veel serveri poolt toetatud"),
        ("Password Required", "Parool on nõutud"),
        ("Wrong Password", "Vale parool"),
        ("Connection Error", "Ühenduse viga"),
        ("Login Error", "Sisselogimise viga"),
        ("Show Hidden Files", "Kuva peidetud faile"),
        ("Refresh File", "Värskenda faili"),
        ("Remote Computer", "Kaugarvuti"),
        ("Local Computer", "Kohalik arvuti"),
        ("Confirm Delete", "Kinnita kustutamist"),
        ("Multi Select", "Mitmikvalik"),
        ("Select All", "Vali kõik"),
        ("Unselect All", "Tühista kõigi valik"),
        ("Empty Directory", "Tühi kaust"),
        ("Custom Image Quality", "Kohandatud pildikvaliteet"),
        ("Adjust Window", "Kohanda akent"),
        ("Insert Lock", "Sisesta lukk"),
        ("Set Password", "Määra parool"),
        ("OS Password", "Opsüsteemi parool"),
        ("install_tip", "Parima jõudluse saavutamiseks viige lõpule täielik installimine."),
        ("config_acc", "Töölaua kaugjuhtimiseks tuleb HopToDeskile anda \"juurdepääsetavuse\" õigused."),
        ("config_screen", "Töölaua kaugjuhtimiseks tuleb HopToDeskile anda \"ekraanisalvestuse\" õigused."),
        ("Installation Path", "Paigaldustee"),
        ("agreement_tip", "Paigalduse alustamisel nõustud litsentsilepinguga."),
        ("Accept and Install", "Nõustu ja paigalda"),
        ("not_close_tcp_tip", "Ara sulge seda akent, kuni kasutad tunnelit"),
        ("Remote Host", "Kaughost"),
        ("Remote Port", "Kaugport"),
        ("Local Port", "Kohalik port"),
        ("Local Address", "Kohalik aadress"),
        ("Change Local Port", "Muuda kohalikku porti"),
        ("setup_server_tip", "Kiirema ühenduse jaoks palun seadista oma server"),
        ("Enter Remote ID", "Sisesta kaug-ID"),
        ("Auto Login", "Logi automaatselt sisse (Kehtib vaid valiku \"lukusta pärast seansi lõppu\" lubamisel)"),
        ("Change Path", "Muuda failiteed"),
        ("Create Folder", "Loo kaust"),
        ("whitelist_tip", "Ainult lubamisloendis IP saab mulle ligi"),
        ("verification_tip", "Registreeritud e-posti aadressile on saadetud kinnituskood, sisselogimise jätkamiseks sisesta kinnituskood."),
        ("whitelist_sep", "Eraldatud koma, semikooloni, tühikute või uue reaga"),
        ("Add Tag", "Lisa silt"),
        ("Wrong credentials", "Vale kasutajanimi või parool"),
        ("Edit Tag", "Muuda silti"),
        ("Forget Password", "Unusta parool"),
        ("Add to Favorites", "Lisa lemmikutesse"),
        ("Remove from Favorites", "Eemalda lemmikutest"),
        ("SOCKS5 Proxy", "SOCKS5 proksi"),
        ("install_daemon_tip", "Süsteemikäivitusel käivitamiseks tuleb paigaldada süsteemiteenus."),
        ("Are you sure to close the connection?", "Kas soovid kindlasti ühenduse sulgeda?"),
        ("One-Finger Tap", "Ühe sõrme koputus"),
        ("Left Mouse", "Vasak hiireklahv"),
        ("One-Long Tap", "Üks pikk koputus"),
        ("Two-Finger Tap", "Kahe sõrme koputus"),
        ("Right Mouse", "Parem hiireklahv"),
        ("One-Finger Move", "Üks sõrmeliigutus"),
        ("Double Tap & Move", "Topeltkoputus ja liigutus"),
        ("Mouse Drag", "Hiirega sikutamine"),
        ("Three-Finger vertically", "Kolm sõrme vertikaalselt"),
        ("Mouse Wheel", "Hiirerullik"),
        ("Two-Finger Move", "Kahe sõrme liigutus"),
        ("Canvas Move", "Lõuendi liigutus"),
        ("Pinch to Zoom", "Näpistus-suum"),
        ("Canvas Zoom", "Lõuendi suum"),
        ("Share Screen", "Jaga ekraani"),
        ("Screen Capture", "Ekraanisalvestus"),
        ("Input Control", "Sisendjuhtimine"),
        ("Audio Capture", "Helisalvestus"),
        ("File Connection", "Failiühendus"),
        ("Screen Connection", "Kuvaühendus"),
        ("Open System Setting", "Ava süsteemisätted"),
        ("android_input_permission_tip1", "Selleks, et kaugseade saaks sinu Androidi seadet juhtida hiire või puute abil, pead andma HopToDeskile \"juurdepääsetavuse\" loa."),
        ("android_input_permission_tip2", "Palun mine järgmisele süsteemiseadete lehele, leia ja sisesta [Paigaldatud teenused], lülita teenus [HopToDesk Input] sisse."),
        ("android_new_connection_tip", "Saabunud on uus juhtimistaotlus, mis soovib sinu praegust seadet juhtida."),
        ("android_service_will_start_tip", "\"Ekraanisalvestuse\" lubamine käivitab teenuse automaatselt, lubades teistel seadetel sinu seadmesse ühendust taotleda."),
        ("android_stop_service_tip", "Teenuse sulgemine sulgeb automaatselt kõik loodud ühendused."),
        ("android_version_audio_tip", "Kasutatav Androidi versioon ei toeta helisalvestust, palun täienda Android 10 või uuemale versioonile."),
        ("android_start_service_tip", "Koputa [Alusta teenust] või anna [Ekraanisalvestuse] luba, et ekraanijagamisteenust alustada."),
        ("android_permission_may_not_change_tip", "Loodud ühenduste õigused ei pruugi muutuda enne taasühendamist koheselt."),
        ("Ignore Battery Optimizations", "Ignoreeri akuoptimeerimisi"),
        ("android_open_battery_optimizations_tip", "Kui soovid selle funktsiooni keelata, palun mine järgmisele HopToDeski rakenduse seadete lehele, leia ja sisesta [Aku], eemalda linnuke valikult [Piiramata]"),
        ("remote_restarting_tip", "Kaugseade taaskäivitub, palun sulge see sõnumikast ja ühendu mõne aja pärast uuesti püsiva parooliga."),
        ("Exit Fullscreen", "Lahku täisekraanist"),
        ("Mobile Actions", "Mobiilitegevused"),
        ("Select Monitor", "Vali kuvar"),
        ("Control Actions", "Juhtimistegevused"),
        ("Display Settings", "Kuvasätted"),
        ("Image Quality", "Pildikvaliteet"),
        ("Scroll Style", "Kerimisstiil"),
        ("Show Toolbar", "Kuva tööriistariba"),
        ("Hide Toolbar", "Peida tööriistariba"),
        ("Direct Connection", "Otseühendus"),
        ("Relay Connection", "Releeühendus"),
        ("Secure Connection", "Turvaline ühendus"),
        ("Insecure Connection", "Ebaturvaline ühendus"),
        ("Dark Theme", "Tume teema"),
        ("Light Theme", "Hele teema"),
        ("Follow System", "Järgi süsteemi"),
        ("Unlock Security Settings", "Lukusta lahti turvasätted"),
        ("Unlock Network Settings", "Lukusta lahti võrgusätted"),
        ("Direct IP Access", "Otsene IP-ligipääs"),
        ("Audio Input Device", "Heli sisendseade"),
        ("Use IP Whitelisting", "Kasuta IP-lubamisloendit"),
        ("Pin Toolbar", "Kinnita tööriistariba"),
        ("Unpin Toolbar", "Eemalda tööriistariba kinnitus"),
        ("elevated_foreground_window_tip", "Kaugtöölaua praegune aken nõuab töötamiseks kõrgemaid õigusi, mistõttu ei saa see ajutiselt hiirt ja klaviatuuri kasutada. Võid kaugkasutajal paluda minimeerida praegune aken või klõpsata ühenduse haldamise aknas kõrgendatud loa nuppu. Selle probleemi vältimiseks on soovitatav kaugseadmesse tarkvara paigaldada."),
        ("Keyboard Settings", "Klaviatuurisätted"),
        ("Full Access", "Täielik ligipääs"),
        ("Screen Share", "Ekraanijagamine"),
        ("JumpLink", "Kuva"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Palun vali jagatav ekraan (tegutse partneri poolel)."),
        ("One-time Password", "Ühekordne parool"),
        ("hide_cm_tip", "Luba varjamine ainult siis, kui parooliga seansse võetakse vastu ning kasutatakse püsivat parooli."),
        ("wayland_experiment_tip", "Waylandi tugi on katsetusjärgus, järelvalveta juurdepääsu vajadusel palun kasuta X11."),
        ("software_render_tip", "Kui kasutad Linuxis Nvidia graafikakaarti ja kaugaken sulgub kohe pärast ühendamist, võib aidata üleminek avatud lähtekoodiga Nouveau draiverile ja valida tarkvaraline renderdamise. Vajalik on tarkvara taaskäivitamine."),
        ("config_input", "Kaugtöölaua klaviatuuriga juhtimiseks pead andma HopToDeskile \"sisendi jälgimise\" õigused."),
        ("config_microphone", "Kaugelt rääkimiseks pead andma HopToDeskile \"heli salvestamise\" õigused."),
        ("request_elevation_tip", "Sa võid kõrgendatud õigusi taotleda ka siis, kui keegi on kaugpoolel."),
        ("Elevation Error", "Kõrgendatud õiguste viga"),
        ("still_click_uac_tip", "Kaugkasutaja peab siiski ise vajutama käitatud HopToDeski kasutajakonto kontrollis (UAC) OK-nuppu."),
        ("Request Elevation", "Taotle kõrgendatud õigusi"),
        ("wait_accept_uac_tip", "Palun oota, kuni kaugkasutaja nõustub UAC-dialoogiga (kasutajakonto kontroll)."),
        ("Switch Sides", "Vaheta pooli"),
        ("Default View Style", "Vaikimisi kuvastiil"),
        ("Default Scroll Style", "Vaikimisi kerimisstiil"),
        ("Default Image Quality", "Vaikimisi pildikvaliteet"),
        ("Default Codec", "Vaikimisi koodek"),
        ("Other Default Options", "Teised vaikevalikud"),
        ("relay_hint_tip", "Otseühendust ei pruugi olla võimalik luua; võid proovida ühendust relee kaudu. Lisaks, kui soovid esimesel katsel kasutada releed, võid lisada ID-le järelliite \"/r\" või valida viimaste seansside kaardil - kui see on olemas - valiku \"Ühenda alati relee kaudu\"."),
        ("install_cert_tip", "Paigalda HopToDesk sertifikaat"),
        ("confirm_install_cert_tip", "See on HopToDeski testimise sertifikaat, mida võib usaldada. Sertifikaati kasutatakse vajadusel HopToDeski draiverite usaldamiseks ja paigaldamiseks."),
        ("RDP Settings", "RDP seaded"),
        ("New Connection", "Uus ühendus"),
        ("Your Device", "Sinu seade"),
        ("empty_recent_tip", "Siin kuvatakse viimased seansid."),
        ("empty_favorite_tip", "Siin kuvatakse lemmikkaaslased."),
        ("empty_lan_tip", "Avastatud eakaaslased kuvatakse siin."),
        ("empty_address_book_tip", "Teie aadressiraamatus pole praegu ühtegi partnerit."),
        ("Empty Username", "Tühi kasutajanimi"),
        ("Empty Password", "Tühi parool"),
        ("identical_file_tip", "See fail on partneri omaga identne."),
        ("show_monitors_tip", "Kuva kuvarid tööriistaribal"),
        ("View Mode", "Kuvarežiim"),
        ("login_linux_tip", "X-töölaua seansi lubamiseks pead sisse logima Linuxi kaugkontosse."),
        ("verify_rustdesk_password_tip", "Kinnita HopToDeski parooli"),
        ("remember_account_tip", "Jäta see konto meelde"),
        ("os_account_desk_tip", "Seda kontot kasutatakse kaug-opsüsteemi sisselogimiseks ja töölaua seansi lubamiseks headless-režiimis."),
        ("OS Account", "Opsüsteemi konto"),
        ("another_user_login_title_tip", "Teine kasutaja on juba sisse logitud"),
        ("another_user_login_text_tip", "Ühenda lahti"),
        ("xorg_not_found_title_tip", "Xorg-i ei leitud"),
        ("xorg_not_found_text_tip", "Palun paigalda Xorg"),
        ("no_desktop_title_tip", "Töölaud pole saadaval"),
        ("no_desktop_text_tip", "Palun paigalda GNOME Desktop"),
        ("System Sound", "Süsteemiheli"),
        ("Copy Fingerprint", "Kopeeri sõrmejälg"),
        ("no fingerprints", "Sõrmejäljed puuduvad"),
        ("resolution_original_tip", "Originaalne eraldusvõime"),
        ("resolution_fit_local_tip", "Ühita kohaliku eraldusvõimega"),
        ("resolution_custom_tip", "Kohandatud eraldusvõime"),
        ("Accept and Elevate", "Luba kõrgemate õigustega"),
        ("accept_and_elevate_btn_tooltip", "Võta ühendus vastu ja anna kõrgemad UAC-õigused (kasutajakonto kontroll)."),
        ("clipboard_wait_response_timeout_tip", "Koopia vastuse ootamisel tekkis ajalõpp."),
        ("logout_tip", "Kas soovid kindlasti välja logida?"),
        ("exceed_max_devices", "Oled saavutanud hallatavate seadmete maksimaalse arvu."),
        ("Change Password", "Vaheta parooli"),
        ("Refresh Password", "Värskenda parool"),
        ("Grid View", "Ruudustikuvaade"),
        ("List View", "Loendivaade"),
        ("Toggle Tags", "Lülita silte"),
        ("pull_ab_failed_tip", "Aadressiraamatu värskendamine ebaõnnestus"),
        ("push_ab_failed_tip", "Aadressiraamatu sünkroonimine serveriga ebaõnnestus"),
        ("synced_peer_readded_tip", "Hiljutistel seanssidel olnud seadmed sünkroonitakse tagasi aadressiraamatusse."),
        ("Change Color", "Vaheta värvi"),
        ("Primary Color", "Põhivärv"),
        ("HSV Color", "HSV-värv"),
        ("Installation Successful!", "Paigaldus oli edukas!"),
        ("auto_disconnect_option_tip", "Sissetulevate seansside automaatne sulgemine kasutaja mitteaktiivsuse korral"),
        ("Connection failed due to inactivity", "Mitteaktiivsuse tõttu automaatselt lahti ühendatud"),
        ("pull_group_failed_tip", "Grupi värskendamine ebaõnnestus"),
        ("display_is_plugged_out_msg", "See kuvar on välja lülitatud, lülita esmasele kuvarile."),
        ("elevated_switch_display_msg", "Lülita ümber esmasele kuvarile, sest kõrgendatud kasutajarežiimis ei toetata mitut kuvarit."),
        ("selinux_tip", "SELinux on su seadmes lubatud, mis võib HopToDeskil takistada juhitud poolel toimimist."),
        ("id_input_tip", "Võid sisestada ID, otsese IP või domeeni koos pordiga (<domeen>:<port>).\nKui soovid juurdepääsu seadmele mõnes teises serveris, lisa palun serveri aadress (<id>@<serveri_aadress>?key=<võtme_väärtus>), näiteks,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nKui soovid juurdepääsu seadmele avalikus serveris, sisesta \"<id>@public\", avaliku serveri puhul ei ole võtit vaja."),
        ("privacy_mode_impl_mag_tip", "Režiim 1"),
        ("privacy_mode_impl_virtual_display_tip", "Režiim 2"),
        ("idd_not_support_under_win10_2004_tip", "Kaudse kuvari draiver ei ole toetatud. Vajalik on Windows 10, versioon 2004 või uuem."),
        ("switch_display_elevated_connections_tip", "Mitme ühenduse korral ei toetata kõrgendatud kasutajarežiimil üleminekut muule kui primaarsele kuvale. Kui soovid juhtida mitut ekraani, palun proovi uuesti pärast paigaldamist."),
        ("input_source_1_tip", "Sisendallikas 1"),
        ("input_source_2_tip", "Sisendallikas 2"),
        ("capture_display_elevated_connections_tip", "Mitme ekraani jäädvustamine ei ole kõrgendatud kasutajarežiimis toetatud. Kui soovid juhtida mitut ekraani, palun proovi uuesti pärast paigaldamist."),
        ("swap-left-right-mouse", "Vaheta vasak ja parem hiirenupp"),
		("Enable 2FA", "Luba 2FA"),
        ("Enable 2FA Auto Accept", "Lubage 2FA automaatne aktsepteerimine"),		
		("Enable Wake On LAN", "Luba Wake On LAN"),
		("2FA QR code", "2FA QR-kood"),		
        ("Scan this QR code with a camera on a secondary device such as a phone to set it up as your 2FA authenticator.", "Skannige see QR-kood teise seadme (nt telefoni) kaameraga, et seadistada see oma 2FA autentijaks."),
        ("You will need to confirm the 2FA on the secondary device with you when trying to connect to this desktop.", "Selle töölauaga ühenduse loomisel peate kinnitama teisese seadme 2FA."),		
		("Choose Network", "Valige Võrk"),
		("ID (Click to Copy)", "ID (klõpsake kopeerimiseks)"),		
		("Password (Click to Copy)", "Parool (kopeerimiseks klõpsake)"),
		("Unattended Access", "Järelevalveta juurdepääs"),		
    ].iter().cloned().collect();
}
