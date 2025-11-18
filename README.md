# Hash Utility

High-performance cryptographic hash utility with SIMD optimization.

## Features

- **Algorithms**: MD5, SHA-1, SHA-2/3, BLAKE2/3, xxHash3/128
- **SIMD**: Automatic hardware acceleration (SSE, AVX, AVX2, AVX-512, NEON)
- **Fast Mode**: Quick hashing for large files (samples 300MB)
- **Flexible Input**: Files, stdin, or text strings
- **Directory Scanning**: Recursive hashing with parallel processing
- **Verification**: Compare hashes against stored database
- **.hashignore**: Exclude files using gitignore patterns
- **Formats**: Standard, hashdeep, JSON
- **Compression**: LZMA compression for databases
- **Cross-Platform**: Linux, macOS, Windows, FreeBSD

## Quick Start

```bash
cargo build --release

# Hash a file
./target/release/hash myfile.txt -a sha256

# Hash text
./target/release/hash --text "hello world" -a sha256

# Hash from stdin
cat myfile.txt | ./target/release/hash -a sha256

# Scan directory
./target/release/hash scan -d ./my_dir -a sha256 -o hashes.db

# Verify
./target/release/hash verify -b hashes.db -d ./my_dir

# List algorithms
./target/release/hash list
```

## Usage

### Hash Files

```bash
hash myfile.txt -a sha256                    # Single algorithm
hash myfile.txt -a sha256 -a blake3          # Multiple algorithms
hash largefile.iso -f -a blake3              # Fast mode
hash myfile.txt -a sha256 -o output.txt      # Save to file
hash myfile.txt -a sha256 --json             # JSON output
```

### Hash Text or Stdin

```bash
hash --text "hello world" -a sha256          # Hash text
cat myfile.txt | hash -a sha256              # Hash from stdin
```

### Scan Directory

```bash
hash scan -d /path/to/dir -a sha256 -o hashes.db              # Basic
hash scan -d /path/to/dir -a sha256 -o hashes.db -p           # Parallel
hash scan -d /path/to/dir -a sha256 -o hashes.db -f           # Fast mode
hash scan -d /path/to/dir -a sha256 -o hashes.db -p -f        # Both
hash scan -d /path/to/dir -a sha256 -o hashes.db --compress   # Compressed
hash scan -d /path/to/dir -a sha256 -o hashes.db --format hashdeep  # Hashdeep
```

### Verify Directory

```bash
hash verify -b hashes.db -d /path/to/dir              # Verify
hash verify -b hashes.db.xz -d /path/to/dir           # Compressed
hash verify -b hashes.db -d /path/to/dir --json       # JSON
```

Output shows: Matches, Mismatches, Missing files, New files

### Benchmark & List

```bash
hash benchmark                    # Benchmark all algorithms
hash benchmark -s 500             # Custom data size
hash list                         # List algorithms
hash list --json                  # JSON output
```

## Command-Line Options

| Command | Option | Description |
|---------|--------|-------------|
| hash | `FILE` | File to hash (omit for stdin) |
| | `-t, --text <TEXT>` | Hash text string |
| | `-a, --algorithm <ALG>` | Algorithm (default: sha256) |
| | `-o, --output <FILE>` | Write to file |
| | `-f, --fast` | Fast mode (samples 300MB) |
| | `--json` | JSON output |
| scan | `-d, --directory <DIR>` | Directory to scan |
| | `-a, --algorithm <ALG>` | Algorithm (default: sha256) |
| | `-o, --output <FILE>` | Output database |
| | `-p, --parallel` | Parallel processing |
| | `-f, --fast` | Fast mode |
| | `--format <FMT>` | standard or hashdeep |
| | `--compress` | LZMA compression |
| | `--json` | JSON output |
| verify | `-b, --database <FILE>` | Database file |
| | `-d, --directory <DIR>` | Directory to verify |
| | `--json` | JSON output |
| benchmark | `-s, --size <MB>` | Data size (default: 100) |
| | `--json` | JSON output |

## .hashignore

Exclude files using gitignore-style patterns:

```bash
cat > /path/to/dir/.hashignore << 'EOF'
*.log
*.tmp
build/
node_modules/
!important.log
EOF

hash scan -d /path/to/dir -a sha256 -o hashes.db
```

Patterns: `*.ext`, `dir/`, `!pattern`, `#comments`, `**/*.ext`

## Output Formats

**Standard** (default):
```
<hash>  <algorithm>  <mode>  <filepath>
```

**Hashdeep**: CSV format with file size, compatible with hashdeep tool

**JSON**: Structured output for automation

## Performance

| Algorithm | Throughput | Use Case |
|-----------|-----------|----------|
| xxHash3 | 10-30 GB/s | Non-crypto, max speed |
| BLAKE3 | 1-3 GB/s | Crypto, fastest |
| SHA-512 | 600-900 MB/s | Crypto, 64-bit |
| SHA-256 | 500-800 MB/s | Crypto, common |
| SHA3-256 | 200-400 MB/s | Post-quantum |

**Tips:**
- Use `-p` for parallel (2-4x faster)
- Use `-f` for large files (10-100x faster)
- Use BLAKE3 for fastest crypto
- Compile with `RUSTFLAGS="-C target-cpu=native"` for best performance

**Fast Mode Speedup:**
- 1 GB: ~7x faster
- 10 GB: ~67x faster
- 100 GB: ~667x faster

## Fast Mode

Samples 300MB (first/middle/last 100MB) instead of entire file.

**Good for:** Quick checks, large files, backups
**Not for:** Full verification, forensics, small files

## Common Use Cases

```bash
# Verify downloaded file
hash downloaded-file.iso -a sha256

# Backup verification
hash scan -d /data -a sha256 -o backup.db -p
hash verify -b backup.db -d /data

# Monitor changes
hash scan -d /etc/config -a sha256 -o baseline.db
hash verify -b baseline.db -d /etc/config

# Forensic analysis
hash scan -d /evidence -a sha3-256 -o evidence.db
hash scan -d /evidence -a sha256 -o evidence.txt --format hashdeep

# Quick checksums
hash large-backup.tar.gz -f -a blake3
hash scan -d /backups -a blake3 -o checksums.db -p -f

# Automation
hash verify -b hashes.db -d /data --json | jq '.report.mismatches'
```

## Algorithm Selection

**Recommended:**
- SHA-256: Widely supported, good security
- BLAKE3: Fastest cryptographic hash
- SHA3-256: Post-quantum resistant

**Deprecated:**
- MD5, SHA-1: Use only for compatibility

**Non-crypto (trusted environments):**
- xxHash3/128: Maximum speed

## SIMD Optimization

Automatic support for SSE, AVX, AVX2, AVX-512 (x86_64) and NEON (ARM).

Verify: `cargo test --release --test simd_verification -- --nocapture`

See [SIMD_OPTIMIZATION.md](SIMD_OPTIMIZATION.md) for details.

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Unsupported algorithm | Run `hash list` to see available algorithms |
| Permission errors | Use `sudo hash scan -d /protected/dir ...` |
| Slow performance | Use `-p` for parallel, `-f` for fast mode, or BLAKE3 |
| Fast mode not working | Fast mode only works with files (not stdin/text) |
| .hashignore not working | Check file location: `/path/to/dir/.hashignore` |

## Development

```bash
cargo test --release                    # Run tests
cargo build --release                   # Build
RUSTFLAGS="-C target-cpu=native" cargo build --release  # Optimized
```

## Requirements

- Rust 1.70+
- No runtime dependencies

## License

[Add your license here]

## Acknowledgments

- [RustCrypto](https://github.com/RustCrypto)
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3)
