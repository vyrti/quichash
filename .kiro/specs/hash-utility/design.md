# Design Document

## Overview

The Hash Utility is a cross-platform command-line application written in Rust that provides comprehensive cryptographic hashing capabilities with SIMD optimization. The application follows a modular architecture separating CLI parsing, hash computation, file I/O, and verification logic. It leverages the RustCrypto ecosystem for hash implementations and uses runtime CPU feature detection to enable SIMD acceleration automatically.

The design prioritizes minimal binary size through careful dependency selection, link-time optimization, and avoiding unnecessary features. The application uses streaming I/O to maintain constant memory usage regardless of file size and supports parallel processing for directory operations.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     CLI Interface                        │
│                    (clap minimal)                        │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│                  Command Dispatcher                      │
│         (hash, scan, verify, benchmark, list)           │
└─────┬──────────┬──────────┬──────────┬─────────────────┘
      │          │          │          │
      ▼          ▼          ▼          ▼
┌──────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐
│  Hash    │ │ Scan   │ │ Verify  │ │Benchmark │
│ Computer │ │ Engine │ │ Engine  │ │  Engine  │
└────┬─────┘ └───┬────┘ └────┬────┘ └────┬─────┘
     │           │           │           │
     └───────────┴───────────┴───────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│              Hash Algorithm Registry                     │
│    (MD5, SHA1, SHA2-*, SHA3-*, BLAKE2*, BLAKE3)        │
└────────────────┬────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────┐
│           File I/O & Path Handling                      │
│        (streaming, cross-platform paths)                │
└─────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

1. **CLI Interface** parses command-line arguments and validates input
2. **Command Dispatcher** routes to appropriate engine based on command
3. **Hash Computer** computes hashes for individual files using streaming I/O
4. **Scan Engine** recursively traverses directories and writes hash database
5. **Verify Engine** compares current hashes against stored database
6. **Benchmark Engine** measures throughput for all algorithms
7. **Hash Algorithm Registry** provides unified interface to all hash implementations
8. **File I/O** handles cross-platform file operations with buffering

## Components and Interfaces

### CLI Interface

**Responsibility:** Parse command-line arguments and display help information

**Dependencies:** clap (minimal features only)

**Interface:**
```rust
pub enum Command {
    Hash {
        file: PathBuf,
        algorithms: Vec<String>,
        output: Option<PathBuf>,
    },
    Scan {
        directory: PathBuf,
        algorithm: String,
        output: PathBuf,
        parallel: bool,
    },
    Verify {
        database: PathBuf,
        directory: PathBuf,
    },
    Benchmark {
        size_mb: Option<usize>,
    },
    List,
}

pub fn parse_args() -> Result<Command, CliError>;
```

### Hash Algorithm Registry

**Responsibility:** Provide unified interface to all hash implementations with SIMD support

**Dependencies:** 
- md-5
- sha1
- sha2 (for SHA-224, SHA-256, SHA-384, SHA-512)
- sha3 (for SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE)
- blake2
- blake3

**Interface:**
```rust
pub trait Hasher: Send {
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> Vec<u8>;
    fn output_size(&self) -> usize;
}

pub struct HashRegistry;

impl HashRegistry {
    pub fn get_hasher(algorithm: &str) -> Result<Box<dyn Hasher>, HashError>;
    pub fn list_algorithms() -> Vec<AlgorithmInfo>;
    pub fn is_post_quantum(algorithm: &str) -> bool;
}

pub struct AlgorithmInfo {
    pub name: String,
    pub output_bits: usize,
    pub post_quantum: bool,
}
```

### Hash Computer

**Responsibility:** Compute hash for a single file using streaming I/O

**Interface:**
```rust
pub struct HashComputer {
    buffer_size: usize,
}

impl HashComputer {
    pub fn compute_hash(
        &self,
        path: &Path,
        algorithm: &str,
    ) -> Result<HashResult, ComputeError>;
    
    pub fn compute_multiple_hashes(
        &self,
        path: &Path,
        algorithms: &[String],
    ) -> Result<Vec<HashResult>, ComputeError>;
}

pub struct HashResult {
    pub algorithm: String,
    pub hash: String,  // hex-encoded
    pub file_path: PathBuf,
}
```

### Scan Engine

