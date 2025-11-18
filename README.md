# Hash Utility

A high-performance, cross-platform cryptographic hash utility with SIMD optimization support.

## Features

- **Multiple Hash Algorithms**: MD5, SHA-1, SHA-2 family (SHA-224, SHA-256, SHA-384, SHA-512), SHA-3 family (SHA3-224, SHA3-256, SHA3-384, SHA3-512), BLAKE2b, BLAKE2s, BLAKE3, and non-cryptographic hashes (xxHash3, xxHash128)
- **SIMD Acceleration**: Automatic hardware acceleration using SSE, AVX, AVX2, AVX-512 (x86_64) and NEON (ARM)
- **Post-Quantum Algorithms**: Support for SHA-3 family algorithms
- **Fast Mode**: Quick hashing for large files by sampling first, middle, and last 100MB
- **Directory Scanning**: Recursively hash all files in a directory with progress tracking
- **Hash Verification**: Compare current hashes against stored database
- **Flexible Input**: Hash files, stdin, or text strings directly
- **.hashignore Support**: Exclude files using gitignore-style patterns
- **Multiple Output Formats**: Standard, hashdeep-compatible, and JSON formats
- **Database Compression**: LZMA compression for hash databases
- **Benchmarking**: Measure hash algorithm performance on your hardware
- **Minimal Binary Size**: Optimized for size with LTO and stripping
- **Cross-Platform**: Works on Linux, macOS, Windows, and FreeBSD

## Quick Start

```bash
# Build the utility
cargo build --release

# Compute a hash from a file
./target/release/hash myfile.txt -a sha256

# Hash text directly
./target/release/hash --text "hello world" -a sha256

# Hash from stdin
cat myfile.txt | ./target/release/hash -a sha256

# Fast mode for large files (samples 300MB)
./target/release/hash largefile.iso -f -a sha256

# Scan a directory
./target/release/hash scan -d ./my_directory -a sha256 -o hashes.db

# Verify integrity
./target/release/hash verify -b hashes.db -d ./my_directory

# See all available algorithms
./target/release/hash list
```

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd hash-utility

# Build with release optimizations
cargo build --release

# The binary will be at target/release/hash
```

### Optimized Build (Maximum Performance)

For maximum performance on your specific CPU:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

See [SIMD_OPTIMIZATION.md](SIMD_OPTIMIZATION.md) for detailed compilation options.

## Usage

### Getting Help

```bash
# Display general help
hash --help
hash -h          # Short form (equivalent)

# Display help for a specific command
hash hash --help
hash scan --help
hash verify --help
hash benchmark --help
```

### Command-Line Flags

All commands support both short and long form flags:

| Short | Long | Description | Commands |
|-------|------|-------------|----------|
| `-a` | `--algorithm` | Specify hash algorithm | hash, scan |
| `-o` | `--output` | Specify output file | hash, scan |
| `-f` | `--fast` | Enable fast mode (samples 300MB) | hash, scan |
| `-t` | `--text` | Hash text directly | hash |
| `-d` | `--directory` | Specify directory | scan, verify |
| `-b` | `--database` | Specify database file | verify |
| `-p` | `--parallel` | Enable parallel processing | scan |
| `-s` | `--size` | Specify benchmark data size (MB) | benchmark |
| | `--format` | Output format (standard/hashdeep) | scan |
| | `--json` | Output in JSON format | all |
| | `--compress` | Compress database with LZMA | scan |
| `-h` | `--help` | Display help | all |
| `-V` | `--version` | Display version | all |

Both short and long forms are equivalent and can be used interchangeably.

### Feature Comparison

| Feature | Hash | Scan | Verify | Benchmark | List |
|---------|------|------|--------|-----------|------|
| File input | ✓ | - | - | - | - |
| Stdin input | ✓ | - | - | - | - |
| Text input | ✓ | - | - | - | - |
| Multiple algorithms | ✓ | - | - | ✓ | ✓ |
| Fast mode | ✓ | ✓ | - | - | - |
| Parallel processing | - | ✓ | - | - | - |
| Progress bar | - | ✓ | ✓ | - | - |
| .hashignore support | - | ✓ | - | - | - |
| JSON output | ✓ | ✓ | ✓ | ✓ | ✓ |
| Hashdeep format | - | ✓ | ✓ | - | - |
| Database compression | - | ✓ | ✓ | - | - |
| File output | ✓ | ✓ | - | - | - |

### Compute Hash for a File

```bash
# Single algorithm (defaults to SHA-256)
hash myfile.txt -a sha256

# Multiple algorithms in one pass (more efficient than running separately)
hash myfile.txt -a sha256 -a blake3 -a sha3-256

# Save output to file instead of displaying on screen
hash myfile.txt -a sha256 -o hashes.txt

