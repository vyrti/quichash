// Verification module
// Compares current hashes against stored database

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::database::DatabaseHandler;
use crate::hash::HashComputer;
use crate::path_utils;
use crate::error::HashUtilityError;

// Re-export HashUtilityError as VerifyError for backward compatibility
pub type VerifyError = HashUtilityError;

/// Represents a hash mismatch between expected and actual values
#[derive(Debug, Clone)]
pub struct Mismatch {
    pub path: PathBuf,
    pub expected: String,
    pub actual: String,
}

/// Report of verification results
#[derive(Debug)]
pub struct VerifyReport {
    pub matches: usize,
    pub mismatches: Vec<Mismatch>,
    pub missing_files: Vec<PathBuf>,
    pub new_files: Vec<PathBuf>,
}

impl VerifyReport {
    /// Display a detailed report of verification results
    pub fn display(&self) {
        println!("\n=== Verification Report ===\n");
        
        println!("Matches: {}", self.matches);
        println!("Mismatches: {}", self.mismatches.len());
        println!("Missing files: {}", self.missing_files.len());
        println!("New files: {}", self.new_files.len());
        
        if !self.mismatches.is_empty() {
            println!("\n--- Files with Changed Hashes ---");
            for mismatch in &self.mismatches {
                println!("  File: {}", mismatch.path.display());
                println!("    Expected: {}", mismatch.expected);
                println!("    Actual:   {}", mismatch.actual);
            }
        }
        
        if !self.missing_files.is_empty() {
            println!("\n--- Deleted Files (in database but not filesystem) ---");
            for path in &self.missing_files {
                println!("  {}", path.display());
            }
        }
        
        if !self.new_files.is_empty() {
            println!("\n--- New Files (in filesystem but not database) ---");
            for path in &self.new_files {
                println!("  {}", path.display());
            }
        }
        
        println!("\n=== Summary ===");
        let total_checked = self.matches + self.mismatches.len();
        println!("Total files checked: {}", total_checked);
        println!("Total files in database: {}", total_checked + self.missing_files.len());
        println!("Total files in filesystem: {}", total_checked + self.new_files.len());
    }
}

/// Engine for verifying file integrity against a hash database
pub struct VerifyEngine {
    computer: HashComputer,
}

impl VerifyEngine {
    /// Create a new VerifyEngine
    pub fn new() -> Self {
        Self {
            computer: HashComputer::new(),
        }
    }
    
