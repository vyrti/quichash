# Implementation Plan

- [x] 1. Set up project structure and dependencies
  - Create new Rust project with `cargo init`
  - Configure Cargo.toml with minimal dependencies: clap, md-5, sha1, sha2, sha3, blake2, blake3
  - Set up release profile with size optimizations (opt-level="z", lto=true, strip=true, panic="abort")
  - Create module structure: cli, hash, scan, verify, benchmark, database
  - _Requirements: 5.2, 5.3, 5.4, 5.5_

- [x] 2. Implement hash algorithm registry
  - [x] 2.1 Create Hasher trait and HashRegistry
    - Define Hasher trait with update, finalize, and output_size methods
    - Implement HashRegistry with get_hasher, list_algorithms, and is_post_quantum functions
    - Create AlgorithmInfo struct with name, output_bits, and post_quantum fields
    - _Requirements: 1.2, 7.1, 7.2_
  
  - [ ]* 2.2 Write property test for post-quantum labeling
    - **Property 8: Post-quantum algorithm labeling**
    - **Validates: Requirements 7.4**
  
  - [x] 2.3 Implement wrapper types for each hash algorithm
    - Create wrapper structs for MD5, SHA1, SHA2 variants, SHA3 variants, BLAKE2, BLAKE3
    - Implement Hasher trait for each wrapper
    - Register all algorithms in HashRegistry
    - _Requirements: 1.2, 7.1, 7.2_

- [x] 3. Implement core hash computation
  - [x] 3.1 Create HashComputer with streaming I/O
    - Implement compute_hash function with 64KB buffered reads
    - Implement compute_multiple_hashes for multiple algorithms in single pass
    - Create HashResult struct with algorithm, hash (hex-encoded), and file_path
    - Handle file I/O errors gracefully
    - _Requirements: 1.1, 1.4, 1.5_
  
  - [ ]* 3.2 Write property test for hash output format
    - **Property 1: Hash output format validity**
    - **Validates: Requirements 1.1, 9.1**

- [x] 4. Implement database format handler
  - [x] 4.1 Create DatabaseHandler for reading and writing
    - Implement write_entry function with format: `<hash>  <filepath>` (two spaces)
    - Implement read_database function to parse database files into HashMap
    - Handle malformed lines gracefully (skip with warning)
    - _Requirements: 2.2, 9.1, 9.5_
  
  - [ ]* 4.2 Write property test for database round-trip
    - **Property 3: Database format round-trip**
    - **Validates: Requirements 2.2, 9.5**

- [x] 5. Implement directory scanning engine
  - [x] 5.1 Create ScanEngine with recursive traversal
    - Implement scan_directory function to recursively find all files
    - Use HashComputer to compute hash for each file
    - Write results to output file using DatabaseHandler
    - Track statistics: files_processed, files_failed, total_bytes, duration
    - Handle permission errors without stopping scan
    - Display progress information during scan
    - _Requirements: 2.1, 2.2, 2.4, 2.5_
  
  - [ ]* 5.2 Write property test for complete directory traversal
    - **Property 2: Complete directory traversal**
    - **Validates: Requirements 2.1**

- [x] 6. Implement verification engine
  - [x] 6.1 Create VerifyEngine for hash comparison
    - Implement verify function to load database and compare with current hashes
    - Classify files as: matches, mismatches, missing (deleted), new (added)
    - Create VerifyReport with all findings
    - Create Mismatch struct with path, expected, and actual hashes
    - Display detailed report with all categories
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ]* 6.2 Write property test for verification correctness
    - **Property 4: Verification correctness**
    - **Validates: Requirements 3.1, 3.2, 3.5**

- [x] 7. Implement benchmark engine
  - [x] 7.1 Create BenchmarkEngine for performance testing
    - Generate test data of configurable size (default 100MB)
    - Run each algorithm and measure throughput
    - Create BenchmarkResult with algorithm name and throughput_mbps
    - Display results in formatted table
    - _Requirements: 8.5_