# Fast mode for large files (samples first, middle, and last 100MB)
hash largefile.iso -f -a sha256

# Using long-form flags
hash myfile.txt --algorithm sha256 --output hashes.txt --fast
```

**Output format:**
```
<hash_hex>  <algorithm>  <mode>  <filepath>
```

Example:
```
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  sha256  normal  myfile.txt
```

### Hash Text Directly

Hash a text string without creating a file:

```bash
# Hash a text string
hash --text "hello world" -a sha256

# Short form
hash -t "hello world" -a sha256

# Multiple algorithms
hash -t "my secret data" -a sha256 -a sha3-256 -a blake3

# Save to file
hash -t "hello world" -a sha256 -o hash.txt
```

**Output:**
```
b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9  sha256  normal  <text>
```

### Hash from Stdin

Read data from stdin for hashing (useful for pipes):

```bash
# Pipe file contents
cat myfile.txt | hash -a sha256

# Pipe command output
echo "hello world" | hash -a sha256

# Multiple algorithms
cat largefile.bin | hash -a sha256 -a blake3

# From compressed file
gunzip -c archive.gz | hash -a sha256
```

**Note:** Fast mode is not supported for stdin or text input.

### Scan Directory

Recursively hash all files in a directory:

```bash
# Basic directory scan
hash scan -d /path/to/directory -a sha256 -o hashes.db

# Use parallel processing for faster scanning (recommended for large directories)
hash scan -d /path/to/directory -a sha256 -o hashes.db --parallel

# Fast mode for large files (samples 300MB per file)
hash scan -d /path/to/directory -a sha256 -o hashes.db -f

# Combine parallel and fast mode for maximum speed
hash scan -d /path/to/directory -a sha256 -o hashes.db -p -f

# Scan current directory
hash scan -d . -a blake3 -o checksums.txt

# Compress the output database with LZMA
hash scan -d /path/to/directory -a sha256 -o hashes.db --compress

# Output in hashdeep format
hash scan -d /path/to/directory -a sha256 -o hashes.db --format hashdeep

# Output in JSON format
hash scan -d /path/to/directory -a sha256 -o hashes.db --json

# Using long-form flags
hash scan --directory /path/to/directory --algorithm sha256 --output hashes.db --parallel --fast
```

**Progress information:**
During scanning, you'll see a progress bar with:
- Number of files processed
- Current file being hashed
- Throughput (MB/s)
- Estimated time remaining
- Number of files that failed (e.g., permission errors)
- Total bytes processed

### Using .hashignore Files

Exclude files from scanning using gitignore-style patterns:

**Create a .hashignore file:**
```bash
# Create .hashignore in the directory you want to scan
cat > /path/to/directory/.hashignore << 'EOF'
# Ignore log files
*.log

# Ignore temporary files
*.tmp
*.temp

# Ignore build directories
build/
dist/
node_modules/

# Ignore specific files
.DS_Store
Thumbs.db

# But don't ignore important.log (negation)
!important.log
EOF
```

**Supported patterns:**
- `*.ext` - Wildcard matching (all files with .ext extension)
- `dir/` - Directory matching (all files in dir/)
- `!pattern` - Negation (don't ignore files matching pattern)
- `#` - Comments (lines starting with # are ignored)
- `**/*.ext` - Recursive wildcard (all .ext files in any subdirectory)

**How it works:**
- The scanner automatically looks for `.hashignore` files in the scanned directory and parent directories
- Patterns are applied using the same rules as `.gitignore`
- Files matching ignore patterns are skipped during scanning
- The `.hashignore` file itself is always excluded from scans

**Example:**
```bash
# Create .hashignore
echo "*.log" > /data/.hashignore
echo "temp/" >> /data/.hashignore

# Scan directory (log files and temp/ will be excluded)
hash scan -d /data -a sha256 -o hashes.db
```

### Verify Directory

Compare current hashes against a stored database:

```bash
# Verify directory integrity
hash verify -b hashes.db -d /path/to/directory

# Verify compressed database (automatically decompressed)
hash verify -b hashes.db.xz -d /path/to/directory

# Output in JSON format
hash verify -b hashes.db -d /path/to/directory --json

# Using long-form flags
hash verify --database hashes.db --directory /path/to/directory
```

**Verification report includes:**
- **Matches**: Files with unchanged hashes (integrity verified)
- **Mismatches**: Files with changed hashes (modified or corrupted)
- **Missing**: Files in database but not in filesystem (deleted)
- **New**: Files in filesystem but not in database (added)

Example output:
```
Verification Report:
  Matches: 150 files
  Mismatches: 2 files
    - /path/to/modified.txt
      Expected: abc123...
      Actual:   def456...
  Missing: 1 file
    - /path/to/deleted.txt
  New: 3 files
    - /path/to/newfile1.txt
    - /path/to/newfile2.txt
```

