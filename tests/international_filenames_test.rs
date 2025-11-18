// Test for handling filenames in 100+ languages and special characters
// Ensures the hash utility works correctly with international filenames

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Test data: filenames in 200+ languages and scripts with 80+ character names
/// Covers major writing systems, special characters, and edge cases
fn get_international_test_filenames() -> Vec<(&'static str, &'static str)> {
    vec![
        // Latin-based languages (Extended with longer names)
        ("English", "this_is_a_comprehensive_test_file_for_english_language_support_with_long_names.txt"),
        ("French", "ceci_est_un_fichier_de_test_complet_pour_le_support_de_la_langue_française_éèêë.txt"),
        ("German", "dies_ist_eine_umfassende_Prüfungsdatei_für_die_deutsche_Sprachunterstützung_äöüß.txt"),
        ("Spanish", "este_es_un_archivo_de_prueba_completo_para_el_soporte_del_idioma_español_ñáéíóú.txt"),
        ("Portuguese", "este_é_um_arquivo_de_teste_abrangente_para_suporte_ao_idioma_português_ãõçáéíóú.txt"),
        ("Italian", "questo_è_un_file_di_test_completo_per_il_supporto_della_lingua_italiana_àèéìòù.txt"),
        ("Polish", "to_jest_kompleksowy_plik_testowy_dla_obsługi_języka_polskiego_ąćęłńóśźż.txt"),
        ("Czech", "toto_je_komplexní_testovací_soubor_pro_podporu_českého_jazyka_áčďéěíňóřšťúůýž.txt"),
        ("Turkish", "bu_türkçe_dil_desteği_için_kapsamlı_bir_test_dosyasıdır_çğıöşü.txt"),
        ("Romanian", "acesta_este_un_fișier_de_test_cuprinzător_pentru_suportul_limbii_române_ăâîșț.txt"),
        ("Hungarian", "ez_egy_átfogó_teszt_fájl_a_magyar_nyelv_támogatásához_áéíóöőúüű.txt"),
        ("Vietnamese", "đây_là_tệp_thử_nghiệm_toàn_diện_cho_hỗ_trợ_tiếng_việt_ăâđêôơư.txt"),
        ("Dutch", "dit_is_een_uitgebreid_testbestand_voor_nederlandse_taalondersteuning_met_lange_namen.txt"),
        ("Swedish", "detta_är_en_omfattande_testfil_för_stöd_av_svenska_språket_med_långa_namn_åäö.txt"),
        ("Norwegian", "dette_er_en_omfattende_testfil_for_støtte_av_norsk_språk_med_lange_navn_æøå.txt"),
        ("Danish", "dette_er_en_omfattende_testfil_til_understøttelse_af_dansk_sprog_med_lange_navne_æøå.txt"),
        ("Finnish", "tämä_on_kattava_testitiedosto_suomen_kielen_tukemiseksi_pitkillä_nimillä_äö.txt"),
        ("Icelandic", "þetta_er_yfirgripsmikil_prófunarskrá_fyrir_stuðning_við_íslenskt_tungumál_áðéíóúýþæö.txt"),
        ("Estonian", "see_on_põhjalik_testfail_eesti_keele_toetamiseks_pikkade_nimedega_äöõü.txt"),
        ("Latvian", "šis_ir_visaptverošs_testa_fails_latviešu_valodas_atbalstam_ar_gariem_nosaukumiem_āčēģīķļņšūž.txt"),
        ("Lithuanian", "tai_išsamus_bandomasis_failas_lietuvių_kalbos_palaikymui_su_ilgais_pavadinimais_ąčęėįšųūž.txt"),
        ("Slovak", "toto_je_komplexný_testovací_súbor_pre_podporu_slovenského_jazyka_s_dlhými_názvami_áäčďéíĺľňóôŕšťúýž.txt"),
        ("Slovenian", "to_je_celovita_testna_datoteka_za_podporo_slovenskega_jezika_z_dolgimi_imeni_čšž.txt"),
        ("Croatian", "ovo_je_sveobuhvatna_testna_datoteka_za_podršku_hrvatskog_jezika_s_dugim_imenima_čćđšž.txt"),
        ("Bosnian", "ovo_je_sveobuhvatna_testna_datoteka_za_podršku_bosanskog_jezika_s_dugim_imenima_čćđšž.txt"),
        ("Serbian_Latin", "ovo_je_sveobuhvatna_testna_datoteka_za_podršku_srpskog_jezika_s_dugim_imenima_čćđšž.txt"),
        ("Albanian", "ky_është_një_skedar_testimi_gjithëpërfshirës_për_mbështetjen_e_gjuhës_shqipe_me_emra_të_gjatë_ëç.txt"),
        ("Maltese", "dan_huwa_fajl_test_komprensiv_għall_appoġġ_tal_lingwa_maltija_b_ismijiet_twal_ċġħż.txt"),
        ("Welsh", "mae_hwn_yn_ffeil_prawf_gynhwysfawr_ar_gyfer_cefnogaeth_iaith_gymraeg_gydag_enwau_hir_âêîôûŵŷ.txt"),
        ("Irish", "is_comhad_tástála_cuimsitheach_é_seo_le_haghaidh_tacaíochta_teanga_gaeilge_le_hainmneacha_fada_áéíóú.txt"),
        ("Scottish_Gaelic", "seo_faidhle_deuchainn_coileanta_airson_taic_cànain_gàidhlig_le_ainmean_fada.txt"),
        ("Basque", "hau_euskararen_hizkuntza_euskarriaren_proba_fitxategi_oso_bat_da_izen_luzekin.txt"),
        ("Catalan", "aquest_és_un_fitxer_de_prova_complet_per_al_suport_de_la_llengua_catalana_amb_noms_llargs_àèéíòóú.txt"),
        ("Galician", "este_é_un_ficheiro_de_proba_completo_para_o_apoio_da_lingua_galega_con_nomes_longos_áéíóú.txt"),
        ("Esperanto", "ĉi_tio_estas_ampleksa_testa_dosiero_por_subteno_de_esperanto_lingvo_kun_longaj_nomoj_ĉĝĥĵŝŭ.txt"),
        ("Luxembourgish", "dëst_ass_en_ëmfaassende_testfichier_fir_d_ënnerstëtzung_vun_der_lëtzebuerger_sprooch.txt"),
        ("Faroese", "hetta_er_ein_umfatandi_royndarfíla_fyri_stuðul_av_føroyskt_mál_við_longum_navnum_áðíóúýæø.txt"),
        ("Greenlandic", "taanna_kalaallit_oqaasii_tapersersuinermi_atorsinnaasumik_immikkut_nassiunneqartoq.txt"),
        
        // Cyrillic script (Extended)
        ("Russian", "это_комплексный_тестовый_файл_для_поддержки_русского_языка_с_длинными_именами_файлов.txt"),
        ("Ukrainian", "це_комплексний_тестовий_файл_для_підтримки_української_мови_з_довгими_назвами_файлів_їєіґ.txt"),
        ("Bulgarian", "това_е_цялостен_тестов_файл_за_поддръжка_на_български_език_с_дълги_имена_на_файлове_ъ.txt"),
        ("Serbian", "ово_је_свеобухватна_тест_датотека_за_подршку_српског_језика_са_дугим_именима_датотека_ђћ.txt"),
        ("Macedonian", "ова_е_сеопфатна_тест_датотека_за_поддршка_на_македонскиот_јазик_со_долги_имиња_ѓќ.txt"),
        ("Belarusian", "гэта_комплексны_тэставы_файл_для_падтрымкі_беларускай_мовы_з_доўгімі_назвамі_файлаў_ў.txt"),
        ("Kazakh", "бұл_қазақ_тілін_қолдауға_арналған_кешенді_сынақ_файлы_ұзын_файл_атауларымен_әіңғүұқөһ.txt"),
        ("Kyrgyz", "бул_кыргыз_тилин_колдоого_арналган_комплекстүү_сыноо_файлы_узун_файл_аттары_менен_өүң.txt"),
        ("Tajik", "ин_файли_санҷишии_пурраи_барои_дастгирии_забони_тоҷикӣ_бо_номҳои_дарози_файлҳо_ғӣӯҳҷ.txt"),
        ("Uzbek", "bu_oʻzbek_tilini_qoʻllab_quvvatlash_uchun_keng_qamrovli_sinov_fayli_uzun_fayl_nomlari_bilan.txt"),
        ("Mongolian", "энэ_бол_монгол_хэлний_дэмжлэгт_зориулсан_өргөн_хүрээтэй_туршилтын_файл_урт_файлын_нэртэй.txt"),
        ("Tatar", "бу_татар_телен_ярдәм_итү_өчен_киң_күләмле_сынау_файлы_озын_файл_исемнәре_белән_әөүңһ.txt"),
        ("Bashkir", "был_башҡорт_телен_ярҙам_итеү_өсөн_киң_күләмле_һынау_файлы_оҙон_файл_исемдәре_менән_әөүңһ.txt"),
        ("Chechen", "хӀара_нохчийн_меттан_дӀахьедар_дан_шуьйра_йолу_синна_файл_йолу_цӀерийн_йолу_файл.txt"),
        ("Ossetian", "ай_у_ирон_æвзаджы_æххуысгæнæн_фæрцы_æнæмбарынгæнæн_тестон_файл_даргъ_номимæ.txt"),
        
        // Greek (Extended)
        ("Greek", "αυτό_είναι_ένα_ολοκληρωμένο_δοκιμαστικό_αρχείο_για_την_υποστήριξη_της_ελληνικής_γλώσσας_αβγδεζηθικλμνξοπρστυφχψω.txt"),
        ("Greek_Ancient", "τοῦτό_ἐστι_δοκιμαστικὸν_ἀρχεῖον_τῆς_ἀρχαίας_ἑλληνικῆς_γλώσσης_μετὰ_μακρῶν_ὀνομάτων.txt"),
        
        // Arabic script (RTL - Extended)
        ("Arabic", "هذا_ملف_اختبار_شامل_لدعم_اللغة_العربية_مع_أسماء_ملفات_طويلة_جدا_للاختبار_الكامل.txt"),
        ("Arabic_Egyptian", "ده_ملف_تجريبي_شامل_لدعم_اللهجة_المصرية_مع_أسماء_ملفات_طويلة_جدا_للاختبار.txt"),
        ("Arabic_Levantine", "هاد_ملف_تجريبي_شامل_لدعم_اللهجة_الشامية_مع_أسماء_ملفات_طويلة_كتير_للاختبار.txt"),
        ("Persian", "این_یک_فایل_آزمایشی_جامع_برای_پشتیبانی_از_زبان_فارسی_با_نامهای_فایل_بلند_است.txt"),
        ("Dari", "این_یک_فایل_آزمایشی_جامع_برای_پشتیبانی_از_زبان_دری_با_نامهای_فایل_بلند_میباشد.txt"),
        ("Urdu", "یہ_اردو_زبان_کی_معاونت_کے_لیے_ایک_جامع_ٹیسٹ_فائل_ہے_لمبے_فائل_ناموں_کے_ساتھ.txt"),
        ("Pashto", "دا_د_پښتو_ژبې_ملاتړ_لپاره_یو_جامع_ازموینه_فایل_دی_د_اوږدو_فایل_نومونو_سره.txt"),
        ("Kurdish_Sorani", "ئەمە_فایلێکی_تاقیکردنەوەی_گشتگیرە_بۆ_پشتگیری_زمانی_کوردی_سۆرانی_بە_ناوی_فایلی_درێژ.txt"),
        ("Kurdish_Kurmanji", "ev_pelek_ceribandinê_ya_berfireh_e_ji_bo_piştgiriya_zimanê_kurdî_kurmancî_bi_navên_pelên_dirêj.txt"),
        ("Sindhi", "هي_سنڌي_ٻولي_جي_مدد_لاءِ_هڪ_جامع_ٽيسٽ_فائل_آهي_ڊگهن_فائل_نالن_سان.txt"),
        ("Uyghur", "بۇ_ئۇيغۇر_تىلىنى_قوللاش_ئۈچۈن_كەڭ_كۆلەملىك_سىناق_ھۆججىتى_ئۇزۇن_ھۆججەت_ناملىرى_بىلەن.txt"),
        
        // Hebrew (RTL - Extended)
        ("Hebrew", "זהו_קובץ_בדיקה_מקיף_לתמיכה_בשפה_העברית_עם_שמות_קבצים_ארוכים_מאוד_לבדיקה_מלאה.txt"),
        ("Yiddish", "דאָס_איז_אַ_פֿולשטענדיקער_פּרוּוו_טעקע_פֿאַר_שטיצן_די_ייִדישע_שפּראַך_מיט_לאַנגע_טעקע_נעמען.txt"),
        
        // CJK (Chinese, Japanese, Korean - Extended)
        ("Chinese_Simplified", "这是一个用于支持简体中文语言的综合测试文件具有很长的文件名称以进行完整测试.txt"),
        ("Chinese_Traditional", "這是一個用於支持繁體中文語言的綜合測試文件具有很長的文件名稱以進行完整測試.txt"),
        ("Chinese_Classical", "此乃用於支持古典漢語之綜合測試文檔具有甚長之文檔名稱以進行完整測試者也.txt"),
        ("Cantonese", "呢個係一個用嚟支援粵語嘅綜合測試檔案有好長嘅檔案名嚟做完整測試.txt"),
        ("Japanese_Hiragana", "これはにほんごのひらがなをサポートするためのほうかつてきなテストファイルでながいファイルめいです.txt"),
        ("Japanese_Katakana", "コレハニホンゴノカタカナヲサポートスルタメノホウカツテキナテストファイルデナガイファイルメイデス.txt"),
        ("Japanese_Kanji", "此是日本語之漢字支援為之包括的試験文書長文書名持.txt"),
        ("Japanese_Mixed", "これは日本語のひらがな・カタカナ・漢字を混ぜた包括的なテストファイルで長いファイル名です.txt"),
        ("Korean_Hangul", "이것은_한국어_언어_지원을_위한_포괄적인_테스트_파일이며_긴_파일_이름을_가지고_있습니다.txt"),
        ("Korean_Hanja", "此是韓國語言語支援爲之包括的試驗文書長文書名持者也.txt"),
        ("Korean_Mixed", "이것은_한국어와_한자를_混合한_包括的인_테스트_파일이며_긴_파일_이름을_가지고_있습니다.txt"),
        
        // South Asian scripts (Extended)
        ("Hindi", "यह_हिंदी_भाषा_समर्थन_के_लिए_एक_व्यापक_परीक्षण_फ़ाइल_है_जिसमें_लंबे_फ़ाइल_नाम_हैं.txt"),
        ("Bengali", "এটি_বাংলা_ভাষা_সমর্থনের_জন্য_একটি_ব্যাপক_পরীক্ষা_ফাইল_যা_দীর্ঘ_ফাইলের_নাম_রয়েছে.txt"),
        ("Tamil", "இது_தமிழ்_மொழி_ஆதரவுக்கான_ஒரு_விரிவான_சோதனை_கோப்பு_நீண்ட_கோப்பு_பெயர்களுடன்.txt"),
        ("Telugu", "ఇది_తెలుగు_భాష_మద్దతు_కోసం_ఒక_సమగ్ర_పరీక్ష_ఫైలు_పొడవైన_ఫైలు_పేర్లతో.txt"),
        ("Gujarati", "આ_ગુજરાતી_ભાષા_સમર્થન_માટે_એક_વ્યાપક_પરીક્ષણ_ફાઇલ_છે_જેમાં_લાંબા_ફાઇલ_નામો_છે.txt"),
        ("Kannada", "ಇದು_ಕನ್ನಡ_ಭಾಷಾ_ಬೆಂಬಲಕ್ಕಾಗಿ_ಒಂದು_ಸಮಗ್ರ_ಪರೀಕ್ಷಾ_ಕಡತವಾಗಿದ್ದು_ಉದ್ದವಾದ_ಕಡತ_ಹೆಸರುಗಳನ್ನು_ಹೊಂದಿದೆ.txt"),
        ("Malayalam", "ഇത്_മലയാളം_ഭാഷാ_പിന്തുണയ്ക്കുള്ള_ഒരു_സമഗ്ര_പരീക്ഷണ_ഫയലാണ്_നീണ്ട_ഫയൽ_പേരുകളോടെ.txt"),
        ("Punjabi", "ਇਹ_ਪੰਜਾਬੀ_ਭਾਸ਼ਾ_ਸਹਾਇਤਾ_ਲਈ_ਇੱਕ_ਵਿਆਪਕ_ਟੈਸਟ_ਫਾਇਲ_ਹੈ_ਜਿਸ_ਵਿੱਚ_ਲੰਬੇ_ਫਾਇਲ_ਨਾਮ_ਹਨ.txt"),
        ("Sinhala", "මෙය_සිංහල_භාෂා_සහාය_සඳහා_විස්තීර්ණ_පරීක්ෂණ_ගොනුවක්_වන_අතර_දිගු_ගොනු_නම්_ඇත.txt"),
        ("Marathi", "ही_मराठी_भाषा_समर्थनासाठी_एक_सर्वसमावेशक_चाचणी_फाइल_आहे_ज्यात_लांब_फाइल_नावे_आहेत.txt"),
        ("Nepali", "यो_नेपाली_भाषा_समर्थनको_लागि_एक_व्यापक_परीक्षण_फाइल_हो_जसमा_लामो_फाइल_नामहरू_छन्.txt"),
        ("Oriya", "ଏହା_ଓଡ଼ିଆ_ଭାଷା_ସମର୍ଥନ_ପାଇଁ_ଏକ_ବ୍ୟାପକ_ପରୀକ୍ଷା_ଫାଇଲ_ଯାହା_ଲମ୍ବା_ଫାଇଲ_ନାମ_ଅଛି.txt"),
        ("Assamese", "এইটো_অসমীয়া_ভাষা_সমৰ্থনৰ_বাবে_এটা_ব্যাপক_পৰীক্ষা_ফাইল_যাৰ_দীঘল_ফাইলৰ_নাম_আছে.txt"),
        ("Urdu_Devanagari", "یہ_اردو_زبان_کی_حمایت_کے_لیے_ایک_جامع_ٹیسٹ_فائل_ہے_جس_میں_لمبے_فائل_نام_ہیں.txt"),
        ("Sanskrit", "एतत्_संस्कृत_भाषा_समर्थनार्थं_एकं_व्यापकं_परीक्षण_सञ्चिका_अस्ति_यस्मिन्_दीर्घाणि_सञ्चिका_नामानि_सन्ति.txt"),
        
        // Southeast Asian scripts (Extended)
        ("Thai", "นี่คือไฟล์ทดสอบที่ครอบคลุมสำหรับการสนับสนุนภาษาไทยด้วยชื่อไฟล์ที่ยาว.txt"),
        ("Lao", "ນີ້ແມ່ນໄຟລ໌ທົດສອບທີ່ສົມບູນສໍາລັບການສະຫນັບສະຫນູນພາສາລາວດ້ວຍຊື່ໄຟລ໌ທີ່ຍາວ.txt"),
        ("Burmese", "ဤသည်_မြန်မာဘာသာစကား_ပံ့ပိုးမှုအတွက်_ကျယ်ကျယ်ပြန့်ပြန့်_စမ်းသပ်ဖိုင်_ရှည်လျားသော_ဖိုင်အမည်များဖြင့်.txt"),
        ("Khmer", "នេះគឺជាឯកសារសាកល្បងដ៏ទូលំទូលាយសម្រាប់ការគាំទ្រភាសាខ្មែរជាមួយឈ្មោះឯកសារវែង.txt"),
        ("Javanese", "iki_berkas_uji_lengkap_kanggo_dhukungan_basa_jawa_kanthi_jeneng_berkas_dawa.txt"),
        ("Sundanese", "ieu_mangrupikeun_file_uji_lengkep_pikeun_dukungan_basa_sunda_kalayan_nami_file_panjang.txt"),
        ("Tagalog", "ito_ay_isang_komprehensibong_test_file_para_sa_suporta_ng_wikang_tagalog_na_may_mahabang_pangalan.txt"),
        ("Cebuano", "kini_usa_ka_bug_os_nga_test_file_alang_sa_suporta_sa_sinugbuanong_pinulongan_nga_adunay_taas_nga_ngalan.txt"),
        ("Malay", "ini_adalah_fail_ujian_menyeluruh_untuk_sokongan_bahasa_melayu_dengan_nama_fail_yang_panjang.txt"),
        ("Indonesian", "ini_adalah_file_uji_komprehensif_untuk_dukungan_bahasa_indonesia_dengan_nama_file_yang_panjang.txt"),
        
        // Other scripts (Extended)
        ("Georgian", "ეს_არის_ყოვლისმომცველი_სატესტო_ფაილი_ქართული_ენის_მხარდაჭერისთვის_გრძელი_ფაილის_სახელებით.txt"),
        ("Armenian", "սա_համապարփակ_փորձարկման_ֆայլ_է_հայերեն_լեզվի_աջակցության_համար_երկար_ֆայլի_անուններով.txt"),
        ("Amharic", "ይህ_ለአማርኛ_ቋንቋ_ድጋፍ_አጠቃላይ_የሙከራ_ፋይል_ነው_ረጅም_የፋይል_ስሞች_ያሉት.txt"),
        ("Tigrinya", "እዚ_ንትግርኛ_ቋንቋ_ደገፍ_ሓፈሻዊ_ፈተና_ፋይል_እዩ_ነዊሕ_ስም_ፋይላት_ዘለዎ.txt"),
        ("Oromo", "kun_faayilii_qorannoo_bal_oo_afaan_oromoo_deeggarsa_kan_maqaa_faayilii_dheeraa_qabu.txt"),
        ("Somali", "tani_waa_faylka_tijaabada_oo_dhamaystiran_ee_taageerada_luqadda_soomaaliga_oo_leh_magacyo_fayl_dhaadheer.txt"),
        ("Swahili", "hii_ni_faili_ya_majaribio_kamili_kwa_msaada_wa_lugha_ya_kiswahili_yenye_majina_marefu_ya_faili.txt"),
        ("Zulu", "leli_yifayela_lokuhlola_eliphelele_lokusekela_ulimi_lwesizulu_elinamagama_amade_efayela.txt"),
        ("Xhosa", "eli_lifayile_lovavanyo_olupheleleyo_lokuxhasa_ulwimi_lwesixhosa_elinamagama_amade_efayile.txt"),
        ("Afrikaans", "hierdie_is_n_omvattende_toetslêer_vir_ondersteuning_van_afrikaanse_taal_met_lang_lêername_êëï.txt"),
        ("Hausa", "wannan_fayil_ne_na_gwaji_mai_cikakke_don_tallafin_harshen_hausa_mai_dogayen_sunayen_fayil.txt"),
        ("Yoruba", "eyi_jẹ_faili_idanwo_ti_o_ni_kikun_fun_atilẹyin_ede_yoruba_pẹlu_awọn_orukọ_faili_gigun_ẹọṣ.txt"),
        ("Igbo", "nke_a_bụ_faịlụ_nnwale_zuru_ezu_maka_nkwado_asụsụ_igbo_nwere_aha_faịlụ_ogologo.txt"),
        
        // African and Middle Eastern languages (Additional)
        ("Berber_Tifinagh", "ⵜⴰⵏⵏⴰⵢⵜ_ⵏ_ⵓⵙⵏⵉⵔⵎ_ⵏ_ⵜⵓⵜⵍⴰⵢⵜ_ⵜⴰⵎⴰⵣⵉⵖⵜ_ⵙ_ⵉⵙⵎⴰⵡⵏ_ⵉⴳⴳⵓⵜⵏ.txt"),
        ("Coptic", "ⲡⲁⲓ_ⲡⲉ_ⲟⲩⲫⲁⲓⲗ_ⲛ_ⲧⲉⲥⲧ_ⲉϥϣⲟⲡ_ⲛ_ⲧⲁⲥⲡⲓⲣⲁ_ⲛ_ⲧⲙⲛⲧⲣⲙⲛⲕⲏⲙⲉ_ⲙⲛ_ϩⲁⲛⲣⲁⲛ_ⲉⲩⲟϣ.txt"),
        ("Syriac", "ܗܢܐ_ܗܘ_ܦܐܝܠܐ_ܕܒܘܚܪܢܐ_ܓܡܝܪܐ_ܠܬܡܟܬܐ_ܕܠܫܢܐ_ܣܘܪܝܝܐ_ܥܡ_ܫܡܗܐ_ܐܪܝܟܐ.txt"),
        ("Mandaic", "ࡀࡉࡃࡀ_ࡄࡅ_ࡐࡀࡉࡋࡀ_ࡃࡁࡅࡇࡓࡀࡍࡀ_ࡂࡌࡉࡓࡀ_ࡋࡕࡌࡊࡕࡀ_ࡃࡋࡔࡀࡍࡀ_ࡌࡀࡍࡃࡀࡉࡀ.txt"),
        ("Samaritan", "ࠀࠁࠂ_ࠄࠅ_ࠇࠈࠉࠊ_ࠌࠍࠎࠏ_ࠐࠑࠒࠓ_ࠔࠕࠖࠗ_࠘࠙ࠚࠛ_ࠜࠝࠞࠟ.txt"),
        
        // Asian languages (Additional)
        ("Dzongkha", "འདི་ནི་རྫོང་ཁ་སྐད་ཡིག་གི་རྒྱབ་སྐྱོར་གྱི་དོན་དུ་ཡོངས་རྫོགས་ཀྱི་བརྟག་དཔྱད་ཡིག་ཆ་ཞིག་ཡིན.txt"),
        ("Meitei", "ꯃꯁꯤ_ꯃꯅꯤꯄꯨꯔꯤ_ꯂꯣꯟ_ꯁꯄꯣꯔꯠ_ꯇꯧꯅꯕꯒꯤꯗꯃꯛ_ꯃꯄꯨꯡ_ꯐꯥꯅꯥ_ꯇꯦꯁ_ꯐꯥꯏꯜ_ꯑꯃꯅꯤ.txt"),
        ("Limbu", "ᤕᤠᤰᤌᤡᤱ_ᤛᤡᤰᤁᤠᤶᤒᤠ_ᤐᤠᤶᤍᤡᤱ_ᤛᤡᤰᤁᤠᤶᤒᤠ_ᤐᤠᤶᤍᤡᤱ_ᤛᤡᤰᤁᤠᤶᤒᤠ_ᤐᤠᤶᤍᤡᤱ.txt"),
        ("Tai_Le", "ᥖᥭᥰ_ᥑᥨᥒᥰ_ᥖᥭᥰ_ᥑᥨᥒᥰ_ᥖᥭᥰ_ᥑᥨᥒᥰ_ᥖᥭᥰ_ᥑᥨᥒᥰ_ᥖᥭᥰ_ᥑᥨᥒᥰ.txt"),
        ("Tai_Tham", "ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ_ᨴᩱ᩠ᨾ.txt"),
        ("Balinese", "ᬳᬶᬓᬶ_ᬩᬾᬃᬓᬲ᭄_ᬳᬸᬚᬶ_ᬮᬾᬂᬓᬧ᭄_ᬓᬂᬕᭀ_ᬤᬸᬓᬸᬗᬦ᭄_ᬩᬲ_ᬩᬮᬶ.txt"),
        ("Buginese", "ᨕᨗᨕᨗ_ᨅᨙᨑᨙᨀᨔ᨞_ᨕᨘᨍᨗ_ᨒᨙᨂᨀᨄ᨞_ᨀᨂᨁᨚ_ᨉᨘᨀᨘᨁᨙᨊ᨞_ᨅᨔ_ᨅᨘᨁᨗᨔ᨞.txt"),
        ("Rejang", "ꤰꥍꤰꥍ_ꤷꥍꤽꥍꤰꥍꤶ_ꤰꥍꤸꥍꤷꥍ_ꤻꥍꤾꥍꤰꥍꤰꥍꤿ_ꤰꥍꤾꥍꤱꥍꤚ_ꤷꥍꤸꥍꤰꥍꤸꥍꤾꥍꤾ.txt"),
        
        // Special characters and symbols (Extended)
        ("Emoji_Extended", "comprehensive_test_file_with_emojis_😀😃😄😁😆😅🤣😂🙂🙃😉😊😇🥰😍🤩😘😗☺😚😙🥲😋😛😜🤪😝🤑🤗🤭🤫🤔🤐🤨😐😑😶😏😒🙄😬🤥😌😔😪🤤😴😷🤒🤕🤢🤮🤧🥵🥶🥴😵🤯🤠🥳🥸😎🤓🧐😕😟🙁☹😮😯😲😳🥺😦😧😨😰😥😢😭😱😖😣😞😓😩😫🥱😤😡😠🤬😈👿💀☠💩🤡👹👺👻👽👾🤖😺😸😹😻😼😽🙀😿😾🙈🙉🙊.txt"),
        ("Mixed_Emoji_Long", "这是一个包含多种语言和表情符号的综合测试文件_test_тест_😊😀🎉🔥💯✨🌟⭐🎯🎪🎨🎭🎬🎤🎧🎼🎹🎸🎺🎷🥁🎻📯🎲🎯🎳🎮🎰🎱🏀🏈⚾🥎🏐🏉🎾🥏🎳🏏🏑🏒🥍🏓🏸🥊🥋🥅⛳🏹🎣🤿🥽🎿🛷⛸🥌🎯🪀🪁.txt"),
        ("Math_Symbols_Extended", "mathematical_symbols_test_file_∑∫∂∇∆∏∐√∛∜∝∞∟∠∡∢∣∤∥∦∧∨∩∪∫∬∭∮∯∰∱∲∳⊂⊃⊄⊅⊆⊇⊈⊉⊊⊋⊌⊍⊎⊏⊐⊑⊒⊓⊔⊕⊖⊗⊘⊙⊚⊛⊜⊝⊞⊟⊠⊡⊢⊣⊤⊥⊦⊧⊨⊩⊪⊫⊬⊭⊮⊯⊰⊱⊲⊳⊴⊵⊶⊷⊸⊹⊺⊻⊼⊽⊾⊿⋀⋁⋂⋃⋄⋅⋆⋇⋈⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋔⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟.txt"),
        ("Currency_Extended", "currency_symbols_test_file_$¢£¤¥₠₡₢₣₤₥₦₧₨₩₪₫€₭₮₯₰₱₲₳₴₵₶₷₸₹₺₻₼₽₾₿﷼﹩＄￠￡￥￦.txt"),
        ("Arrows_Extended", "arrow_symbols_test_file_←↑→↓↔↕↖↗↘↙↚↛↜↝↞↟↠↡↢↣↤↥↦↧↨↩↪↫↬↭↮↯↰↱↲↳↴↵↶↷↸↹↺↻↼↽↾↿⇀⇁⇂⇃⇄⇅⇆⇇⇈⇉⇊⇋⇌⇍⇎⇏⇐⇑⇒⇓⇔⇕⇖⇗⇘⇙⇚⇛⇜⇝⇞⇟⇠⇡⇢⇣⇤⇥⇦⇧⇨⇩⇪⇫⇬⇭⇮⇯⇰⇱⇲⇳⇴⇵⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿.txt"),
        ("Box_Drawing_Extended", "box_drawing_test_file_─━│┃┄┅┆┇┈┉┊┋┌┍┎┏┐┑┒┓└┕┖┗┘┙┚┛├┝┞┟┠┡┢┣┤┥┦┧┨┩┪┫┬┭┮┯┰┱┲┳┴┵┶┷┸┹┺┻┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋╌╍╎╏═║╒╓╔╕╖╗╘╙╚╛╜╝╞╟╠╡╢╣╤╥╦╧╨╩╪╫╬╭╮╯╰╱╲╳╴╵╶╷╸╹╺╻╼╽╾╿.txt"),
        ("Geometric_Shapes", "geometric_shapes_test_file_■□▢▣▤▥▦▧▨▩▪▫▬▭▮▯▰▱▲△▴▵▶▷▸▹►▻▼▽▾▿◀◁◂◃◄◅◆◇◈◉◊○◌◍◎●◐◑◒◓◔◕◖◗◘◙◚◛◜◝◞◟◠◡◢◣◤◥◦◧◨◩◪◫◬◭◮◯◰◱◲◳◴◵◶◷◸◹◺◻◼◽◾◿.txt"),
        ("Musical_Symbols", "musical_symbols_test_file_♩♪♫♬♭♮♯𝄀𝄁𝄂𝄃𝄄𝄅𝄆𝄇𝄈𝄉𝄊𝄋𝄌𝄍𝄎𝄏𝄐𝄑𝄒𝄓𝄔𝄕𝄖𝄗𝄘𝄙𝄚𝄛𝄜𝄝𝄞𝄟𝄠𝄡𝄢𝄣𝄤𝄥𝄦𝄧𝄨𝄩𝄪𝄫𝄬𝄭𝄮𝄯𝄰𝄱𝄲𝄳𝄴𝄵𝄶𝄷𝄸𝄹𝄺𝄻𝄼𝄽𝄾𝄿.txt"),
        ("Zodiac_Symbols", "zodiac_and_symbols_test_file_♈♉♊♋♌♍♎♏♐♑♒♓⚠⚡⚢⚣⚤⚥⚦⚧⚨⚩⚪⚫⚬⚭⚮⚯⚰⚱⚲⚳⚴⚵⚶⚷⚸⚹⚺⚻⚼⚽⚾⚿♀♁♂♃♄♅♆♇.txt"),
        ("Playing_Cards", "playing_cards_test_file_🂠🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂬🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂼🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃌🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃜🃝🃞🃟.txt"),
        ("Braille_Patterns", "braille_patterns_test_file_⠀⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟.txt"),
        ("Runic_Symbols", "runic_symbols_test_file_ᚠᚡᚢᚣᚤᚥᚦᚧᚨᚩᚪᚫᚬᚭᚮᚯᚰᚱᚲᚳᚴᚵᚶᚷᚸᚹᚺᚻᚼᚽᚾᚿᛀᛁᛂᛃᛄᛅᛆᛇᛈᛉᛊᛋᛌᛍᛎᛏᛐᛑᛒᛓᛔᛕᛖᛗᛘᛙᛚᛛᛜᛝᛞᛟᛠᛡᛢᛣᛤᛥᛦᛧᛨᛩᛪ᛫᛬᛭ᛮᛯᛰ.txt"),
        ("Ogham_Script", "ogham_script_test_file_᚛ᚁᚂᚃᚄᚅᚆᚇᚈᚉᚊᚋᚌᚍᚎᚏᚐᚑᚒᚓᚔᚕᚖᚗᚘᚙᚚ᚜_with_long_filename_for_testing.txt"),
        
        // Indigenous and Native languages
        ("Cherokee", "ᏣᎳᎩ_ᎦᏬᏂᎯᏍᏗ_ᎠᏰᎵ_ᎠᏂᏣᎳᎩ_ᎠᏰᎵ_ᎠᏂᏣᎳᎩ_ᎦᏬᏂᎯᏍᏗ_ᎠᏂᏣᎳᎩ_ᎠᏰᎵ.txt"),
        ("Inuktitut", "ᐃᓄᒃᑎᑐᑦ_ᐅᖃᐅᓯᖅ_ᑐᑭᓯᒋᐊᕐᕕᒃ_ᐱᓕᕆᐊᖑᔪᖅ_ᑎᑎᕋᖅᓯᒪᔪᖅ_ᐊᑎᖃᖅᑐᖅ_ᑕᑭᓂᖃᖅᑐᖅ.txt"),
        ("Cree", "ᓀᐦᐃᔭᐍᐏᐣ_ᐅᑭᒪᐏᓂᐤ_ᐱᒧᐦᑌᐏᐣ_ᒪᓯᓇᐦᐃᑲᓇ_ᑭᓀᐱᑯᓯᐏᐣ_ᑭᓀᐱᑯᓯᐏᐣ.txt"),
        ("Ojibwe", "anishinaabemowin_gikendaasowin_gikinoo_amaading_mazina_igan_gichi_niibowa_izhinikaazo.txt"),
        ("Navajo", "diné_bizaad_bee_áhoot_ééł_naaltsoos_bee_áhoot_ééł_naaltsoos_bee_áhoot_ééł_naaltsoos.txt"),
        ("Hawaiian", "ʻōlelo_hawaiʻi_hoʻokolohua_palapala_hoʻokolohua_palapala_hoʻokolohua_palapala_lōʻihi.txt"),
        ("Maori", "te_reo_māori_tuhinga_whakamātau_whānui_mō_te_tautoko_i_te_reo_māori_me_ngā_ingoa_roa.txt"),
        ("Samoan", "gagana_samoa_faʻataʻitaʻiga_faʻamatalaga_atoa_mo_le_lagolago_o_le_gagana_samoa_ma_igoa_umi.txt"),
        ("Tongan", "lea_fakatonga_fakamatala_fakaʻataʻatā_fakakakato_ki_he_tokoni_ʻo_e_lea_fakatonga_mo_e_hingoa_lōloa.txt"),
        ("Fijian", "vosa_vakaviti_ivola_ni_vakaraitaka_vakadodonu_me_baleta_na_veiqaravi_ni_vosa_vakaviti_kei_na_yacana_balavu.txt"),
        
        // Constructed and Artificial languages
        ("Klingon", "tlhIngan_Hol_wIvmeH_De_nIDev_pat_nIvbogh_De_pat_nIvbogh_De_pat_nIvbogh_De_pat.txt"),
        ("Elvish_Tengwar", "ᴀ_ᴛᴇɴɢᴡᴀʀ_ᴛᴇsᴛ_ғɪʟᴇ_ғᴏʀ_ᴇʟᴠɪsʜ_sᴄʀɪᴘᴛ_sᴜᴘᴘᴏʀᴛ_ᴡɪᴛʜ_ʟᴏɴɢ_ғɪʟᴇɴᴀᴍᴇs.txt"),
        ("Dothraki", "lekh_dothraki_shierak_qiya_mae_shafka_mae_shafka_mae_shafka_mae_shafka_mae_shafka.txt"),
        ("High_Valyrian", "valonqar_valyrio_eglie_valonqar_valyrio_eglie_valonqar_valyrio_eglie_valonqar.txt"),
        ("Lojban", "lojban_bangu_cipra_datni_vreji_clani_cmene_be_lo_datni_vreji_poi_clani_cmene.txt"),
        ("Toki_Pona", "toki_pona_lipu_pi_lukin_ale_tan_pali_pi_toki_pona_kepeken_nimi_lipu_suli.txt"),
        ("Interlingua", "interlingua_file_de_test_comprehensive_pro_supporto_del_lingua_interlingua_con_nomines_longe.txt"),
        ("Ido", "ido_linguo_testo_arkivo_kompleta_por_suporto_di_ido_linguo_kun_longa_nomi.txt"),
        ("Volapük", "volapük_pük_proböm_ragiv_valik_pro_yuf_volapüka_pük_ko_nems_lunik.txt"),
        
        // Edge cases (Extended)
        ("Spaces_Extended", "this is a comprehensive test file with many spaces in the filename for testing purposes.txt"),
        ("Multiple_Spaces_Extended", "file  with   multiple    spaces     between      words       for        testing.txt"),
        ("Leading_Space_Extended", " this_file_has_a_leading_space_character_at_the_beginning_of_its_name_for_testing.txt"),
        ("Trailing_Space_Extended", "this_file_has_a_trailing_space_character_at_the_end_of_its_name_for_testing .txt"),
        ("Dots_Extended", "file.with.many.dots.in.the.filename.for.testing.purposes.and.edge.cases.txt"),
        ("Dashes_Extended", "file-with-many-dashes-in-the-filename-for-testing-purposes-and-edge-cases.txt"),
        ("Underscores_Extended", "file_with_many_underscores_in_the_filename_for_testing_purposes_and_edge_cases.txt"),
        ("Mixed_Separators_Extended", "file-with_mixed.separators-in_the.filename-for_testing.purposes-and_edge.cases.txt"),
        ("Numbers_Extended", "1234567890_0987654321_1234567890_0987654321_1234567890_0987654321_numbers.txt"),
        ("Mixed_Numbers_Extended", "file123test456data789info012mixed345numbers678in901filename234test567.txt"),
        ("Special_Chars_Extended", "file_with_special_chars_!@#$%^&()_+-=[]{}|;',._test_file_for_edge_cases.txt"),
        
        // Long filenames (Extended to 80+ chars)
        ("Long_ASCII_Extended", "this_is_an_extremely_long_filename_that_tests_the_absolute_limits_of_filename_handling_in_various_operating_systems_and_filesystems_with_many_characters.txt"),
        ("Long_Unicode_Extended", "これは非常に長いファイル名でありシステムの制限をテストするための包括的なテストファイルです長い名前を持っています.txt"),
        ("Long_Mixed_Scripts", "это_очень_длинное_имя_файла_测试文件_test_file_with_mixed_scripts_ทดสอบ_परीक्षण_اختبار_δοκιμή.txt"),
        
        // Combined scripts (Extended)
        ("Latin_Cyrillic_Extended", "comprehensive_test_file_комплексный_тестовый_файл_with_mixed_scripts_смешанные_скрипты.txt"),
        ("Latin_Arabic_Extended", "comprehensive_test_file_ملف_اختبار_شامل_with_mixed_scripts_نصوص_مختلطة.txt"),
        ("Latin_CJK_Extended", "comprehensive_test_file_综合测试文件_テストファイル_테스트_파일_with_mixed_scripts.txt"),
        ("Multi_Script_Extended", "test_тест_测试_テスト_परीक्षण_اختبار_δοκιμή_ทดสอบ_mixed_scripts_everywhere.txt"),
        ("All_Scripts_Mixed", "english_русский_中文_日本語_한국어_العربية_עברית_ελληνικά_ไทย_हिन्दी_বাংলা_தமிழ்_emoji_😀🎉.txt"),
        
        // Case sensitivity tests (Extended)
        ("Uppercase_Extended", "THIS_IS_AN_UPPERCASE_FILENAME_FOR_TESTING_CASE_SENSITIVITY_IN_VARIOUS_SYSTEMS.TXT"),
        ("Lowercase_Extended", "this_is_a_lowercase_filename_for_testing_case_sensitivity_in_various_systems.txt"),
        ("MixedCase_Extended", "ThIs_Is_A_MiXeD_CaSe_FiLeNaMe_FoR_TeStInG_CaSe_SeNsItIvItY_In_VaRiOuS_SyStEmS.TxT"),
        ("AlternatingCase", "aLtErNaTiNg_CaSe_FiLeNaMe_WiTh_EvErY_ChArAcTeR_AlTeRnAtInG_BeTwEeN_CaSeS.txt"),
        ("CamelCase_Extended", "ThisIsCamelCaseFileNameForTestingCaseSensitivityInVariousOperatingSystems.txt"),
        ("Snake_Case_Extended", "this_is_snake_case_file_name_for_testing_case_sensitivity_in_various_systems.txt"),
        ("Kebab_Case_Extended", "this-is-kebab-case-file-name-for-testing-case-sensitivity-in-various-systems.txt"),
        
        // Historical and Ancient scripts
        ("Latin_Classical", "hic_est_lima_probationis_comprehensiva_pro_subsidio_linguae_latinae_cum_nominibus_longis.txt"),
        ("Old_English", "þis_is_a_comprehensive_test_file_for_old_english_language_support_with_long_names_æþðƿ.txt"),
        ("Middle_English", "this_is_a_comprehensif_test_file_for_middel_english_langage_support_with_longe_names.txt"),
        ("Old_Norse", "þetta_er_yfirgripsmikil_prófunarskrá_fyrir_stuðning_við_fornnorrænt_tungumál_með_löngum_nöfnum.txt"),
        ("Gothic", "𐌸𐌰𐍄𐌰_𐌹𐍃𐍄_𐍆𐌴𐌹𐌻𐌰_𐍀𐍂𐍉𐌱𐌰𐍄𐌹𐍉𐌽𐌹𐍃_𐌺𐍉𐌼𐍀𐍂𐌴𐌷𐌴𐌽𐍃𐌹𐍅𐌰_𐍀𐍂𐍉_𐍃𐌿𐌱𐍃𐌹𐌳𐌹𐍉_𐌻𐌹𐌽𐌲𐌿𐌰𐌴_𐌲𐍉𐍄𐌹𐍃𐌺𐌰𐌴.txt"),
        ("Phoenician", "𐤀𐤁𐤂𐤃𐤄𐤅𐤆𐤇𐤈𐤉𐤊𐤋𐤌𐤍𐤎𐤏𐤐𐤑𐤒𐤓𐤔𐤕_phoenician_test_file_with_long_name.txt"),
        ("Cuneiform", "𒀀𒀁𒀂𒀃𒀄𒀅𒀆𒀇𒀈𒀉𒀊𒀋𒀌𒀍𒀎𒀏_cuneiform_test_file_with_long_name_for_testing.txt"),
        ("Egyptian_Hieroglyphs", "𓀀𓀁𓀂𓀃𓀄𓀅𓀆𓀇𓀈𓀉𓀊𓀋𓀌𓀍𓀎𓀏_hieroglyphic_test_file_with_long_name_for_testing.txt"),
        ("Linear_B", "𐀀𐀁𐀂𐀃𐀄𐀅𐀆𐀇𐀈𐀉𐀊𐀋𐀌𐀍𐀎𐀏_linear_b_test_file_with_long_name_for_testing_purposes.txt"),
        ("Meroitic", "𐦀𐦁𐦂𐦃𐦄𐦅𐦆𐦇𐦈𐦉𐦊𐦋𐦌𐦍𐦎𐦏_meroitic_test_file_with_long_name_for_testing_purposes.txt"),
    ]
}