- [x] 8. Implement CLI interface
  - [x] 8.1 Define command structure with clap
    - Create Command enum: Hash, Scan, Verify, Benchmark, List
    - Define arguments for each command with short and long flags
    - Implement parse_args function
    - Add help text and examples
    - _Requirements: 6.1, 6.2, 6.4_
  
  - [ ]* 8.2 Write property test for flag equivalence
    - **Property 7: Flag equivalence**
    - **Validates: Requirements 6.4**
  
  - [ ]* 8.3 Write property test for invalid argument handling
    - **Property 6: Invalid argument handling**
    - **Validates: Requirements 6.3**

- [x] 9. Implement command dispatcher and main function
  - [x] 9.1 Create command dispatcher
    - Route parsed commands to appropriate engines
    - Handle Hash command: compute and display hash(es)
    - Handle Scan command: scan directory and write database
    - Handle Verify command: compare database with directory
    - Handle Benchmark command: run performance tests
    - Handle List command: display available algorithms
    - _Requirements: 1.1, 2.1, 3.1, 8.5, 1.2_
  
  - [x] 9.2 Implement output handling
    - Support stdout output by default
    - Support file output with -o/--output flag
    - Ensure file output doesn't appear on stdout
    - _Requirements: 9.3, 9.4_
  
  - [ ]* 9.3 Write property test for file output isolation
    - **Property 10: File output isolation**
    - **Validates: Requirements 9.4**

- [x] 10. Add cross-platform path handling
  - [x] 10.1 Implement path normalization utilities
    - Use std::path::Path for all path operations
    - Handle both forward and backward slashes in database parsing
    - Use canonicalize for absolute paths
    - Use strip_prefix for relative paths in database
    - _Requirements: 4.2_
  
  - [ ]* 10.2 Write property test for path separator handling
    - **Property 5: Path separator handling**
    - **Validates: Requirements 4.2**

- [x] 11. Add parallel processing support
  - [x] 11.1 Add rayon dependency
    - Add rayon to Cargo.toml
    - _Requirements: 8.4_
  
  - [x] 11.2 Implement parallel scan mode
    - Use rayon to parallelize file hashing
    - Ensure thread-safe progress tracking
    - _Requirements: 8.4_
  
  - [ ]* 11.3 Write property test for parallel processing equivalence
    - **Property 9: Parallel processing equivalence**
    - **Validates: Requirements 8.4**

- [x] 12. Implement error handling
  - [x] 12.1 Define error types
    - Create HashError enum with variants for all error categories
    - Implement Display trait for user-friendly error messages
    - Add context to errors (file paths, operations)
    - _Requirements: 6.3_
  
  - [x] 12.2 Add error handling throughout codebase
    - Use Result types consistently
    - Provide helpful error messages with suggestions
    - Log errors during directory scans without stopping
    - _Requirements: 2.4, 6.3_

- [x] 13. Add SIMD optimization verification
  - [x] 13.1 Verify SIMD support in hash crates
    - Test that ALL hashes uses SIMD when available
    - Verify fallback to scalar implementations works
    - Document RUSTFLAGS for optimal compilation
    - _Requirements: 1.3, 4.4, 4.5_

- [x] 14. Implement fast mode for large file hashing
  - [x] 14.1 Add fast mode flag to CLI
    - Add `--fast` or `-F` flag to scan command
    - Update help text to explain fast mode behavior
    - _Requirements: 1.1, 2.1_
  
  - [x] 14.2 Implement fast hash computation strategy
    - Create FastHashComputer that samples three regions of file
    - Read first 100MB of file
    - Read last 100MB of file
    - Read middle 100MB of file (calculated as: file_size/2 - 50MB to file_size/2 + 50MB)
    - Concatenate the three samples and compute hash on combined data
    - Ensure deterministic middle calculation so same file always produces same hash
    - Handle files smaller than 300MB gracefully (use full file)
    - _Requirements: 1.1, 1.4, 1.5_
  
  - [ ]* 14.3 Write property test for fast mode consistency
    - **Property: Fast mode determinism**
    - Verify that hashing the same file twice with fast mode produces identical results
    - Verify that middle region calculation is consistent across multiple runs
    - _Requirements: 1.1_
  
  - [ ]* 14.4 Write property test for fast mode correctness
    - **Property: Fast mode sampling correctness**
    - Verify that fast mode correctly samples first, middle, and last regions
    - Verify that files smaller than 300MB are handled correctly
    - _Requirements: 1.4, 1.5_

