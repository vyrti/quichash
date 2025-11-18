// CLI interface module
// Handles command-line argument parsing and validation

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::error::HashUtilityError;

/// Hash Utility - Cryptographic hash computation and verification tool
/// 
/// A cross-platform console application for computing cryptographic hashes,
/// scanning directories, and verifying file integrity.
#[derive(Parser, Debug)]
#[command(name = "hash-utility")]
#[command(version = "0.1.0")]
#[command(about = "Cryptographic hash computation and verification tool", long_about = None)]
#[command(after_help = "EXAMPLES:\n  \
    hash-utility hash -f file.txt -a sha256\n  \
    hash-utility scan -d /path/to/dir -a sha256 -o hashes.txt\n  \
    hash-utility verify -b hashes.txt -d /path/to/dir\n  \
    hash-utility benchmark\n  \
    hash-utility list")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Compute hash for a single file
    /// 
    /// Computes cryptographic hash(es) for the specified file using one or more algorithms.
    /// Multiple algorithms can be specified to compute all hashes in a single pass.
    Hash {
        /// File to hash
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: PathBuf,
        
        /// Hash algorithm(s) to use (can be specified multiple times)
        #[arg(short = 'a', long = "algorithm", value_name = "ALGORITHM", default_value = "sha256")]
        algorithms: Vec<String>,
        
        /// Output file (optional, defaults to stdout)
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: Option<PathBuf>,
    },
    
    /// Scan directory and generate hash database
    /// 
    /// Recursively scans a directory and computes hashes for all files,
    /// storing the results in a plain text database file.
    Scan {
        /// Directory to scan
        #[arg(short = 'd', long = "directory", value_name = "DIR")]
        directory: PathBuf,
        
        /// Hash algorithm to use
        #[arg(short = 'a', long = "algorithm", value_name = "ALGORITHM", default_value = "sha256")]
        algorithm: String,
        
        /// Output database file
        #[arg(short = 'o', long = "output", value_name = "FILE")]
        output: PathBuf,
        
        /// Enable parallel processing
        #[arg(short = 'p', long = "parallel")]
        parallel: bool,
    },
    
    /// Verify directory against hash database
    /// 
    /// Compares current file hashes against a stored database to detect
    /// modifications, deletions, and new files.
    Verify {
        /// Hash database file
        #[arg(short = 'b', long = "database", value_name = "FILE")]
        database: PathBuf,
        
        /// Directory to verify
        #[arg(short = 'd', long = "directory", value_name = "DIR")]
        directory: PathBuf,
    },
    
    /// Benchmark hash algorithms
    /// 
    /// Tests all supported hash algorithms and displays their throughput
    /// on the current hardware.
    Benchmark {
        /// Size of test data in MB
        #[arg(short = 's', long = "size", value_name = "MB", default_value = "100")]
        size_mb: usize,
    },
    
    /// List available hash algorithms
    /// 
    /// Displays all supported hash algorithms with their properties,
    /// including output size and post-quantum resistance status.
    List,
}

/// Parse command-line arguments
/// 
/// # Returns
/// Parsed CLI structure containing the command and its arguments
/// 
/// # Errors
/// Returns an error if arguments are invalid or missing required values
pub fn parse_args() -> Result<Cli, HashUtilityError> {
    match Cli::try_parse() {
        Ok(cli) => Ok(cli),
        Err(e) => Err(HashUtilityError::InvalidArguments {
            message: e.to_string(),
        }),
    }
}