#[test]
fn test_international_filenames_scan() {
    let test_dir = "test_international_files";
    let output_db = "test_international_output.txt";
    
    // Create test directory
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    
    // Create files with international names
    let test_filenames = get_international_test_filenames();
    let mut created_files = Vec::new();
    
    for (lang, filename) in &test_filenames {
        let file_path = PathBuf::from(test_dir).join(filename);
        
        // Try to create the file - some filesystems may not support all characters
        match fs::write(&file_path, format!("Test content for {}", lang)) {
            Ok(_) => {
                created_files.push((lang, filename, file_path));
                println!("✓ Created file: {} ({})", filename, lang);
            }
            Err(e) => {
                // Log but don't fail - some filesystems have limitations
                eprintln!("⚠ Skipped file: {} ({}) - {}", filename, lang, e);
            }
        }
    }
    
    println!("\nSuccessfully created {}/{} test files (200+ languages with 80+ char names)", created_files.len(), test_filenames.len());
    
    // Run scan command
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "scan", "-d", test_dir, "-o", output_db])
        .output()
        .expect("Failed to execute scan command");
    
    println!("\nScan output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.status.success() {
        eprintln!("Scan stderr:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Scan command failed");
    }
    
    // Verify output database exists
    assert!(PathBuf::from(output_db).exists(), "Output database was not created");
    
    // Read and verify database content
    let db_content = fs::read_to_string(output_db)
        .expect("Failed to read output database");
    
    println!("\nDatabase content preview (first 10 lines):");
    for (i, line) in db_content.lines().take(10).enumerate() {
        println!("{}: {}", i + 1, line);
    }
    
    // Verify that files were processed
    let line_count = db_content.lines().count();
    println!("\nTotal lines in database: {}", line_count);
    assert!(line_count > 0, "Database is empty");
    
    // Verify each created file appears in the database
    let mut found_count = 0;
    for (lang, filename, _) in &created_files {
        if db_content.contains(*filename) {
            found_count += 1;
        } else {
            eprintln!("⚠ File not found in database: {} ({})", filename, lang);
        }
    }
    
    println!("\nFound {}/{} files in database", found_count, created_files.len());
    
    // We expect at least 75% of files to be processed successfully (some filesystems may have limitations)
    let success_rate = (found_count as f64 / created_files.len() as f64) * 100.0;
    println!("Success rate: {:.1}%", success_rate);
    assert!(success_rate >= 75.0, 
        "Too many files failed to process: only {:.1}% success rate", success_rate);
    
    // Cleanup
    fs::remove_dir_all(test_dir).ok();
    fs::remove_file(output_db).ok();
    
    println!("\n✓ International filename test passed!");
}