**Responsibility:** Recursively scan directories and generate hash database

**Interface:**
```rust
pub struct ScanEngine {
    computer: HashComputer,
    parallel: bool,
}

impl ScanEngine {
    pub fn scan_directory(
        &self,
        root: &Path,
        algorithm: &str,
        output: &Path,
    ) -> Result<ScanStats, ScanError>;
}

pub struct ScanStats {
    pub files_processed: usize,
    pub files_failed: usize,
    pub total_bytes: u64,
    pub duration: Duration,
}
```

### Verify Engine

**Responsibility:** Compare current hashes against stored database

**Interface:**
```rust
pub struct VerifyEngine {
    computer: HashComputer,
}

impl VerifyEngine {
    pub fn verify(
        &self,
        database: &Path,
        directory: &Path,
    ) -> Result<VerifyReport, VerifyError>;
}

pub struct VerifyReport {
    pub matches: usize,
    pub mismatches: Vec<Mismatch>,
    pub missing_files: Vec<PathBuf>,
    pub new_files: Vec<PathBuf>,
}

pub struct Mismatch {
    pub path: PathBuf,
    pub expected: String,
    pub actual: String,
}
```

### Benchmark Engine

**Responsibility:** Measure throughput for all hash algorithms

**Interface:**
```rust
pub struct BenchmarkEngine;

impl BenchmarkEngine {
    pub fn run_benchmarks(
        &self,
        data_size_mb: usize,
    ) -> Result<Vec<BenchmarkResult>, BenchmarkError>;
}

pub struct BenchmarkResult {
    pub algorithm: String,
    pub throughput_mbps: f64,
    pub simd_enabled: bool,
}
```

### Database Format Handler

**Responsibility:** Read and write plain text hash database files

**Interface:**
```rust
pub struct DatabaseHandler;

impl DatabaseHandler {
    pub fn write_entry(
        writer: &mut impl Write,
        hash: &str,
        path: &Path,
    ) -> Result<(), IoError>;
    
    pub fn read_database(
        path: &Path,
    ) -> Result<HashMap<PathBuf, String>, IoError>;
}
```

## Data Models

### Hash Database File Format

Plain text format with one entry per line:
```
<hash_hex> <relative_or_absolute_path>
```

Example:
```
d41d8cd98f00b204e9800998ecf8427e  ./empty.txt
5d41402abc4b2a76b9719d911017c592  ./hello.txt
098f6bcd4621d373cade4e832627b4f6  ./test/data.bin
```

### Internal Data Structures

```rust
// Parsed command-line arguments
pub struct Config {
    pub command: Command,
    pub verbose: bool,
}

// Hash database entry
pub struct DatabaseEntry {
    pub hash: String,
    pub path: PathBuf,
}

// Progress tracking for scans
pub struct Progress {
    pub files_done: AtomicUsize,
    pub bytes_done: AtomicU64,
    pub start_time: Instant,
}
```

## Correc
tness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Hash output format validity

*For any* file and supported hash algorithm, computing the hash should produce output in the format `<hash_hex> <filepath>` where hash_hex is valid hexadecimal of the correct length for that algorithm.

**Validates: Requirements 1.1, 9.1**

### Property 2: Complete directory traversal

*For any* directory structure, scanning should produce hash entries for all regular files in the directory tree, with no files omitted or duplicated.

**Validates: Requirements 2.1**

### Property 3: Database format round-trip

*For any* set of hash results, writing them to a database file and then reading that file back should produce equivalent hash entries.

**Validates: Requirements 2.2, 9.5**

### Property 4: Verification correctness

*For any* hash database and directory, verification should correctly identify all matches (files with unchanged hashes), mismatches (files with changed hashes), deletions (files in database but not filesystem), and additions (files in filesystem but not database).

**Validates: Requirements 3.1, 3.2, 3.5**

### Property 5: Path separator handling

*For any* valid file path with platform-specific or mixed separators, the system should correctly resolve and access the file.

**Validates: Requirements 4.2**

### Property 6: Invalid argument handling

*For any* invalid command-line argument combination, the system should produce an error message without crashing.

**Validates: Requirements 6.3**

### Property 7: Flag equivalence

*For any* command with flags, using short form (-h) or long form (--help) should produce identical behavior.

**Validates: Requirements 6.4**