**Note:** The verify command automatically detects the database format (standard, hashdeep, or compressed) and handles it appropriately.

### Benchmark Algorithms

Test hash algorithm performance on your hardware:

```bash
# Default benchmark (100 MB test data)
hash benchmark

# Custom data size (in MB)
hash benchmark --size 500

# Short form
hash benchmark -s 1000
```

**Output shows:**
- Algorithm name
- Throughput in MB/s
- Relative performance comparison

### List Available Algorithms

```bash
# List all algorithms
hash list

# Output in JSON format
hash list --json
```

This displays all supported algorithms with:
- Algorithm name
- Output size in bits
- Post-quantum resistance status
- Cryptographic vs non-cryptographic

Example output:
```
Available Hash Algorithms:

Algorithm            Output Bits   Post-Quantum   Cryptographic
-----------------------------------------------------------------
md5                          128             No             Yes
sha1                         160             No             Yes
sha224                       224             No             Yes
sha256                       256             No             Yes
sha384                       384             No             Yes
sha512                       512             No             Yes
sha3-224                     224            Yes             Yes
sha3-256                     256            Yes             Yes
sha3-384                     384            Yes             Yes
sha3-512                     512            Yes             Yes
blake2b                      512             No             Yes
blake2s                      256             No             Yes
blake3                       256             No             Yes
xxh3                          64             No              No
xxh128                       128             No              No
```

**Non-cryptographic algorithms** (xxHash3, xxHash128) are extremely fast but should not be used for security purposes. They're ideal for:
- Checksums and data integrity in trusted environments
- Hash tables and data structures
- Fast file deduplication
- Performance-critical applications where security is not a concern

## Output Formats

The Hash Utility supports multiple output formats for different use cases.

### Standard Format (Default)

Plain text format with metadata:
```
<hash>  <algorithm>  <mode>  <filepath>
```

Example:
```
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  sha256  normal  file.txt
5d41402abc4b2a76b9719d911017c592  md5  fast  largefile.iso
```

### Hashdeep Format

Compatible with the hashdeep tool for forensic analysis:

```bash
# Create hashdeep-format database
hash scan -d /path/to/directory -a sha256 -o hashes.txt --format hashdeep

# Verify hashdeep-format database (auto-detected)
hash verify -b hashes.txt -d /path/to/directory
```

**Hashdeep format structure:**
```
%%%% HASHDEEP-1.0
%%%% size,sha256,filename
## Invoked from: /path/to/directory
## $ hash scan -d /path/to/directory -a sha256 -o hashes.txt --format hashdeep
##
1024,e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855,./file.txt
2048,5d41402abc4b2a76b9719d911017c592,./data.bin
```

**Benefits:**
- Compatible with existing hashdeep tools
- Includes file size for additional verification
- Widely used in digital forensics
- Human-readable with metadata

### JSON Format

Structured output for automation and integration:

```bash
# Hash with JSON output
hash myfile.txt -a sha256 --json

# Scan with JSON output
hash scan -d /path/to/directory -a sha256 -o hashes.db --json

# Verify with JSON output
hash verify -b hashes.db -d /path/to/directory --json

# Benchmark with JSON output
hash benchmark --json

# List algorithms with JSON output
hash list --json
```

**JSON output structure for hash command:**
```json
{
  "files": [
    {
      "file_path": "myfile.txt",
      "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "algorithm": "sha256",
      "fast_mode": false
    }
  ],
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "algorithms": ["sha256"],
    "file_count": 1,
    "fast_mode": false
  }
}
```

**JSON output structure for scan command:**
```json
{
  "stats": {
    "files_processed": 150,
    "files_failed": 2,
    "total_bytes": 1048576000,
    "duration_secs": 12.5
  },
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "directory": "/path/to/directory",
    "algorithm": "sha256",
    "output_file": "hashes.db",
    "parallel": true,
    "fast_mode": false,
    "format": "standard"
  }
}
```

**JSON output structure for verify command:**
```json
{
  "report": {
    "matches": 148,
    "mismatches": [
      {
        "path": "/path/to/modified.txt",
        "expected": "abc123...",
        "actual": "def456..."
      }
    ],
    "missing_files": ["/path/to/deleted.txt"],
    "new_files": ["/path/to/newfile.txt"]
  },
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "database": "hashes.db",
    "directory": "/path/to/directory"
  }
}
```

### Database Compression

Compress hash databases to save disk space:

```bash
# Create compressed database
hash scan -d /path/to/directory -a sha256 -o hashes.db --compress

# This creates hashes.db.xz (LZMA compressed)
# The original hashes.db is automatically removed

# Verify compressed database (automatically decompressed)
hash verify -b hashes.db.xz -d /path/to/directory
```