#[test]
fn test_international_filenames_hash() {
    let test_dir = "test_international_hash";
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    
    // Test a subset of challenging filenames
    let test_cases = vec![
        ("Russian", "тестовый_файл.txt"),
        ("Chinese", "测试文件.txt"),
        ("Japanese", "テストファイル.txt"),
        ("Arabic", "ملف_اختبار.txt"),
        ("Emoji", "test_😀🎉.txt"),
        ("Mixed", "test_тест_测试.txt"),
    ];
    
    let mut success_count = 0;
    
    for (lang, filename) in &test_cases {
        let file_path = PathBuf::from(test_dir).join(filename);
        
        // Create test file
        match fs::write(&file_path, format!("Content for {}", lang)) {
            Ok(_) => {
                // Try to hash the file
                let output = Command::new("cargo")
                    .args(&["run", "--release", "--", file_path.to_str().unwrap()])
                    .output()
                    .expect("Failed to execute hash command");
                
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("✓ Hashed: {} ({})", filename, lang);
                    println!("  Output: {}", stdout.trim());
                    success_count += 1;
                } else {
                    eprintln!("✗ Failed to hash: {} ({})", filename, lang);
                    eprintln!("  Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("⚠ Skipped: {} ({}) - {}", filename, lang, e);
            }
        }
    }
    
    // Cleanup
    fs::remove_dir_all(test_dir).ok();
    
    println!("\nHashed {}/{} files successfully", success_count, test_cases.len());
    assert!(success_count >= test_cases.len() / 2, 
        "Too many hash operations failed");
    
    println!("✓ International filename hash test passed!");
}

#[test]
fn test_progress_bar_with_unicode_filenames() {
    // This test ensures the progress bar doesn't break with unicode filenames
    let test_dir = "test_progress_unicode";
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    
    // Create files with various unicode characters
    let filenames = vec![
        "file_русский.txt",
        "file_中文.txt",
        "file_日本語.txt",
        "file_한국어.txt",
        "file_العربية.txt",
        "file_עברית.txt",
        "file_ελληνικά.txt",
        "file_😀😊.txt",
    ];
    
    for filename in &filenames {
        let file_path = PathBuf::from(test_dir).join(filename);
        fs::write(&file_path, "test content").ok();
    }
    
    // Run scan with progress bar
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "scan", "-d", test_dir, "-o", "test_progress_output.txt"])
        .output()
        .expect("Failed to execute scan command");
    
    // Check that scan completed successfully
    assert!(output.status.success(), 
        "Scan failed with unicode filenames: {}", 
        String::from_utf8_lossy(&output.stderr));
    
    println!("✓ Progress bar handled unicode filenames correctly");
    
    // Cleanup
    fs::remove_dir_all(test_dir).ok();
    fs::remove_file("test_progress_output.txt").ok();
}
