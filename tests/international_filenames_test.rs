// Test for handling filenames in 100+ languages and special characters
// Ensures the hash utility works correctly with international filenames

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Test data: filenames in various languages and scripts
/// Covers major writing systems and special characters
fn get_international_test_filenames() -> Vec<(&'static str, &'static str)> {
    vec![
        // Latin-based languages
        ("English", "test_file.txt"),
        ("French", "fichier_testé.txt"),
        ("German", "Prüfungsdatei_äöü.txt"),
        ("Spanish", "archivo_español_ñ.txt"),
        ("Portuguese", "arquivo_português_ção.txt"),
        ("Italian", "file_italiano_è.txt"),
        ("Polish", "plik_testowy_ąćęłńóśźż.txt"),
        ("Czech", "testovací_soubor_čřž.txt"),
        ("Turkish", "test_dosyası_şğı.txt"),
        ("Romanian", "fișier_test_ăîșț.txt"),
        ("Hungarian", "teszt_fájl_őű.txt"),
        ("Vietnamese", "tệp_thử_nghiệm_ăâđ.txt"),
        
        // Cyrillic script
        ("Russian", "тестовый_файл.txt"),
        ("Ukrainian", "тестовий_файл_їє.txt"),
        ("Bulgarian", "тестов_файл_ъ.txt"),
        ("Serbian", "тест_датотека_ђћ.txt"),
        ("Macedonian", "тест_датотека_ѓќ.txt"),
        ("Belarusian", "тэставы_файл_ў.txt"),
        ("Kazakh", "сынақ_файлы_әіңғ.txt"),
        
        // Greek
        ("Greek", "δοκιμαστικό_αρχείο_αβγ.txt"),
        
        // Arabic script (RTL)
        ("Arabic", "ملف_اختبار.txt"),
        ("Persian", "فایل_آزمایشی.txt"),
        ("Urdu", "ٹیسٹ_فائل.txt"),
        
        // Hebrew (RTL)
        ("Hebrew", "קובץ_בדיקה.txt"),
        
        // CJK (Chinese, Japanese, Korean)
        ("Chinese_Simplified", "测试文件.txt"),
        ("Chinese_Traditional", "測試文件.txt"),
        ("Japanese_Hiragana", "てすとふぁいる.txt"),
        ("Japanese_Katakana", "テストファイル.txt"),
        ("Japanese_Kanji", "試験ファイル.txt"),
        ("Japanese_Mixed", "テスト試験ファイル.txt"),
        ("Korean_Hangul", "테스트_파일.txt"),
        ("Korean_Mixed", "테스트_파일_試驗.txt"),
        
        // South Asian scripts
        ("Hindi", "परीक्षण_फ़ाइल.txt"),
        ("Bengali", "পরীক্ষা_ফাইল.txt"),
        ("Tamil", "சோதனை_கோப்பு.txt"),
        ("Telugu", "పరీక్ష_ఫైలు.txt"),
        ("Gujarati", "પરીક્ષણ_ફાઇલ.txt"),
        ("Kannada", "ಪರೀಕ್ಷಾ_ಕಡತ.txt"),
        ("Malayalam", "പരീക്ഷണ_ഫയൽ.txt"),
        ("Punjabi", "ਟੈਸਟ_ਫਾਇਲ.txt"),
        ("Sinhala", "පරීක්ෂණ_ගොනුව.txt"),
        
        // Southeast Asian scripts
        ("Thai", "ไฟล์ทดสอบ.txt"),
        ("Lao", "ໄຟລ໌ທົດສອບ.txt"),
        ("Burmese", "စမ်းသပ်ဖိုင်.txt"),
        ("Khmer", "ឯកសារសាកល្បង.txt"),
        
        // Other scripts
        ("Georgian", "სატესტო_ფაილი.txt"),
        ("Armenian", "փորձարկման_ֆայլ.txt"),
        ("Amharic", "የሙከራ_ፋይል.txt"),
        ("Tigrinya", "ፈተና_ፋይል.txt"),
        
        // Special characters and symbols
        ("Emoji", "test_file_😀🎉🔥.txt"),
        ("Mixed_Emoji", "测试_test_файл_😊.txt"),
        ("Math_Symbols", "file_∑∫∂∇.txt"),
        ("Currency", "file_€£¥₹₽.txt"),
        ("Arrows", "file_←→↑↓.txt"),
        ("Box_Drawing", "file_│─┌┐.txt"),
        
        // Edge cases
        ("Spaces", "file with spaces.txt"),
        ("Multiple_Spaces", "file  with   multiple    spaces.txt"),
        ("Leading_Space", " leading_space.txt"),
        ("Trailing_Space", "trailing_space .txt"),
        ("Dots", "file.with.many.dots.txt"),
        ("Dashes", "file-with-many-dashes.txt"),
        ("Underscores", "file_with_many_underscores.txt"),
        ("Mixed_Separators", "file-with_mixed.separators.txt"),
        ("Numbers", "12345_67890.txt"),
        ("Mixed_Numbers", "file123test456.txt"),
        
        // Long filenames
        ("Long_ASCII", "this_is_a_very_long_filename_that_tests_the_limits_of_filename_handling_in_various_systems.txt"),
        ("Long_Unicode", "これは非常に長いファイル名でシステムの制限をテストします.txt"),
        
        // Combined scripts
        ("Latin_Cyrillic", "test_тест.txt"),
        ("Latin_Arabic", "test_اختبار.txt"),
        ("Latin_CJK", "test_测试.txt"),
        ("Multi_Script", "test_тест_测试_テスト.txt"),
        
        // Case sensitivity tests
        ("Uppercase", "UPPERCASE_FILE.TXT"),
        ("Lowercase", "lowercase_file.txt"),
        ("MixedCase", "MiXeD_CaSe_FiLe.txt"),
        
        // Additional languages
        ("Icelandic", "prófunarskrá_þæð.txt"),
        ("Norwegian", "testfil_æøå.txt"),
        ("Swedish", "testfil_åäö.txt"),
        ("Danish", "testfil_æøå.txt"),
        ("Finnish", "testitiedosto_äö.txt"),
        ("Estonian", "testfail_õäöü.txt"),
        ("Latvian", "testa_fails_āčēģ.txt"),
        ("Lithuanian", "bandomasis_failas_ąčė.txt"),
        ("Slovak", "testovací_súbor_áäô.txt"),
        ("Slovenian", "testna_datoteka_čšž.txt"),
        ("Croatian", "testna_datoteka_čćđ.txt"),
        ("Bosnian", "testna_datoteka_čćđ.txt"),
        ("Albanian", "skedar_testimi_ëç.txt"),
        ("Maltese", "fajl_test_ċġħ.txt"),
        ("Welsh", "ffeil_prawf_ŵŷ.txt"),
        ("Irish", "comhad_tástála_áéí.txt"),
        ("Scottish_Gaelic", "faidhle_deuchainn.txt"),
        ("Basque", "proba_fitxategia.txt"),
        ("Catalan", "fitxer_prova_àèé.txt"),
        ("Galician", "ficheiro_proba_áéí.txt"),
        ("Esperanto", "testa_dosiero_ĉĝĥ.txt"),
        
        // More Asian languages
        ("Mongolian", "туршилтын_файл.txt"),
        ("Tibetan", "བརྟག་དཔྱད་ཡིག་ཆ.txt"),
        ("Nepali", "परीक्षण_फाइल.txt"),
        ("Marathi", "चाचणी_फाइल.txt"),
        ("Oriya", "ପରୀକ୍ଷା_ଫାଇଲ.txt"),
        ("Assamese", "পৰীক্ষা_ফাইল.txt"),
        
        // African languages
        ("Swahili", "faili_ya_majaribio.txt"),
        ("Hausa", "fayil_gwaji.txt"),
        ("Yoruba", "faili_idanwo_ẹọṣ.txt"),
        ("Zulu", "ifayela_lokuhlola.txt"),
        ("Afrikaans", "toetslêer_êëï.txt"),
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
    
    println!("\nSuccessfully created {}/{} test files", created_files.len(), test_filenames.len());
    
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
    
    // We expect at least 80% of files to be processed successfully
    let success_rate = (found_count as f64 / created_files.len() as f64) * 100.0;
    println!("Success rate: {:.1}%", success_rate);
    assert!(success_rate >= 80.0, 
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