- [x] 15. Add progress bar reporting
  - [x] 15.1 Add indicatif dependency
    - Add indicatif crate to Cargo.toml for progress bar support
    - _Requirements: 2.1, 2.5_
  
  - [x] 15.2 Implement progress bar for scan operations
    - Create progress bar showing files processed, current file, and percentage
    - Display throughput (MB/s) and estimated time remaining
    - Update progress bar during directory scanning
    - Clear progress bar on completion and show summary
    - _Requirements: 2.5_
  
  - [x] 15.3 Implement progress bar for verify operations
    - Show progress during verification with files checked count
    - Display current file being verified
    - _Requirements: 3.1_

- [x] 16. Add stdin hash support
  - [x] 16.1 Implement stdin reading in hash command
    - Detect when no file is provided and stdin is available
    - Read from stdin with buffered I/O
    - Support piping: `cat file.txt | hash -a sha256`
    - _Requirements: 1.1_
  
  - [ ]* 16.2 Write property test for stdin hashing
    - **Property: Stdin hash equivalence**
    - Verify that hashing via stdin produces same result as hashing the file directly
    - _Requirements: 1.1_

- [x] 17. Add command-line text hashing
  - [x] 17.1 Add --text flag to hash command
    - Add `-t/--text` flag that accepts a string argument
    - Hash the provided text directly: `hash -a sha256 --text "hello world"`
    - Handle UTF-8 encoding properly
    - _Requirements: 1.1_
  
  - [ ]* 17.2 Write property test for text hashing
    - **Property: Text hash consistency**
    - Verify that hashing text produces consistent results
    - _Requirements: 1.1_

- [ ] 18. Add HMAC support
  - [ ] 18.1 Add HMAC dependencies
    - Add hmac crate to Cargo.toml
    - Add support for HMAC variants of existing algorithms
    - _Requirements: 1.1_
  
  - [ ] 18.2 Implement HMAC computation
    - Add `--key` flag to hash command for keyed hashing
    - Implement HMAC wrappers for SHA256, SHA512, SHA3, BLAKE2, BLAKE3
    - Support usage: `hash -a sha256 --key "secret" -f file.txt`
    - Register HMAC algorithms in HashRegistry with "hmac-" prefix
    - _Requirements: 1.1_
  
  - [ ]* 18.3 Write property test for HMAC correctness
    - **Property: HMAC key sensitivity**
    - Verify that different keys produce different HMACs for same input
    - Verify that same key produces same HMAC for same input
    - _Requirements: 1.1_

- [x] 19. Add non-cryptographic hash algorithms
  - [x] 19.1 Add xxhash dependency
    - Add xxhash-rust crate to Cargo.toml
    - Research and select fastest non-cryptographic hash (xxh3, xxh128)
    - _Requirements: 1.2_
  
  - [x] 19.2 Implement non-cryptographic hash wrappers
    - Create wrappers for xxh3 and xxh128
    - Register in HashRegistry with appropriate metadata
    - Mark as non-cryptographic in algorithm info
    - _Requirements: 1.2_
  
  - [ ]* 19.3 Write property test for non-cryptographic hash performance
    - **Property: Non-cryptographic hash speed**
    - Verify that xxh3 is significantly faster than cryptographic hashes
    - _Requirements: 1.2_

- [ ] 20. Add Base64 output format
  - [ ] 20.1 Add base64 dependency
    - Add base64 crate to Cargo.toml
    - _Requirements: 9.1_
  
  - [ ] 20.2 Implement Base64 output option
    - Add `--base64` flag to hash, scan, and verify commands
    - Convert hash output from hex to Base64 when flag is set
    - Update DatabaseHandler to support both hex and Base64 formats
    - Auto-detect format when reading database files
    - _Requirements: 9.1_
  
  - [ ]* 20.3 Write property test for Base64 encoding
    - **Property: Base64 round-trip**
    - Verify that hex to Base64 conversion is reversible
    - Verify that Base64 hashes can be verified correctly
    - _Requirements: 9.1_