    /// Verify directory contents against a hash database
    /// 
    /// This function:
    /// 1. Loads the hash database from the specified file
    /// 2. Recursively scans the directory to find all files
    /// 3. Computes current hashes for files in the database
    /// 4. Classifies files as: matches, mismatches, missing, or new
    /// 5. Returns a detailed report
    pub fn verify(
        &self,
        database_path: &Path,
        directory: &Path,
    ) -> Result<VerifyReport, VerifyError> {
        // Verify database file exists
        if !database_path.exists() {
            return Err(HashUtilityError::DatabaseNotFound {
                path: database_path.to_path_buf(),
            });
        }
        
        // Verify directory exists
        if !directory.exists() || !directory.is_dir() {
            return Err(HashUtilityError::DirectoryNotFound {
                path: directory.to_path_buf(),
            });
        }
        
        // Load the hash database
        let database = DatabaseHandler::read_database(database_path)?;
        
        // Extract algorithm from first hash in database (all should use same algorithm)
        let algorithm = self.detect_algorithm(&database)?;
        
        // Get canonical path of database file to exclude it from scan
        let database_canonical = database_path.canonicalize().ok();
        
        // Collect all files in the directory (as canonical paths), excluding the database file
        let mut current_files = self.collect_files(directory)?;
        if let Some(db_path) = database_canonical {
            current_files.remove(&db_path);
        }
        
        // Convert database paths to canonical for comparison
        let database_canonical = self.resolve_database_paths(&database, directory)?;
        
        // Track results
        let mut matches = 0;
        let mut mismatches = Vec::new();
        let mut missing_files = Vec::new();
        let mut checked_files = HashSet::new();
        
        // Check each file in the database
        for (db_path, expected_hash) in &database_canonical {
            checked_files.insert(db_path.clone());
            
            if current_files.contains(db_path) {
                // File exists, compute current hash
                match self.computer.compute_hash(db_path, &algorithm) {
                    Ok(result) => {
                        if result.hash == *expected_hash {
                            matches += 1;
                        } else {
                            mismatches.push(Mismatch {
                                path: db_path.clone(),
                                expected: expected_hash.clone(),
                                actual: result.hash,
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to hash {}: {}", db_path.display(), e);
                    }
                }
            } else {
                // File in database but not in filesystem
                missing_files.push(db_path.clone());
            }
        }
        
        // Find new files (in filesystem but not in database)
        let new_files: Vec<PathBuf> = current_files
            .iter()
            .filter(|path| !checked_files.contains(*path))
            .cloned()
            .collect();
        
        Ok(VerifyReport {
            matches,
            mismatches,
            missing_files,
            new_files,
        })
    }
    
    /// Detect the hash algorithm used in the database based on hash length
    fn detect_algorithm(&self, database: &HashMap<PathBuf, String>) -> Result<String, VerifyError> {
        if database.is_empty() {
            return Err(HashUtilityError::EmptyDatabase {
                path: PathBuf::from("database"),
            });
        }
        
        // Get first hash to determine algorithm
        let first_hash = database.values().next().unwrap();
        let hash_len = first_hash.len();
        
        // Map hash length to algorithm (in hex characters)
        let algorithm = match hash_len {
            32 => "md5",           // 128 bits = 32 hex chars
            40 => "sha1",          // 160 bits = 40 hex chars
            56 => "sha224",        // 224 bits = 56 hex chars
            64 => "sha256",        // 256 bits = 64 hex chars (also SHA3-256, BLAKE2s, BLAKE3)
            96 => "sha384",        // 384 bits = 96 hex chars
            128 => "sha512",       // 512 bits = 128 hex chars (also SHA3-512, BLAKE2b)
            _ => {
                return Err(HashUtilityError::DatabaseParseError {
                    path: PathBuf::from("database"),
                    line: 1,
                    reason: format!("Unknown hash length: {} characters", hash_len),
                });
            }
        };
        
        Ok(algorithm.to_string())
    }
    
    /// Recursively collect all files in a directory (returns canonical paths)
    fn collect_files(&self, directory: &Path) -> Result<HashSet<PathBuf>, VerifyError> {
        let mut files = HashSet::new();
        self.collect_files_recursive(directory, &mut files)?;
        Ok(files)
    }
    
    /// Helper function for recursive file collection
    fn collect_files_recursive(
        &self,
        directory: &Path,
        files: &mut HashSet<PathBuf>,
    ) -> Result<(), VerifyError> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                // Canonicalize the path for consistent comparison
                match path.canonicalize() {
                    Ok(canonical) => {
                        files.insert(canonical);
                    }
                    Err(_) => {
                        // Skip files that can't be canonicalized
                        continue;
                    }
                }
            } else if path.is_dir() {
                self.collect_files_recursive(&path, files)?;
            }
        }
        
        Ok(())
    }
    
    /// Resolve database paths to absolute paths for comparison
    /// Uses path_utils for proper cross-platform path handling
    fn resolve_database_paths(
        &self,
        database: &HashMap<PathBuf, String>,
        base_directory: &Path,
    ) -> Result<HashMap<PathBuf, String>, VerifyError> {
        let mut resolved = HashMap::new();
        
        for (path, hash) in database {
            // Use path_utils to resolve the path properly
            let absolute_path = path_utils::resolve_path(path, base_directory);
            
            // Try to canonicalize if the file exists, otherwise use as-is
            let final_path = match path_utils::try_canonicalize(&absolute_path) {
                Ok(canonical) => canonical,
                Err(_) => absolute_path,
            };
            
            resolved.insert(final_path, hash.clone());
        }
        
        Ok(resolved)
    }
}