### Property 8: Post-quantum algorithm labeling

*For any* algorithm in the supported list, if it is SHA-3 family, it should be labeled as post-quantum resistant.

**Validates: Requirements 7.4**

### Property 9: Parallel processing equivalence

*For any* directory, scanning with parallel processing enabled should produce the same set of hash results as sequential processing (order may differ).

**Validates: Requirements 8.4**

### Property 10: File output isolation

*For any* hash computation with file output specified, the results should be written to the file and nothing should appear on stdout.

**Validates: Requirements 9.4**

## Error Handling

### Error Categories

1. **File System Errors**
   - File not found
   - Permission denied
   - I/O errors during read
   - Path too long or invalid characters

2. **Parse Errors**
   - Invalid hash database format
   - Malformed command-line arguments
   - Invalid algorithm name

3. **Verification Errors**
   - Database file not found
   - Directory not found
   - Hash mismatch (not an error, but a finding)

### Error Handling Strategy

- All errors use Result types with descriptive error enums
- File system errors during directory scans are logged but don't stop the scan
- Invalid arguments produce helpful error messages with usage examples
- Verification mismatches are reported as findings, not errors
- All errors include context (file path, operation, etc.)

### Error Types

```rust
#[derive(Debug)]
pub enum HashError {
    UnsupportedAlgorithm(String),
    IoError(io::Error),
    ParseError(String),
    VerificationError(String),
}

impl Display for HashError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HashError::UnsupportedAlgorithm(alg) => 
                write!(f, "Unsupported algorithm: {}. Use --list to see available algorithms.", alg),
            HashError::IoError(e) => 
                write!(f, "I/O error: {}", e),
            HashError::ParseError(msg) => 
                write!(f, "Parse error: {}", msg),
            HashError::VerificationError(msg) => 
                write!(f, "Verification error: {}", msg),
        }
    }
}
```

## Testing Strategy

### Unit Testing

Unit tests will cover:
- CLI argument parsing with various flag combinations
- Database file format parsing with edge cases (empty files, malformed lines)
- Path normalization across platforms
- Error message formatting
- Algorithm registry lookup

### Property-Based Testing

The system will use **proptest** as the property-based testing library for Rust. Each property-based test will be configured to run a minimum of 100 iterations.

Property-based tests will verify:

1. **Hash Output Format** - Generate random files and algorithms, verify output format
   - Tag: `Feature: hash-utility, Property 1: Hash output format validity`
   
2. **Complete Directory Traversal** - Generate random directory structures, verify all files are hashed
   - Tag: `Feature: hash-utility, Property 2: Complete directory traversal`
   
3. **Database Format Round-Trip** - Generate random hash results, write and read back
   - Tag: `Feature: hash-utility, Property 3: Database format round-trip`
   
4. **Verification Correctness** - Generate random databases and directories, verify classification
   - Tag: `Feature: hash-utility, Property 4: Verification correctness`
   
5. **Path Separator Handling** - Generate paths with various separators, verify access
   - Tag: `Feature: hash-utility, Property 5: Path separator handling`
   
6. **Invalid Argument Handling** - Generate invalid argument combinations, verify error handling
   - Tag: `Feature: hash-utility, Property 6: Invalid argument handling`
   
7. **Flag Equivalence** - Generate commands with short/long flags, verify equivalence
   - Tag: `Feature: hash-utility, Property 7: Flag equivalence`
   
8. **Post-Quantum Algorithm Labeling** - Verify SHA-3 algorithms are labeled correctly
   - Tag: `Feature: hash-utility, Property 8: Post-quantum algorithm labeling`
   
9. **Parallel Processing Equivalence** - Generate directories, compare parallel vs sequential results
   - Tag: `Feature: hash-utility, Property 9: Parallel processing equivalence`
   
10. **File Output Isolation** - Generate hash operations with file output, verify stdout is empty
    - Tag: `Feature: hash-utility, Property 10: File output isolation`

### Integration Testing

Integration tests will verify:
- End-to-end workflow: scan directory → verify against database
- Benchmark command produces output for all algorithms
- Help and list commands display expected information
- Cross-platform path handling in real filesystem operations

### Performance Testing

Performance tests will:
- Verify SIMD-enabled builds are faster than scalar builds (when available)
- Ensure memory usage remains constant for large files
- Validate parallel processing improves throughput for multiple files