// Re-export HashUtilityError as CliError for backward compatibility
pub type CliError = HashUtilityError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hash_command() {
        let args = vec!["hash-utility", "hash", "-f", "test.txt", "-a", "sha256"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Hash { file, algorithms, output } => {
                assert_eq!(file, PathBuf::from("test.txt"));
                assert_eq!(algorithms, vec!["sha256"]);
                assert_eq!(output, None);
            }
            _ => panic!("Expected Hash command"),
        }
    }
    
    #[test]
    fn test_parse_hash_command_multiple_algorithms() {
        let args = vec!["hash-utility", "hash", "-f", "test.txt", "-a", "sha256", "-a", "md5"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Hash { file, algorithms, output } => {
                assert_eq!(file, PathBuf::from("test.txt"));
                assert_eq!(algorithms, vec!["sha256", "md5"]);
                assert_eq!(output, None);
            }
            _ => panic!("Expected Hash command"),
        }
    }
    
    #[test]
    fn test_parse_hash_command_with_output() {
        let args = vec!["hash-utility", "hash", "-f", "test.txt", "-a", "sha256", "-o", "output.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Hash { file, algorithms, output } => {
                assert_eq!(file, PathBuf::from("test.txt"));
                assert_eq!(algorithms, vec!["sha256"]);
                assert_eq!(output, Some(PathBuf::from("output.txt")));
            }
            _ => panic!("Expected Hash command"),
        }
    }
    
    #[test]
    fn test_parse_hash_command_long_flags() {
        let args = vec!["hash-utility", "hash", "--file", "test.txt", "--algorithm", "sha256"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Hash { file, algorithms, .. } => {
                assert_eq!(file, PathBuf::from("test.txt"));
                assert_eq!(algorithms, vec!["sha256"]);
            }
            _ => panic!("Expected Hash command"),
        }
    }
    
    #[test]
    fn test_parse_scan_command() {
        let args = vec!["hash-utility", "scan", "-d", "/path/to/dir", "-a", "sha256", "-o", "hashes.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Scan { directory, algorithm, output, parallel } => {
                assert_eq!(directory, PathBuf::from("/path/to/dir"));
                assert_eq!(algorithm, "sha256");
                assert_eq!(output, PathBuf::from("hashes.txt"));
                assert_eq!(parallel, false);
            }
            _ => panic!("Expected Scan command"),
        }
    }
    
    #[test]
    fn test_parse_scan_command_with_parallel() {
        let args = vec!["hash-utility", "scan", "-d", "/path/to/dir", "-a", "sha256", "-o", "hashes.txt", "-p"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Scan { directory, algorithm, output, parallel } => {
                assert_eq!(directory, PathBuf::from("/path/to/dir"));
                assert_eq!(algorithm, "sha256");
                assert_eq!(output, PathBuf::from("hashes.txt"));
                assert_eq!(parallel, true);
            }
            _ => panic!("Expected Scan command"),
        }
    }
    
    #[test]
    fn test_parse_scan_command_long_flags() {
        let args = vec!["hash-utility", "scan", "--directory", "/path/to/dir", "--algorithm", "sha256", "--output", "hashes.txt", "--parallel"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Scan { directory, algorithm, output, parallel } => {
                assert_eq!(directory, PathBuf::from("/path/to/dir"));
                assert_eq!(algorithm, "sha256");
                assert_eq!(output, PathBuf::from("hashes.txt"));
                assert_eq!(parallel, true);
            }
            _ => panic!("Expected Scan command"),
        }
    }
    
    #[test]
    fn test_parse_verify_command() {
        let args = vec!["hash-utility", "verify", "-b", "hashes.txt", "-d", "/path/to/dir"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Verify { database, directory } => {
                assert_eq!(database, PathBuf::from("hashes.txt"));
                assert_eq!(directory, PathBuf::from("/path/to/dir"));
            }
            _ => panic!("Expected Verify command"),
        }
    }
    
    #[test]
    fn test_parse_verify_command_long_flags() {
        let args = vec!["hash-utility", "verify", "--database", "hashes.txt", "--directory", "/path/to/dir"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Verify { database, directory } => {
                assert_eq!(database, PathBuf::from("hashes.txt"));
                assert_eq!(directory, PathBuf::from("/path/to/dir"));
            }
            _ => panic!("Expected Verify command"),
        }
    }
    
    #[test]
    fn test_parse_benchmark_command() {
        let args = vec!["hash-utility", "benchmark"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Benchmark { size_mb } => {
                assert_eq!(size_mb, 100); // default value
            }
            _ => panic!("Expected Benchmark command"),
        }
    }
    
    #[test]
    fn test_parse_benchmark_command_with_size() {
        let args = vec!["hash-utility", "benchmark", "-s", "50"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Benchmark { size_mb } => {
                assert_eq!(size_mb, 50);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }
    
    #[test]
    fn test_parse_benchmark_command_long_flag() {
        let args = vec!["hash-utility", "benchmark", "--size", "200"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Benchmark { size_mb } => {
                assert_eq!(size_mb, 200);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }
    
    #[test]
    fn test_parse_list_command() {
        let args = vec!["hash-utility", "list"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::List => {
                // Success - List command has no arguments
            }
            _ => panic!("Expected List command"),
        }
    }
    
    #[test]
    fn test_parse_invalid_command() {
        let args = vec!["hash-utility", "invalid-command"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_missing_required_argument() {
        // Hash command requires -f flag
        let args = vec!["hash-utility", "hash"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_scan_missing_output() {
        // Scan command requires -o flag
        let args = vec!["hash-utility", "scan", "-d", "/path/to/dir", "-a", "sha256"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_verify_missing_database() {
        // Verify command requires -b flag
        let args = vec!["hash-utility", "verify", "-d", "/path/to/dir"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_hash_command_default_algorithm() {
        let args = vec!["hash-utility", "hash", "-f", "test.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Hash { algorithms, .. } => {
                assert_eq!(algorithms, vec!["sha256"]); // default algorithm
            }
            _ => panic!("Expected Hash command"),
        }
    }
    
    #[test]
    fn test_scan_command_default_algorithm() {
        let args = vec!["hash-utility", "scan", "-d", "/path/to/dir", "-o", "hashes.txt"];
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Command::Scan { algorithm, .. } => {
                assert_eq!(algorithm, "sha256"); // default algorithm
            }
            _ => panic!("Expected Scan command"),
        }
    }
}
