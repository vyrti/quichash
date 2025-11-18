// Hash computation module
// Provides hash algorithm registry and computation logic

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::error::HashUtilityError;

/// Trait for hash algorithm implementations
pub trait Hasher: Send {
    /// Update the hasher with new data
    fn update(&mut self, data: &[u8]);
    
    /// Finalize the hash and return the result
    fn finalize(self: Box<Self>) -> Vec<u8>;
    
    /// Get the output size in bytes
    fn output_size(&self) -> usize;
}

/// Information about a hash algorithm
#[derive(Debug, Clone)]
pub struct AlgorithmInfo {
    pub name: String,
    pub output_bits: usize,
    pub post_quantum: bool,
}

// Re-export HashUtilityError as HashError for backward compatibility
pub type HashError = HashUtilityError;

// Wrapper types for hash algorithms
use md5::{Md5, Digest as Md5Digest};
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest as Sha2Digest};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512, Digest as Sha3Digest};
use blake2::{Blake2b512, Blake2s256, Digest as Blake2Digest};
use blake3::Hasher as Blake3Hasher;

// MD5 wrapper
pub struct Md5Wrapper(Md5);

impl Hasher for Md5Wrapper {
    fn update(&mut self, data: &[u8]) {
        Md5Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Md5Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        16 // 128 bits
    }
}

// SHA1 wrapper
pub struct Sha1Wrapper(Sha1);

impl Hasher for Sha1Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha1Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha1Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        20 // 160 bits
    }
}

// SHA-224 wrapper
pub struct Sha224Wrapper(Sha224);

impl Hasher for Sha224Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        28 // 224 bits
    }
}

// SHA-256 wrapper
pub struct Sha256Wrapper(Sha256);

impl Hasher for Sha256Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        32 // 256 bits
    }
}

// SHA-384 wrapper
pub struct Sha384Wrapper(Sha384);

impl Hasher for Sha384Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        48 // 384 bits
    }
}

// SHA-512 wrapper
pub struct Sha512Wrapper(Sha512);

impl Hasher for Sha512Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        64 // 512 bits
    }
}

// SHA3-224 wrapper
pub struct Sha3_224Wrapper(Sha3_224);

impl Hasher for Sha3_224Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha3Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        28 // 224 bits
    }
}

// SHA3-256 wrapper
pub struct Sha3_256Wrapper(Sha3_256);

impl Hasher for Sha3_256Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha3Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        32 // 256 bits
    }
}

// SHA3-384 wrapper
pub struct Sha3_384Wrapper(Sha3_384);

impl Hasher for Sha3_384Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha3Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        48 // 384 bits
    }
}

// SHA3-512 wrapper
pub struct Sha3_512Wrapper(Sha3_512);

impl Hasher for Sha3_512Wrapper {
    fn update(&mut self, data: &[u8]) {
        Sha3Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Sha3Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        64 // 512 bits
    }
}

// BLAKE2b wrapper
pub struct Blake2b512Wrapper(Blake2b512);

impl Hasher for Blake2b512Wrapper {
    fn update(&mut self, data: &[u8]) {
        Blake2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Blake2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        64 // 512 bits
    }
}

// BLAKE2s wrapper
pub struct Blake2s256Wrapper(Blake2s256);

impl Hasher for Blake2s256Wrapper {
    fn update(&mut self, data: &[u8]) {
        Blake2Digest::update(&mut self.0, data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        Blake2Digest::finalize(self.0).to_vec()
    }
    
    fn output_size(&self) -> usize {
        32 // 256 bits
    }
}

// BLAKE3 wrapper
pub struct Blake3Wrapper(Blake3Hasher);

impl Hasher for Blake3Wrapper {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.0.finalize().as_bytes().to_vec()
    }
    
    fn output_size(&self) -> usize {
        32 // 256 bits
    }
}

/// Registry for hash algorithms
pub struct HashRegistry;