impl Default for VerifyEngine {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_test_file(path: &Path, content: &[u8]) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    #[test]
    fn test_verify_all_matches() {
        // Create test directory structure
        let test_dir = "test_verify_matches";
        fs::create_dir_all(test_dir).unwrap();
        
        // Create test files
        create_test_file(&PathBuf::from(format!("{}/file1.txt", test_dir)), b"hello");
        create_test_file(&PathBuf::from(format!("{}/file2.txt", test_dir)), b"world");
        
        // Create database with correct hashes (SHA-256)
        let db_path = format!("{}/database.txt", test_dir);
        let mut db_file = fs::File::create(&db_path).unwrap();
        writeln!(db_file, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  file1.txt").unwrap();
        writeln!(db_file, "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7  file2.txt").unwrap();
        
        // Run verification
        let engine = VerifyEngine::new();
        let report = engine.verify(Path::new(&db_path), Path::new(test_dir)).unwrap();
        
        // Verify results
        assert_eq!(report.matches, 2);
        assert_eq!(report.mismatches.len(), 0);
        assert_eq!(report.missing_files.len(), 0);
        assert_eq!(report.new_files.len(), 0);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_verify_with_mismatch() {
        // Create test directory
        let test_dir = "test_verify_mismatch";
        fs::create_dir_all(test_dir).unwrap();
        
        // Create test file
        create_test_file(&PathBuf::from(format!("{}/file1.txt", test_dir)), b"modified content");
        
        // Create database with old hash
        let db_path = format!("{}/database.txt", test_dir);
        let mut db_file = fs::File::create(&db_path).unwrap();
        writeln!(db_file, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  file1.txt").unwrap();
        
        // Run verification
        let engine = VerifyEngine::new();
        let report = engine.verify(Path::new(&db_path), Path::new(test_dir)).unwrap();
        
        // Verify results
        assert_eq!(report.matches, 0);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(report.missing_files.len(), 0);
        assert_eq!(report.new_files.len(), 0);
        
        // Check mismatch details
        let mismatch = &report.mismatches[0];
        assert_eq!(mismatch.expected, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        assert_ne!(mismatch.actual, mismatch.expected);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_verify_with_missing_file() {
        // Create test directory
        let test_dir = "test_verify_missing";
        fs::create_dir_all(test_dir).unwrap();
        
        // Create database with file that doesn't exist
        let db_path = format!("{}/database.txt", test_dir);
        let mut db_file = fs::File::create(&db_path).unwrap();
        writeln!(db_file, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  missing_file.txt").unwrap();
        
        // Run verification
        let engine = VerifyEngine::new();
        let report = engine.verify(Path::new(&db_path), Path::new(test_dir)).unwrap();
        
        // Verify results
        assert_eq!(report.matches, 0);
        assert_eq!(report.mismatches.len(), 0);
        assert_eq!(report.missing_files.len(), 1);
        assert_eq!(report.new_files.len(), 0);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_verify_with_new_file() {
        // Create test directory
        let test_dir = "test_verify_new";
        fs::create_dir_all(test_dir).unwrap();
        
        // Create test file
        create_test_file(&PathBuf::from(format!("{}/new_file.txt", test_dir)), b"new content");
        
        // Create empty database
        let db_path = format!("{}/database.txt", test_dir);
        let mut db_file = fs::File::create(&db_path).unwrap();
        writeln!(db_file, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  dummy.txt").unwrap();
        
        // Run verification
        let engine = VerifyEngine::new();
        let report = engine.verify(Path::new(&db_path), Path::new(test_dir)).unwrap();
        
        // Verify results - should have 1 missing (dummy.txt) and 1 new (new_file.txt)
        assert_eq!(report.matches, 0);
        assert_eq!(report.mismatches.len(), 0);
        assert_eq!(report.missing_files.len(), 1);
        assert_eq!(report.new_files.len(), 1);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_verify_mixed_results() {
        // Create test directory
        let test_dir = "test_verify_mixed";
        fs::create_dir_all(test_dir).unwrap();
        
        // Create test files
        create_test_file(&PathBuf::from(format!("{}/match.txt", test_dir)), b"hello");
        create_test_file(&PathBuf::from(format!("{}/mismatch.txt", test_dir)), b"modified");
        create_test_file(&PathBuf::from(format!("{}/new.txt", test_dir)), b"new");
        
        // Create database
        let db_path = format!("{}/database.txt", test_dir);
        let mut db_file = fs::File::create(&db_path).unwrap();
        // match.txt - correct hash
        writeln!(db_file, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  match.txt").unwrap();
        // mismatch.txt - wrong hash
        writeln!(db_file, "0000000000000000000000000000000000000000000000000000000000000000  mismatch.txt").unwrap();
        // missing.txt - file doesn't exist
        writeln!(db_file, "1111111111111111111111111111111111111111111111111111111111111111  missing.txt").unwrap();
        // new.txt is not in database
        
        // Run verification
        let engine = VerifyEngine::new();
        let report = engine.verify(Path::new(&db_path), Path::new(test_dir)).unwrap();
        
        // Verify results
        assert_eq!(report.matches, 1);
        assert_eq!(report.mismatches.len(), 1);
        assert_eq!(report.missing_files.len(), 1);
        assert_eq!(report.new_files.len(), 1);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_verify_database_not_found() {
        let engine = VerifyEngine::new();
        let result = engine.verify(
            Path::new("nonexistent_database.txt"),
            Path::new(".")
        );
        
        assert!(result.is_err());
        match result {
            Err(HashUtilityError::DatabaseNotFound { .. }) => {},
            _ => panic!("Expected DatabaseNotFound error"),
        }
    }

    #[test]
    fn test_verify_directory_not_found() {
        // Create a temporary database file
        let db_path = "test_db_temp.txt";
        fs::write(db_path, "abc123  file.txt\n").unwrap();
        
        let engine = VerifyEngine::new();
        let result = engine.verify(
            Path::new(db_path),
            Path::new("nonexistent_directory")
        );
        
        assert!(result.is_err());
        match result {
            Err(HashUtilityError::DirectoryNotFound { .. }) => {},
            _ => panic!("Expected DirectoryNotFound error"),
        }
        
        // Cleanup
        fs::remove_file(db_path).unwrap();
    }

    #[test]
    fn test_detect_algorithm_md5() {
        let engine = VerifyEngine::new();
        let mut database = HashMap::new();
        database.insert(PathBuf::from("file.txt"), "d41d8cd98f00b204e9800998ecf8427e".to_string());
        
        let algorithm = engine.detect_algorithm(&database).unwrap();
        assert_eq!(algorithm, "md5");
    }

    #[test]
    fn test_detect_algorithm_sha256() {
        let engine = VerifyEngine::new();
        let mut database = HashMap::new();
        database.insert(
            PathBuf::from("file.txt"),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()
        );
        
        let algorithm = engine.detect_algorithm(&database).unwrap();
        assert_eq!(algorithm, "sha256");
    }

    #[test]
    fn test_detect_algorithm_sha512() {
        let engine = VerifyEngine::new();
        let mut database = HashMap::new();
        database.insert(
            PathBuf::from("file.txt"),
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".to_string()
        );
        
        let algorithm = engine.detect_algorithm(&database).unwrap();
        assert_eq!(algorithm, "sha512");
    }

    #[test]
    fn test_detect_algorithm_empty_database() {
        let engine = VerifyEngine::new();
        let database = HashMap::new();
        
        let result = engine.detect_algorithm(&database);
        assert!(result.is_err());
    }

    #[test]
    fn test_collect_files_recursive() {
        // Create nested directory structure
        let test_dir = "test_verify_collect_files";
        fs::create_dir_all(format!("{}/subdir1", test_dir)).unwrap();
        fs::create_dir_all(format!("{}/subdir2", test_dir)).unwrap();
        
        create_test_file(&PathBuf::from(format!("{}/file1.txt", test_dir)), b"test");
        create_test_file(&PathBuf::from(format!("{}/subdir1/file2.txt", test_dir)), b"test");
        create_test_file(&PathBuf::from(format!("{}/subdir2/file3.txt", test_dir)), b"test");
        
        let engine = VerifyEngine::new();
        let files = engine.collect_files(Path::new(test_dir)).unwrap();
        
        // Should find all 3 files
        assert_eq!(files.len(), 3);
        
        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }
}