## Implementation Details

### SIMD Optimization Strategy

The hash algorithm crates from RustCrypto automatically enable SIMD optimizations through Rust's target feature detection:

1. **Compile-time features**: Enable CPU features via RUSTFLAGS
   ```
   RUSTFLAGS="-C target-cpu=native"
   ```

2. **Runtime detection**: Crates like `blake3` use runtime CPU feature detection automatically

3. **Fallback**: All crates provide scalar implementations as fallback

### Dependency Selection for Minimal Binary Size

**Core dependencies:**
- `clap` (minimal features: `derive`, `std`) - CLI parsing
- `md-5` - MD5 hashing
- `sha1` - SHA-1 hashing
- `sha2` - SHA-2 family (SHA-224, SHA-256, SHA-384, SHA-512)
- `sha3` - SHA-3 family and SHAKE
- `blake2` - BLAKE2b and BLAKE2s
- `blake3` - BLAKE3 with SIMD support

**Build optimizations:**
```toml
[profile.release]
opt-level = "z"          # Optimize for size
lto = true               # Link-time optimization
codegen-units = 1        # Better optimization
strip = true             # Strip symbols
panic = "abort"          # Smaller panic handler
```

### Streaming I/O Implementation

Files are processed in 64KB chunks to maintain constant memory usage:

```rust
const BUFFER_SIZE: usize = 64 * 1024;

pub fn hash_file<H: Hasher>(path: &Path, mut hasher: H) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    
    Ok(hasher.finalize())
}
```

### Parallel Directory Scanning

Use `rayon` for parallel directory traversal (optional dependency, feature-gated):

```rust
use rayon::prelude::*;

pub fn scan_parallel(root: &Path, algorithm: &str) -> Result<Vec<HashResult>> {
    let files: Vec<PathBuf> = collect_files(root)?;
    
    files.par_iter()
        .map(|path| compute_hash(path, algorithm))
        .collect()
}
```

### Cross-Platform Path Handling

Use `std::path::Path` and `PathBuf` for all path operations:
- Automatically handles platform-specific separators
- Use `canonicalize()` for absolute paths
- Use `strip_prefix()` for relative paths in database
- Handle both forward and backward slashes in database parsing

### Database File Format Details

**Writing:**
```rust
fn write_entry(writer: &mut impl Write, hash: &str, path: &Path) -> io::Result<()> {
    writeln!(writer, "{}  {}", hash, path.display())
}
```

**Reading:**
```rust
fn parse_line(line: &str) -> Option<(String, PathBuf)> {
    let parts: Vec<&str> = line.splitn(2, "  ").collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), PathBuf::from(parts[1])))
    } else {
        None
    }
}
```

Note: Two spaces separate hash from path (compatible with `sha256sum` format)

## Security Considerations

1. **Path Traversal**: Validate that paths don't escape intended directories
2. **Symbolic Links**: Decide whether to follow symlinks (default: follow)
3. **Large Files**: Streaming prevents memory exhaustion attacks
4. **Algorithm Selection**: Warn users about deprecated algorithms (MD5, SHA-1)
5. **Timing Attacks**: Not applicable for file hashing use case

## Performance Characteristics

### Expected Throughput (with SIMD on modern CPU)

- BLAKE3: ~3-5 GB/s (fastest)
- SHA-256: ~500-800 MB/s
- SHA-512: ~600-900 MB/s
- SHA3-256: ~200-400 MB/s
- MD5: ~400-600 MB/s

### Memory Usage

- Constant: ~64KB buffer per file being processed
- Parallel mode: ~64KB × number of threads
- Database: O(n) where n is number of files

### Disk I/O

- Sequential reads for file hashing
- Buffered writes for database output
- Minimal seeks during directory traversal

## Future Enhancements

1. **Progress Bar**: Add visual progress indicator for long operations
2. **Incremental Updates**: Only rehash files with changed timestamps
3. **Compression**: Optional compression for large database files
4. **Watch Mode**: Monitor directories for changes
5. **Network Hashing**: Hash files over network protocols
6. **Custom Algorithms**: Plugin system for additional hash functions
7. **Parallel Verification**: Parallelize verification operations
8. **JSON Output**: Optional structured output format for automation