**Compression benefits:**
- Reduces database size by 70-90% (typical)
- LZMA compression for maximum compression ratio
- Automatic decompression during verification
- Useful for archiving or transferring large databases

**Example compression ratios:**
- 100 MB database → ~10-30 MB compressed
- 1 GB database → ~100-300 MB compressed

## Common Use Cases

### Verify Downloaded File Integrity

```bash
# Download a file and its published hash
# Compute the hash and compare
hash hash -f downloaded-file.iso -a sha256

# Compare the output with the published hash
```

### Create Backup Verification Database

```bash
# Before backup: create hash database
hash scan -d /important/data -a sha256 -o backup-hashes.db --parallel

# After restore: verify integrity
hash verify -b backup-hashes.db -d /restored/data
```

### Monitor Directory for Changes

```bash
# Create baseline
hash scan -d /etc/config -a sha256 -o baseline.db

# Later, check for changes
hash verify -b baseline.db -d /etc/config
```

### Compare Two Directories

```bash
# Hash first directory
hash scan -d /path/to/dir1 -a sha256 -o dir1.db

# Hash second directory
hash scan -d /path/to/dir2 -a sha256 -o dir2.db

# Compare the database files (using standard tools)
diff dir1.db dir2.db
```

### Forensic Analysis

```bash
# Use post-quantum resistant algorithm for long-term integrity
hash scan -d /evidence -a sha3-256 -o evidence-hashes.db

# Use hashdeep format for forensic compatibility
hash scan -d /evidence -a sha256 -o evidence.txt --format hashdeep

# Multiple algorithms for redundancy
hash critical-file.bin -a sha256 -a sha3-256 -a blake3

# Compress for archival
hash scan -d /evidence -a sha256 -o evidence.db --compress
```

### Quick Checksums for Large Files

```bash
# Fast mode for quick integrity checks (samples 300MB)
hash large-backup.tar.gz -f -a blake3

# Fast mode with parallel scanning
hash scan -d /backups -a blake3 -o checksums.db -p -f

# Non-cryptographic hash for maximum speed (trusted environment only)
hash huge-dataset.bin -a xxh3
```

### Automated Workflows with JSON

```bash
# Generate JSON output for parsing
hash scan -d /data -a sha256 -o hashes.db --json > scan-results.json

# Parse with jq
hash verify -b hashes.db -d /data --json | jq '.report.mismatches'

# Check if verification passed
if hash verify -b hashes.db -d /data --json | jq -e '.report.mismatches | length == 0' > /dev/null; then
  echo "Verification passed"
else
  echo "Verification failed"
fi
```

### Excluding Files with .hashignore

```bash
# Create .hashignore to exclude build artifacts
cat > /project/.hashignore << 'EOF'
# Build outputs
target/
build/
dist/
*.o
*.so

# Dependencies
node_modules/
vendor/

# Logs and temp files
*.log
*.tmp
.DS_Store
EOF

# Scan project (excluded files will be skipped)
hash scan -d /project -a sha256 -o project-hashes.db

# Result: Only source files are hashed, build artifacts are ignored
```

## Performance

With SIMD enabled on modern hardware (AVX2 or better):

| Algorithm | Typical Throughput | Use Case |
|-----------|-------------------|----------|
| xxHash3 | 10,000-30,000 MB/s | Non-cryptographic, maximum speed |
| xxHash128 | 10,000-30,000 MB/s | Non-cryptographic, 128-bit output |
| BLAKE3 | 1,000-3,000 MB/s | Cryptographic, fastest secure hash |
| BLAKE2b | 800-1,200 MB/s | Cryptographic, good balance |
| SHA-512 | 600-900 MB/s | Cryptographic, widely supported |
| SHA-256 | 500-800 MB/s | Cryptographic, most common |
| SHA-3-256 | 200-400 MB/s | Post-quantum resistant |

Actual performance depends on your CPU and compilation flags. Run `hash benchmark` to test on your system.

**Performance Tips:**
- Use `--parallel` flag for scanning large directories (2-4x faster on multi-core systems)
- Use `--fast` flag for large files (10-100x faster for files > 1GB)
- Use non-cryptographic hashes (xxHash3) for maximum speed in trusted environments
- Compile with `RUSTFLAGS="-C target-cpu=native"` for maximum performance
- BLAKE3 is typically the fastest cryptographic algorithm
- SHA-512 is often faster than SHA-256 on 64-bit systems
- Combine `--parallel` and `--fast` for maximum throughput on large datasets

**Fast Mode Performance:**
Fast mode samples 300MB (first 100MB + middle 100MB + last 100MB) instead of reading the entire file:

| File Size | Normal Mode | Fast Mode | Speedup |
|-----------|-------------|-----------|---------|
| 1 GB | ~2 seconds | ~0.3 seconds | ~7x |
| 10 GB | ~20 seconds | ~0.3 seconds | ~67x |
| 100 GB | ~200 seconds | ~0.3 seconds | ~667x |

*Note: Fast mode provides quick integrity checks but is not suitable for cryptographic verification of the entire file content.*

## SIMD Optimization

The Hash Utility automatically uses SIMD instructions when available:

- **x86_64**: SSE2, SSE4.1, AVX, AVX2, AVX-512
- **ARM/AArch64**: NEON

### Verifying SIMD Support

Run the SIMD verification tests:

```bash
cargo test --release --test simd_verification -- --nocapture
```

This will:
- Test that BLAKE3 uses SIMD when available
- Verify scalar fallback works correctly
- Display available CPU features
- Show performance comparisons

For detailed information about SIMD optimization, see [SIMD_OPTIMIZATION.md](SIMD_OPTIMIZATION.md).

## Database Formats

The Hash Utility supports multiple database formats for different use cases.

### Standard Format (Default)

Plain text format with metadata:

```
<hash>  <algorithm>  <mode>  <filepath>
```

Example:
```
d41d8cd98f00b204e9800998ecf8427e  md5  normal  ./empty.txt
5d41402abc4b2a76b9719d911017c592  md5  fast  ./hello.txt
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  sha256  normal  ./data.bin
```

**Fields:**
- `hash`: Hexadecimal hash value
- `algorithm`: Hash algorithm used (md5, sha256, blake3, etc.)
- `mode`: Hashing mode (normal or fast)
- `filepath`: Relative or absolute path to the file

### Hashdeep Format

Compatible with the hashdeep forensic tool:

```
%%%% HASHDEEP-1.0
%%%% size,sha256,filename
## Invoked from: /path/to/directory
## $ hash scan -d /path/to/directory -a sha256 -o hashes.txt --format hashdeep
##
1024,e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855,./file.txt
2048,5d41402abc4b2a76b9719d911017c592,./data.bin
```

**Features:**
- Header with metadata (lines starting with `%` or `##`)
- CSV format: `size,hash,filename`
- Compatible with existing hashdeep tools
- Includes file size for additional verification

### Compressed Format

Any database can be compressed with LZMA:

```bash
# Create compressed database
hash scan -d /path/to/directory -a sha256 -o hashes.db --compress

# Creates hashes.db.xz (LZMA compressed)
```

**Benefits:**
- 70-90% size reduction (typical)
- Automatic decompression during verification
- Useful for archiving or transferring large databases

## Fast Mode

Fast mode provides quick integrity checks for large files by sampling instead of reading the entire file.

### How Fast Mode Works

Instead of reading the entire file, fast mode samples three regions:
1. **First 100MB** of the file
2. **Middle 100MB** of the file (centered at file_size / 2)
3. **Last 100MB** of the file

The three samples are concatenated and hashed together, providing a deterministic hash that's much faster to compute for large files.

### When to Use Fast Mode

**Good use cases:**
- Quick integrity checks for large files (> 1GB)
- Detecting major file corruption or tampering
- Rapid scanning of large datasets
- Backup verification where speed is critical
- Preliminary checks before full verification

**Not suitable for:**
- Cryptographic verification of entire file contents
- Detecting small changes in the middle of large files
- Legal or forensic evidence (use normal mode)
- Files smaller than 300MB (no benefit)

### Usage Examples

```bash
# Hash a large file in fast mode
hash large-backup.tar.gz -f -a sha256

# Scan directory with fast mode
hash scan -d /backups -a sha256 -o hashes.db -f

# Combine with parallel processing for maximum speed
hash scan -d /large-dataset -a blake3 -o hashes.db -p -f
```

### Performance Comparison

| File Size | Normal Mode | Fast Mode | Time Saved |
|-----------|-------------|-----------|------------|
| 500 MB | 1.0 sec | 0.3 sec | 70% |
| 1 GB | 2.0 sec | 0.3 sec | 85% |
| 10 GB | 20 sec | 0.3 sec | 98.5% |
| 100 GB | 200 sec | 0.3 sec | 99.85% |

*Times are approximate and depend on disk speed and CPU performance.*

### Important Notes

- Fast mode always samples exactly 300MB (or the entire file if smaller)
- The middle region is calculated deterministically, so the same file always produces the same hash
- Fast mode is not supported for stdin or text input
- Files smaller than 300MB are read entirely (no sampling)
- Fast mode hashes are marked as "fast" in the database for transparency

## Post-Quantum Algorithms

The following algorithms are considered post-quantum resistant:

