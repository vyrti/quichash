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

- [-] 12. Implement error handling
  - [x] 12.1 Define error types
    - Create HashError enum with variants for all error categories
    - Implement Display trait for user-friendly error messages
    - Add context to errors (file paths, operations)
    - _Requirements: 6.3_
  
  - [ ] 12.2 Add error handling throughout codebase
    - Use Result types consistently
    - Provide helpful error messages with suggestions
    - Log errors during directory scans without stopping
    - _Requirements: 2.4, 6.3_

- [ ] 13. Add SIMD optimization verification
  - [ ] 13.1 Verify SIMD support in hash crates
    - Test that BLAKE3 uses SIMD when available
    - Verify fallback to scalar implementations works
    - Document RUSTFLAGS for optimal compilation
    - _Requirements: 1.3, 4.4, 4.5_

- [ ] 14. Final integration and testing
  - [ ] 14.2 Create usage examples and documentation
    - Add README with installation instructions
    - Document all commands with examples
    - _Requirements: 6.1, 6.2_