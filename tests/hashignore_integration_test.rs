// Integration test for .hashignore functionality
// Tests that .hashignore patterns correctly exclude files from scans

use std::fs;
use std::process::Command;

#[test]
fn test_hashignore_excludes_files() {
    // Create a test directory structure
    let test_dir = "test_hashignore_integration";
    fs::create_dir_all(format!("{}/subdir", test_dir)).unwrap();
    
    // Create various files
    fs::write(format!("{}/include.txt", test_dir), b"should be included").unwrap();
    fs::write(format!("{}/exclude.log", test_dir), b"should be excluded").unwrap();
    fs::write(format!("{}/exclude.tmp", test_dir), b"should be excluded").unwrap();
    fs::write(format!("{}/subdir/include.txt", test_dir), b"should be included").unwrap();
    fs::write(format!("{}/subdir/exclude.log", test_dir), b"should be excluded").unwrap();
    
    // Create .hashignore file
    let hashignore_content = "*.log\n*.tmp\n";
    fs::write(format!("{}/.hashignore", test_dir), hashignore_content).unwrap();
    
    // Run scan command
    let output_file = format!("{}/hashes.txt", test_dir);
    let output = Command::new("cargo")
        .args(&["run", "--", "scan", "-d", test_dir, "-a", "sha256", "-o", &output_file])
        .output()
        .expect("Failed to execute scan command");
    
    // Check that command succeeded
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Scan command failed");
    }
    
    // Read output file
    let content = fs::read_to_string(&output_file).unwrap();
    
    // Verify that included files are present
    assert!(content.contains("include.txt"), "include.txt should be in output");
    
    // Verify that excluded files are NOT present
    assert!(!content.contains("exclude.log"), "exclude.log should NOT be in output");
    assert!(!content.contains("exclude.tmp"), "exclude.tmp should NOT be in output");
    assert!(!content.contains(".hashignore"), ".hashignore itself should NOT be in output");
    
    // Count lines to verify correct number of files
    let line_count = content.lines().filter(|l| !l.is_empty()).count();
    assert_eq!(line_count, 2, "Should have exactly 2 files (2 include.txt files)");
    
    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_hashignore_directory_patterns() {
    // Create a test directory structure
    let test_dir = "test_hashignore_dirs";
    fs::create_dir_all(format!("{}/build", test_dir)).unwrap();
    fs::create_dir_all(format!("{}/src", test_dir)).unwrap();
    
    // Create files
    fs::write(format!("{}/src/main.rs", test_dir), b"source code").unwrap();
    fs::write(format!("{}/build/output.o", test_dir), b"build artifact").unwrap();
    fs::write(format!("{}/README.md", test_dir), b"readme").unwrap();
    
    // Create .hashignore file with directory pattern
    let hashignore_content = "build/\n";
    fs::write(format!("{}/.hashignore", test_dir), hashignore_content).unwrap();
    
    // Run scan command
    let output_file = format!("{}/hashes.txt", test_dir);
    let output = Command::new("cargo")
        .args(&["run", "--", "scan", "-d", test_dir, "-a", "sha256", "-o", &output_file])
        .output()
        .expect("Failed to execute scan command");
    
    // Check that command succeeded
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Scan command failed");
    }
    
    // Read output file
    let content = fs::read_to_string(&output_file).unwrap();
    
    // Verify that files in build/ are excluded
    assert!(!content.contains("output.o"), "build/output.o should NOT be in output");
    
    // Verify that other files are included
    assert!(content.contains("main.rs"), "src/main.rs should be in output");
    assert!(content.contains("README.md"), "README.md should be in output");
    
    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_hashignore_negation_patterns() {
    // Create a test directory structure
    let test_dir = "test_hashignore_negation";
    fs::create_dir_all(test_dir).unwrap();
    
    // Create files
    fs::write(format!("{}/debug.log", test_dir), b"debug log").unwrap();
    fs::write(format!("{}/error.log", test_dir), b"error log").unwrap();
    fs::write(format!("{}/important.log", test_dir), b"important log").unwrap();
    fs::write(format!("{}/data.txt", test_dir), b"data").unwrap();
    
    // Create .hashignore file with negation pattern
    let hashignore_content = "*.log\n!important.log\n";
    fs::write(format!("{}/.hashignore", test_dir), hashignore_content).unwrap();
    
    // Run scan command
    let output_file = format!("{}/hashes.txt", test_dir);
    let output = Command::new("cargo")
        .args(&["run", "--", "scan", "-d", test_dir, "-a", "sha256", "-o", &output_file])
        .output()
        .expect("Failed to execute scan command");
    
    // Check that command succeeded
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Scan command failed");
    }
    
    // Read output file
    let content = fs::read_to_string(&output_file).unwrap();
    
    // Verify that most .log files are excluded
    assert!(!content.contains("debug.log"), "debug.log should NOT be in output");
    assert!(!content.contains("error.log"), "error.log should NOT be in output");
    
    // Verify that important.log is included (negation pattern)
    assert!(content.contains("important.log"), "important.log should be in output (negation)");
    
    // Verify that other files are included
    assert!(content.contains("data.txt"), "data.txt should be in output");
    
    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}