- SHA3-224
- SHA3-256
- SHA3-384
- SHA3-512

Use `hash list` to see which algorithms are marked as post-quantum.

## Cross-Platform Support

The Hash Utility works on:

- **Linux** (x86_64, ARM64)
- **macOS** (Intel, Apple Silicon)
- **Windows** (x86_64)
- **FreeBSD** (x86_64)

Path handling is automatically adapted for each platform.

## Troubleshooting

### "Unsupported algorithm" Error

```bash
# List all available algorithms
hash list

# Use exact algorithm name from the list
hash hash -f myfile.txt -a sha256  # Correct
hash hash -f myfile.txt -a SHA256  # May not work (case-sensitive)
```

### Permission Errors During Scan

The utility will log permission errors but continue scanning other files:

```bash
# Run with appropriate permissions
sudo hash scan -d /protected/directory -a sha256 -o hashes.db
```

### Verification Shows Many Mismatches

Possible causes:
1. **Different algorithm used**: Ensure you use the same algorithm for scan and verify
2. **Files were actually modified**: This is expected behavior
3. **Path differences**: Ensure you're verifying from the same base directory

```bash
# Check which algorithm was used in the database
head -1 hashes.db  # Look at hash length to identify algorithm
```

### Slow Performance

```bash
# Enable parallel processing
hash scan -d /large/directory -a sha256 -o hashes.db --parallel

# Use a faster algorithm
hash scan -d /large/directory -a blake3 -o hashes.db --parallel

# Compile with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Output Not Appearing

If using `-o` flag, output goes to file, not stdout:

```bash
# This writes to file (nothing on screen)
hash hash -f myfile.txt -a sha256 -o output.txt

# This displays on screen
hash hash -f myfile.txt -a sha256
```

### Binary Size Too Large

```bash
# Ensure you're building in release mode
cargo build --release

# Check the binary size
ls -lh target/release/hash

# Strip additional symbols (if not already done)
strip target/release/hash
```

### Fast Mode Not Working

Fast mode is only supported for file input:

```bash
# This works
hash largefile.iso -f -a sha256

# This doesn't work (stdin)
cat largefile.iso | hash -f -a sha256
# Error: Fast mode is not supported when reading from stdin

# This doesn't work (text)
hash -t "hello" -f -a sha256
# Error: Fast mode is not supported when hashing text
```

### .hashignore Not Working

Check that your .hashignore file is in the correct location:

```bash
# .hashignore should be in the scanned directory or a parent directory
ls -la /path/to/directory/.hashignore

# Check the patterns
cat /path/to/directory/.hashignore

# Test with a simple pattern
echo "*.log" > /path/to/directory/.hashignore
hash scan -d /path/to/directory -a sha256 -o hashes.db
```

### Compressed Database Issues

```bash
# Verify the file has .xz extension
ls -lh hashes.db.xz

# The tool automatically detects and decompresses .xz files
hash verify -b hashes.db.xz -d /path/to/directory

# If you need to manually decompress
xz -d hashes.db.xz  # Creates hashes.db
```

### JSON Output Not Valid

Ensure you're using the `--json` flag:

```bash
# Correct
hash scan -d /path/to/directory -a sha256 -o hashes.db --json

# Incorrect (no JSON output)
hash scan -d /path/to/directory -a sha256 -o hashes.db
```

## .hashignore Syntax Reference

The `.hashignore` file uses gitignore-style patterns to exclude files from scanning.

### Basic Patterns

```bash
# Ignore all .log files
*.log

# Ignore all files in temp directory
temp/

# Ignore specific file
config.local

# Ignore all .tmp files in any directory
**/*.tmp
```

### Wildcards

- `*` - Matches any characters except `/`
- `**` - Matches any characters including `/` (recursive)
- `?` - Matches any single character
- `[abc]` - Matches any character in the set
- `[a-z]` - Matches any character in the range

```bash
# Examples
*.txt           # All .txt files in current directory
**/*.txt        # All .txt files in any subdirectory
file?.txt       # file1.txt, fileA.txt, etc.
file[0-9].txt   # file0.txt through file9.txt
```

### Negation

Use `!` to negate a pattern (don't ignore):

```bash
# Ignore all .log files
*.log

# But don't ignore important.log
!important.log

# Ignore everything in temp/
temp/*

# But don't ignore temp/keep.txt
!temp/keep.txt
```

### Comments

Lines starting with `#` are comments:

```bash
# This is a comment
*.log           # This is also a comment

# Ignore build artifacts
build/
dist/
```

### Directory Matching

Patterns ending with `/` only match directories:

```bash
# Ignore directory named "temp"
temp/

# Ignore all directories named "node_modules"
**/node_modules/
```

### Complete Example

```bash
# .hashignore example for a software project