- [x] 21. Add .hashignore file support
  - [x] 21.1 Add ignore dependency
    - Add ignore crate to Cargo.toml (same crate used by ripgrep)
    - _Requirements: 2.1_
  
  - [x] 21.2 Implement .hashignore parsing and filtering
    - Create IgnoreHandler that reads .hashignore files
    - Support gitignore-style patterns (globs, negation, comments)
    - Integrate with ScanEngine to skip ignored files
    - Search for .hashignore in scanned directory and parent directories
    - _Requirements: 2.1, 2.4_
  
  - [ ]* 21.3 Write property test for ignore pattern matching
    - **Property: Ignore pattern correctness**
    - Verify that files matching ignore patterns are excluded
    - Verify that negation patterns work correctly
    - _Requirements: 2.1_

- [x] 22. Add hashdeep format support
  - [x] 22.1 Implement hashdeep format reader
    - Parse hashdeep format with header and CSV-style entries
    - Support hashdeep format: `size,md5,sha256,filename`
    - Handle hashdeep header lines (starting with %)
    - _Requirements: 2.2, 9.5_
  
  - [x] 22.2 Implement hashdeep format writer
    - Add `--format hashdeep` flag to scan command
    - Write output in hashdeep-compatible format
    - Include size, multiple hashes, and filename
    - Add proper hashdeep header with metadata
    - _Requirements: 2.2, 9.5_
  
  - [x] 22.3 Update verify command for hashdeep format
    - Auto-detect hashdeep format when reading database
    - Support verification of hashdeep-format databases
    - _Requirements: 3.1, 9.5_
  
  - [ ]* 22.4 Write property test for hashdeep format round-trip
    - **Property: Hashdeep format round-trip**
    - Verify that writing and reading hashdeep format preserves data
    - _Requirements: 2.2, 9.5_

- [x] 23. Add JSON output format
  - [x] 23.1 Add serde dependencies
    - Add serde and serde_json to Cargo.toml
    - Derive Serialize for relevant structs
    - _Requirements: 9.1_
  
  - [x] 23.2 Implement JSON output option
    - Add `--json` flag to all commands
    - Output results as structured JSON
    - Include metadata: timestamp, algorithm, file count, etc.
    - Format: `{"files": [{"path": "...", "hash": "...", "size": ...}], "metadata": {...}}`
    - _Requirements: 9.1_
  
  - [ ]* 23.3 Write property test for JSON serialization
    - **Property: JSON round-trip**
    - Verify that JSON output can be parsed back correctly
    - _Requirements: 9.1_

- [x] 24. Add database compression support
  - [x] 24.1 Add compression dependency
    - Add lzma-rs crate to Cargo.toml for LZMA compression
    - _Requirements: 2.2_
  
  - [x] 24.2 Implement automatic database compression
    - Add `--compress` flag to scan command
    - Automatically compress output database with LZMA
    - Use .xz extension for compressed databases
    - Auto-detect and decompress when reading .xz databases
    - _Requirements: 2.2_
  
  - [ ]* 24.3 Write property test for compression round-trip
    - **Property: Compression preserves data**
    - Verify that compressed databases can be read correctly
    - Verify that compression reduces file size
    - _Requirements: 2.2_

- [ ] 25. Final integration and testing
  - [ ] 25.1 Integration testing for new features
    - Test stdin hashing with various inputs
    - Test text hashing with special characters
    - Test HMAC with different keys
    - Test .hashignore with complex patterns
    - Test hashdeep format compatibility
    - Test JSON output parsing
    - Test database compression
    - _Requirements: 1.1, 2.1, 2.2, 9.1_
  
  - [ ] 25.2 Update documentation
    - Add examples for all new features to README
    - Document .hashignore syntax and behavior
    - Document hashdeep format compatibility
    - Document JSON output schema
    - Document HMAC usage and security considerations
    - _Requirements: 6.1, 6.2_