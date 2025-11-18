// Database format handler module
// Reads and writes plain text hash database files

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::path_utils;
use crate::error::HashUtilityError;

/// Handler for reading and writing hash database files
pub struct DatabaseHandler;

impl DatabaseHandler {
    /// Write a single hash entry to the output writer
    /// Format: `<hash>  <filepath>` (two spaces between hash and filepath)
    pub fn write_entry(
        writer: &mut impl Write,
        hash: &str,
        path: &Path,
    ) -> io::Result<()> {
        writeln!(writer, "{}  {}", hash, path.display())
    }
    
    /// Read a hash database file and parse it into a HashMap
    /// Maps file paths to their hash values
    /// Malformed lines are skipped with a warning to stderr
    pub fn read_database(path: &Path) -> Result<HashMap<PathBuf, String>, HashUtilityError> {
        let file = File::open(path).map_err(|e| {
            HashUtilityError::from_io_error(e, "reading database", Some(path.to_path_buf()))
        })?;
        let reader = BufReader::new(file);
        let mut database = HashMap::new();
        
        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result.map_err(|e| {
                HashUtilityError::from_io_error(e, "reading database", Some(path.to_path_buf()))
            })?;
            
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }
            
            // Parse line: split on two spaces
            match Self::parse_line(&line) {
                Some((hash, file_path)) => {
                    database.insert(file_path, hash);
                }
                None => {
                    // Warn about malformed line but continue processing (Requirement 2.4)
                    eprintln!(
                        "Warning: Skipping malformed line {} in database {}: {}",
                        line_num + 1,
                        path.display(),
                        line
                    );
                }
            }
        }
        
        Ok(database)
    }
    
    /// Parse a single line from the database file
    /// Expected format: `<hash>  <filepath>` (two spaces)
    /// Returns None if the line is malformed
    /// Handles both forward and backward slashes in paths
    fn parse_line(line: &str) -> Option<(String, PathBuf)> {
        // Split on two spaces (the standard format)
        let parts: Vec<&str> = line.splitn(2, "  ").collect();
        
        if parts.len() == 2 {
            let hash = parts[0].trim();
            let path_str = parts[1].trim();
            
            // Validate that hash is not empty and path is not empty
            if !hash.is_empty() && !path_str.is_empty() {
                // Use path_utils to parse the path with proper separator handling
                let path = path_utils::parse_database_path(path_str);
                return Some((hash.to_string(), path));
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_entry() {
        let mut buffer = Vec::new();
        let hash = "d41d8cd98f00b204e9800998ecf8427e";
        let path = Path::new("./test/file.txt");
        
        DatabaseHandler::write_entry(&mut buffer, hash, path).unwrap();
        
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "d41d8cd98f00b204e9800998ecf8427e  ./test/file.txt\n");
    }
    
    #[test]
    fn test_write_multiple_entries() {
        let mut buffer = Vec::new();
        
        DatabaseHandler::write_entry(
            &mut buffer,
            "abc123",
            Path::new("file1.txt")
        ).unwrap();
        
        DatabaseHandler::write_entry(
            &mut buffer,
            "def456",
            Path::new("file2.txt")
        ).unwrap();
        
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "abc123  file1.txt\ndef456  file2.txt\n");
    }
    
    #[test]
    fn test_parse_line_valid() {
        let line = "d41d8cd98f00b204e9800998ecf8427e  ./test/file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_some());
        let (hash, path) = result.unwrap();
        assert_eq!(hash, "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(path, PathBuf::from("./test/file.txt"));
    }
    
    #[test]
    fn test_parse_line_with_spaces_in_path() {
        let line = "abc123  ./path with spaces/file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_some());
        let (hash, path) = result.unwrap();
        assert_eq!(hash, "abc123");
        assert_eq!(path, PathBuf::from("./path with spaces/file.txt"));
    }
    
    #[test]
    fn test_parse_line_malformed_single_space() {
        let line = "abc123 file.txt";  // Only one space
        let result = DatabaseHandler::parse_line(line);
        
        // Should fail because we expect two spaces
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_line_malformed_no_space() {
        let line = "abc123file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_line_empty_hash() {
        let line = "  file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_line_empty_path() {
        let line = "abc123  ";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_none());
    }
    
    #[test]
    fn test_read_database() {
        // Create a temporary database file
        let temp_file = "test_db_temp.txt";
        let content = "d41d8cd98f00b204e9800998ecf8427e  ./empty.txt\n\
                       5d41402abc4b2a76b9719d911017c592  ./hello.txt\n\
                       098f6bcd4621d373cade4e832627b4f6  ./test/data.bin\n";
        fs::write(temp_file, content).unwrap();
        
        // Read database
        let database = DatabaseHandler::read_database(Path::new(temp_file)).unwrap();
        
        // Verify entries
        assert_eq!(database.len(), 3);
        assert_eq!(
            database.get(&PathBuf::from("./empty.txt")),
            Some(&"d41d8cd98f00b204e9800998ecf8427e".to_string())
        );
        assert_eq!(
            database.get(&PathBuf::from("./hello.txt")),
            Some(&"5d41402abc4b2a76b9719d911017c592".to_string())
        );
        assert_eq!(
            database.get(&PathBuf::from("./test/data.bin")),
            Some(&"098f6bcd4621d373cade4e832627b4f6".to_string())
        );
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_read_database_with_empty_lines() {
        let temp_file = "test_db_empty_lines_temp.txt";
        let content = "abc123  file1.txt\n\
                       \n\
                       def456  file2.txt\n\
                       \n";
        fs::write(temp_file, content).unwrap();
        
        let database = DatabaseHandler::read_database(Path::new(temp_file)).unwrap();
        
        assert_eq!(database.len(), 2);
        assert!(database.contains_key(&PathBuf::from("file1.txt")));
        assert!(database.contains_key(&PathBuf::from("file2.txt")));
        
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_read_database_with_malformed_lines() {
        let temp_file = "test_db_malformed_temp.txt";
        let content = "abc123  file1.txt\n\
                       malformed line without proper format\n\
                       def456  file2.txt\n";
        fs::write(temp_file, content).unwrap();
        
        // Should skip malformed line and continue
        let database = DatabaseHandler::read_database(Path::new(temp_file)).unwrap();
        
        assert_eq!(database.len(), 2);
        assert!(database.contains_key(&PathBuf::from("file1.txt")));
        assert!(database.contains_key(&PathBuf::from("file2.txt")));
        
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_read_database_file_not_found() {
        let result = DatabaseHandler::read_database(Path::new("nonexistent_db.txt"));
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_round_trip() {
        // Write entries to a buffer
        let mut buffer = Vec::new();
        DatabaseHandler::write_entry(&mut buffer, "hash1", Path::new("file1.txt")).unwrap();
        DatabaseHandler::write_entry(&mut buffer, "hash2", Path::new("file2.txt")).unwrap();
        
        // Write buffer to file
        let temp_file = "test_round_trip_temp.txt";
        fs::write(temp_file, &buffer).unwrap();
        
        // Read back
        let database = DatabaseHandler::read_database(Path::new(temp_file)).unwrap();
        
        // Verify
        assert_eq!(database.len(), 2);
        assert_eq!(database.get(&PathBuf::from("file1.txt")), Some(&"hash1".to_string()));
        assert_eq!(database.get(&PathBuf::from("file2.txt")), Some(&"hash2".to_string()));
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_parse_line_with_forward_slashes() {
        let line = "abc123  path/to/file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_some());
        let (hash, path) = result.unwrap();
        assert_eq!(hash, "abc123");
        // Path should be parsed correctly regardless of platform
        assert!(path.to_str().unwrap().contains("file.txt"));
    }
    
    #[test]
    fn test_parse_line_with_backward_slashes() {
        let line = "abc123  path\\to\\file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_some());
        let (hash, path) = result.unwrap();
        assert_eq!(hash, "abc123");
        // Path should be parsed correctly regardless of platform
        assert!(path.to_str().unwrap().contains("file.txt"));
    }
    
    #[test]
    fn test_parse_line_with_mixed_slashes() {
        let line = "abc123  path/to\\mixed/file.txt";
        let result = DatabaseHandler::parse_line(line);
        
        assert!(result.is_some());
        let (hash, path) = result.unwrap();
        assert_eq!(hash, "abc123");
        // Path should be parsed correctly with normalized separators
        assert!(path.to_str().unwrap().contains("file.txt"));
    }
    
    #[test]
    fn test_read_database_with_mixed_separators() {
        let temp_file = "test_db_mixed_sep_temp.txt";
        // Create database with mixed path separators
        let content = "abc123  path/to/file1.txt\n\
                       def456  path\\to\\file2.txt\n\
                       ghi789  path/to\\file3.txt\n";
        fs::write(temp_file, content).unwrap();
        
        let database = DatabaseHandler::read_database(Path::new(temp_file)).unwrap();
        
        // All paths should be parsed successfully
        assert_eq!(database.len(), 3);
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
}