# Build outputs
build/
dist/
target/
*.o
*.so
*.dll
*.exe

# Dependencies
node_modules/
vendor/
.venv/

# IDE and editor files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db
desktop.ini

# Logs and temporary files
*.log
*.tmp
*.temp
*.cache

# But keep important logs
!important.log
!audit.log

# Test data
test/fixtures/large-files/

# Documentation builds
docs/_build/
docs/.doctrees/
```

### How .hashignore Files Are Found

1. The scanner looks for `.hashignore` in the scanned directory
2. It then looks in each parent directory up to the root
3. All found `.hashignore` files are combined
4. Patterns from child directories take precedence

```bash
# Example directory structure
/project/.hashignore          # Applies to entire project
/project/src/.hashignore      # Additional patterns for src/
/project/tests/.hashignore    # Additional patterns for tests/

# When scanning /project/src, both /project/.hashignore and
# /project/src/.hashignore are applied
```

## Security Considerations

### Algorithm Selection

**Cryptographic vs Non-Cryptographic:**
- **Cryptographic hashes** (SHA-256, SHA-3, BLAKE3): Use for security-sensitive applications, file integrity verification, digital signatures
- **Non-cryptographic hashes** (xxHash3, xxHash128): Use only in trusted environments for performance-critical applications

**Deprecated Algorithms:**
- **MD5**: Cryptographically broken, use only for compatibility with legacy systems
- **SHA-1**: Deprecated for security use, collision attacks are practical

**Recommended Algorithms:**
- **SHA-256**: Widely supported, good security, reasonable performance
- **SHA-3-256**: Post-quantum resistant, future-proof
- **BLAKE3**: Fastest cryptographic hash, modern design

### Fast Mode Security

Fast mode is **not suitable** for:
- Cryptographic verification
- Detecting targeted tampering (attacker could modify unsampled regions)
- Legal or forensic evidence
- Security-critical applications

Fast mode **is suitable** for:
- Quick integrity checks in trusted environments
- Detecting accidental corruption
- Backup verification
- Performance-critical applications where full verification is impractical

### Database Security

**Protecting Hash Databases:**
- Store hash databases securely (they reveal file structure)
- Use appropriate file permissions (chmod 600)
- Consider encrypting databases containing sensitive file information
- Compress databases before transmission to reduce size

**Verification Security:**
- Always use the same algorithm for scanning and verification
- Verify databases are not tampered with (store checksums of databases)
- Use post-quantum algorithms for long-term integrity

### HMAC Support

**Note:** HMAC (Hash-based Message Authentication Code) support is planned but not yet implemented. HMAC provides:
- Keyed hashing for authentication
- Protection against tampering
- Verification that data came from someone with the secret key

**Planned usage:**
```bash
# Future feature (not yet implemented)
hash myfile.txt -a sha256 --key "secret-key"
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --release

# Run SIMD verification tests with output
cargo test --release --test simd_verification -- --nocapture

# Run unit tests
cargo test --release --lib
```

### Building for Distribution

```bash
# Portable build (works on any CPU of target architecture)
cargo build --release