impl HashRegistry {
    /// Get a hasher instance for the specified algorithm
    pub fn get_hasher(algorithm: &str) -> Result<Box<dyn Hasher>, HashError> {
        let alg_lower = algorithm.to_lowercase();
        
        match alg_lower.as_str() {
            "md5" => Ok(Box::new(Md5Wrapper(Md5Digest::new()))),
            "sha1" => Ok(Box::new(Sha1Wrapper(Sha1Digest::new()))),
            "sha224" | "sha-224" => Ok(Box::new(Sha224Wrapper(Sha2Digest::new()))),
            "sha256" | "sha-256" => Ok(Box::new(Sha256Wrapper(Sha2Digest::new()))),
            "sha384" | "sha-384" => Ok(Box::new(Sha384Wrapper(Sha2Digest::new()))),
            "sha512" | "sha-512" => Ok(Box::new(Sha512Wrapper(Sha2Digest::new()))),
            "sha3-224" => Ok(Box::new(Sha3_224Wrapper(Sha3Digest::new()))),
            "sha3-256" => Ok(Box::new(Sha3_256Wrapper(Sha3Digest::new()))),
            "sha3-384" => Ok(Box::new(Sha3_384Wrapper(Sha3Digest::new()))),
            "sha3-512" => Ok(Box::new(Sha3_512Wrapper(Sha3Digest::new()))),
            "blake2b" | "blake2b-512" => Ok(Box::new(Blake2b512Wrapper(Blake2Digest::new()))),
            "blake2s" | "blake2s-256" => Ok(Box::new(Blake2s256Wrapper(Blake2Digest::new()))),
            "blake3" => Ok(Box::new(Blake3Wrapper(Blake3Hasher::new()))),
            _ => Err(HashUtilityError::UnsupportedAlgorithm {
                algorithm: algorithm.to_string(),
            }),
        }
    }
    
    /// List all available hash algorithms
    pub fn list_algorithms() -> Vec<AlgorithmInfo> {
        vec![
            AlgorithmInfo {
                name: "MD5".to_string(),
                output_bits: 128,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA1".to_string(),
                output_bits: 160,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA-224".to_string(),
                output_bits: 224,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA-256".to_string(),
                output_bits: 256,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA-384".to_string(),
                output_bits: 384,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA-512".to_string(),
                output_bits: 512,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "SHA3-224".to_string(),
                output_bits: 224,
                post_quantum: true,
            },
            AlgorithmInfo {
                name: "SHA3-256".to_string(),
                output_bits: 256,
                post_quantum: true,
            },
            AlgorithmInfo {
                name: "SHA3-384".to_string(),
                output_bits: 384,
                post_quantum: true,
            },
            AlgorithmInfo {
                name: "SHA3-512".to_string(),
                output_bits: 512,
                post_quantum: true,
            },
            AlgorithmInfo {
                name: "BLAKE2b-512".to_string(),
                output_bits: 512,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "BLAKE2s-256".to_string(),
                output_bits: 256,
                post_quantum: false,
            },
            AlgorithmInfo {
                name: "BLAKE3".to_string(),
                output_bits: 256,
                post_quantum: false,
            },
        ]
    }
    
    /// Check if an algorithm is post-quantum resistant
    pub fn is_post_quantum(algorithm: &str) -> bool {
        let alg_lower = algorithm.to_lowercase();
        
        // SHA-3 family algorithms are considered post-quantum resistant
        alg_lower.starts_with("sha3-") || 
        alg_lower == "shake128" || 
        alg_lower == "shake256"
    }
}

/// Result of a hash computation
#[derive(Debug, Clone)]
pub struct HashResult {
    pub algorithm: String,
    pub hash: String,  // hex-encoded
    pub file_path: PathBuf,
}

/// Hash computer with streaming I/O
pub struct HashComputer {
    buffer_size: usize,
}

impl HashComputer {
    /// Create a new HashComputer with default buffer size (64KB)
    pub fn new() -> Self {
        Self {
            buffer_size: 64 * 1024,
        }
    }
    
    /// Create a new HashComputer with custom buffer size
    pub fn with_buffer_size(buffer_size: usize) -> Self {
        Self { buffer_size }
    }
    
    /// Compute hash for a single file using streaming I/O
    pub fn compute_hash(
        &self,
        path: &Path,
        algorithm: &str,
    ) -> Result<HashResult, HashError> {
        // Get hasher for the specified algorithm
        let mut hasher = HashRegistry::get_hasher(algorithm)?;
        
        // Open file for reading with better error context
        let mut file = File::open(path).map_err(|e| {
            HashUtilityError::from_io_error(e, "reading", Some(path.to_path_buf()))
        })?;
        
        // Create buffer for streaming reads
        let mut buffer = vec![0u8; self.buffer_size];
        
        // Stream file data through hasher
        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| {
                HashUtilityError::from_io_error(e, "reading", Some(path.to_path_buf()))
            })?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        // Finalize hash and convert to hex
        let hash_bytes = hasher.finalize();
        let hash_hex = bytes_to_hex(&hash_bytes);
        