# Optimized for specific CPU generation
RUSTFLAGS="-C target-cpu=haswell" cargo build --release  # AVX2 support
```

## JSON Output Schema

All commands support JSON output with the `--json` flag for automation and integration.

### Hash Command JSON Schema

```json
{
  "files": [
    {
      "file_path": "string",      // Path to the file (or "<text>" or "<stdin>")
      "hash": "string",            // Hexadecimal hash value
      "algorithm": "string",       // Algorithm used (e.g., "sha256")
      "fast_mode": boolean         // Whether fast mode was used
    }
  ],
  "metadata": {
    "timestamp": "string",         // ISO 8601 timestamp
    "algorithms": ["string"],      // List of algorithms used
    "file_count": number,          // Number of files hashed
    "fast_mode": boolean           // Whether fast mode was enabled
  }
}
```

**Example:**
```json
{
  "files": [
    {
      "file_path": "myfile.txt",
      "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "algorithm": "sha256",
      "fast_mode": false
    }
  ],
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "algorithms": ["sha256"],
    "file_count": 1,
    "fast_mode": false
  }
}
```

### Scan Command JSON Schema

```json
{
  "stats": {
    "files_processed": number,     // Number of files successfully hashed
    "files_failed": number,        // Number of files that failed
    "total_bytes": number,         // Total bytes processed
    "duration_secs": number        // Time taken in seconds
  },
  "metadata": {
    "timestamp": "string",         // ISO 8601 timestamp
    "directory": "string",         // Directory that was scanned
    "algorithm": "string",         // Algorithm used
    "output_file": "string",       // Output database file path
    "parallel": boolean,           // Whether parallel mode was used
    "fast_mode": boolean,          // Whether fast mode was used
    "format": "string"             // Output format ("standard" or "hashdeep")
  }
}
```

**Example:**
```json
{
  "stats": {
    "files_processed": 150,
    "files_failed": 2,
    "total_bytes": 1048576000,
    "duration_secs": 12.5
  },
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "directory": "/path/to/directory",
    "algorithm": "sha256",
    "output_file": "hashes.db",
    "parallel": true,
    "fast_mode": false,
    "format": "standard"
  }
}
```

### Verify Command JSON Schema

```json
{
  "report": {
    "matches": number,             // Number of files with matching hashes
    "mismatches": [                // Files with changed hashes
      {
        "path": "string",          // File path
        "expected": "string",      // Expected hash from database
        "actual": "string"         // Actual hash computed
      }
    ],
    "missing_files": ["string"],   // Files in database but not in filesystem
    "new_files": ["string"]        // Files in filesystem but not in database
  },
  "metadata": {
    "timestamp": "string",         // ISO 8601 timestamp
    "database": "string",          // Database file path
    "directory": "string"          // Directory that was verified
  }
}
```

**Example:**
```json
{
  "report": {
    "matches": 148,
    "mismatches": [
      {
        "path": "/path/to/modified.txt",
        "expected": "abc123...",
        "actual": "def456..."
      }
    ],
    "missing_files": ["/path/to/deleted.txt"],
    "new_files": ["/path/to/newfile.txt"]
  },
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "database": "hashes.db",
    "directory": "/path/to/directory"
  }
}
```

### Benchmark Command JSON Schema

```json
{
  "results": [
    {
      "algorithm": "string",       // Algorithm name
      "throughput_mbps": number,   // Throughput in MB/s
      "simd_enabled": boolean      // Whether SIMD was used
    }
  ],
  "metadata": {
    "timestamp": "string",         // ISO 8601 timestamp
    "data_size_mb": number,        // Size of test data in MB
    "algorithm_count": number      // Number of algorithms tested
  }
}
```

**Example:**
```json
{
  "results": [
    {
      "algorithm": "blake3",
      "throughput_mbps": 2500.5,
      "simd_enabled": true
    },
    {
      "algorithm": "sha256",
      "throughput_mbps": 650.2,
      "simd_enabled": true
    }
  ],
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "data_size_mb": 100,
    "algorithm_count": 15
  }
}
```

### List Command JSON Schema

```json
{
  "algorithms": [
    {
      "name": "string",            // Algorithm name
      "output_bits": number,       // Output size in bits
      "post_quantum": boolean,     // Post-quantum resistant
      "cryptographic": boolean     // Cryptographic vs non-cryptographic
    }
  ],
  "metadata": {
    "timestamp": "string",         // ISO 8601 timestamp
    "algorithm_count": number      // Number of algorithms available
  }
}
```

**Example:**
```json
{
  "algorithms": [
    {
      "name": "sha256",
      "output_bits": 256,
      "post_quantum": false,
      "cryptographic": true
    },
    {
      "name": "xxh3",
      "output_bits": 64,
      "post_quantum": false,
      "cryptographic": false
    }
  ],
  "metadata": {
    "timestamp": "2024-01-15T10:30:00Z",
    "algorithm_count": 15
  }
}
```

### Using JSON Output in Scripts

**Bash with jq:**
```bash
# Check if verification passed
if hash verify -b hashes.db -d /data --json | jq -e '.report.mismatches | length == 0' > /dev/null; then
  echo "✓ Verification passed"
else
  echo "✗ Verification failed"
  hash verify -b hashes.db -d /data --json | jq '.report.mismatches'
fi

# Get scan statistics
hash scan -d /data -a sha256 -o hashes.db --json | jq '.stats'

# List only post-quantum algorithms
hash list --json | jq '.algorithms[] | select(.post_quantum == true) | .name'
```

**Python:**
```python
import json
import subprocess

# Run hash command and parse JSON
result = subprocess.run(
    ['hash', 'myfile.txt', '-a', 'sha256', '--json'],
    capture_output=True,
    text=True
)
data = json.loads(result.stdout)
hash_value = data['files'][0]['hash']
print(f"Hash: {hash_value}")

# Run verification and check results
result = subprocess.run(
    ['hash', 'verify', '-b', 'hashes.db', '-d', '/data', '--json'],
    capture_output=True,
    text=True
)
data = json.loads(result.stdout)
if len(data['report']['mismatches']) == 0:
    print("Verification passed")
else:
    print(f"Found {len(data['report']['mismatches'])} mismatches")
```

## Requirements

- Rust 1.70 or later
- No runtime dependencies beyond the standard library

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

This project uses hash implementations from:
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic hash functions
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - High-performance cryptographic hash