        Ok(HashResult {
            algorithm: algorithm.to_string(),
            hash: hash_hex,
            file_path: path.to_path_buf(),
        })
    }
    
    /// Compute multiple hashes for a single file in a single pass
    pub fn compute_multiple_hashes(
        &self,
        path: &Path,
        algorithms: &[String],
    ) -> Result<Vec<HashResult>, HashError> {
        // Get hashers for all specified algorithms
        let mut hashers: Vec<(String, Box<dyn Hasher>)> = Vec::new();
        for algorithm in algorithms {
            let hasher = HashRegistry::get_hasher(algorithm)?;
            hashers.push((algorithm.clone(), hasher));
        }
        
        // Open file for reading with better error context
        let mut file = File::open(path).map_err(|e| {
            HashUtilityError::from_io_error(e, "reading", Some(path.to_path_buf()))
        })?;
        
        // Create buffer for streaming reads
        let mut buffer = vec![0u8; self.buffer_size];
        
        // Stream file data through all hashers in single pass
        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| {
                HashUtilityError::from_io_error(e, "reading", Some(path.to_path_buf()))
            })?;
            if bytes_read == 0 {
                break;
            }
            
            // Update all hashers with the same data
            for (_, hasher) in &mut hashers {
                hasher.update(&buffer[..bytes_read]);
            }
        }
        
        // Finalize all hashes and collect results
        let mut results = Vec::new();
        for (algorithm, hasher) in hashers {
            let hash_bytes = hasher.finalize();
            let hash_hex = bytes_to_hex(&hash_bytes);
            
            results.push(HashResult {
                algorithm,
                hash: hash_hex,
                file_path: path.to_path_buf(),
            });
        }
        
        Ok(results)
    }
}

impl Default for HashComputer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert bytes to hexadecimal string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_compute_hash_sha256() {
        // Create a temporary test file
        let test_data = b"hello world";
        let temp_file = "test_hash_temp.txt";
        fs::write(temp_file, test_data).unwrap();
        
        // Compute hash
        let computer = HashComputer::new();
        let result = computer.compute_hash(Path::new(temp_file), "sha256").unwrap();
        
        // Verify result
        assert_eq!(result.algorithm, "sha256");
        assert_eq!(result.hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(result.file_path, Path::new(temp_file));
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_compute_multiple_hashes() {
        // Create a temporary test file
        let test_data = b"test data";
        let temp_file = "test_multi_hash_temp.txt";
        fs::write(temp_file, test_data).unwrap();
        
        // Compute multiple hashes
        let computer = HashComputer::new();
        let algorithms = vec!["md5".to_string(), "sha256".to_string()];
        let results = computer.compute_multiple_hashes(Path::new(temp_file), &algorithms).unwrap();
        
        // Verify results
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].algorithm, "md5");
        assert_eq!(results[1].algorithm, "sha256");
        
        // Both should have the same file path
        assert_eq!(results[0].file_path, Path::new(temp_file));
        assert_eq!(results[1].file_path, Path::new(temp_file));
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_streaming_large_file() {
        // Create a file larger than buffer size (64KB)
        let temp_file = "test_large_temp.txt";
        let mut file = fs::File::create(temp_file).unwrap();
        let chunk = vec![b'a'; 1024];
        for _ in 0..100 {  // 100KB file
            file.write_all(&chunk).unwrap();
        }
        drop(file);
        
        // Compute hash with streaming
        let computer = HashComputer::new();
        let result = computer.compute_hash(Path::new(temp_file), "sha256").unwrap();
        
        // Verify hash is computed (not checking exact value, just that it works)
        assert_eq!(result.hash.len(), 64);  // SHA-256 produces 64 hex characters
        
        // Cleanup
        fs::remove_file(temp_file).unwrap();
    }
    
    #[test]
    fn test_file_not_found_error() {
        let computer = HashComputer::new();
        let result = computer.compute_hash(Path::new("nonexistent_file.txt"), "sha256");
        
        assert!(result.is_err());
        match result {
            Err(HashUtilityError::FileNotFound { .. }) => {},
            Err(HashUtilityError::IoError { .. }) => {},
            _ => panic!("Expected FileNotFound or IoError"),
        }
    }
    
    #[test]
    fn test_unsupported_algorithm_error() {
        let temp_file = "test_unsupported_temp.txt";
        fs::write(temp_file, b"test").unwrap();
        
        let computer = HashComputer::new();
        let result = computer.compute_hash(Path::new(temp_file), "invalid_algorithm");
        
        assert!(result.is_err());
        match result {
            Err(HashUtilityError::UnsupportedAlgorithm { .. }) => {},
            _ => panic!("Expected UnsupportedAlgorithm error"),
        }
        
        fs::remove_file(temp_file).unwrap();
    }
}
