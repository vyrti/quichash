mod cli;
mod hash;
mod scan;
mod verify;
mod benchmark;
mod database;
mod path_utils;
mod error;
mod ignore_handler;

use cli::{parse_args, Command};
use hash::{HashComputer, HashRegistry};
use scan::ScanEngine;
use verify::VerifyEngine;
use benchmark::BenchmarkEngine;
use database::DatabaseFormat;
use error::HashUtilityError;
use std::process;

fn main() {
    // Parse command-line arguments
    let cli = match parse_args() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    
    // Dispatch to appropriate handler
    let result = match cli.command {
        Some(Command::Scan { directory, algorithm, output, parallel, fast, format, json, compress }) => {
            handle_scan_command(&directory, &algorithm, &output, parallel, fast, &format, json, compress)
        }
        Some(Command::Verify { database, directory, json }) => {
            handle_verify_command(&database, &directory, json)
        }
        Some(Command::Benchmark { size_mb, json }) => {
            handle_benchmark_command(size_mb, json)
        }
        Some(Command::List { json }) => {
            handle_list_command(json)
        }
        None => {
            // No subcommand means hash mode (default)
            handle_hash_command(cli.file.as_deref(), cli.text.as_deref(), &cli.algorithms, cli.output.as_deref(), cli.fast, cli.json)
        }
    };
    
    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Handle the hash command: compute and display hash(es) for a file, text, or stdin
fn handle_hash_command(
    file: Option<&std::path::Path>,
    text: Option<&str>,
    algorithms: &[String],
    output: Option<&std::path::Path>,
    fast: bool,
    json: bool,
) -> Result<(), HashUtilityError> {
    let computer = HashComputer::new();
    
    // Compute hashes for all specified algorithms
    let results = match (file, text) {
        (Some(file_path), None) => {
            // Hash from file
            if fast {
                // Use fast mode for each algorithm
                let mut results = Vec::new();
                for algorithm in algorithms {
                    results.push(computer.compute_hash_fast(file_path, algorithm)?);
                }
                results
            } else {
                // Use normal mode
                computer.compute_multiple_hashes(file_path, algorithms)?
            }
        }
        (None, Some(text_input)) => {
            // Hash from text (fast mode not supported for text)
            if fast {
                return Err(HashUtilityError::InvalidArguments {
                    message: "Fast mode is not supported when hashing text".to_string(),
                });
            }
            computer.compute_multiple_hashes_text(text_input, algorithms)?
        }
        (None, None) => {
            // Hash from stdin (fast mode not supported for stdin)
            if fast {
                return Err(HashUtilityError::InvalidArguments {
                    message: "Fast mode is not supported when reading from stdin".to_string(),
                });
            }
            computer.compute_multiple_hashes_stdin(algorithms)?
        }
        (Some(_), Some(_)) => {
            // This should be prevented by clap's conflicts_with, but handle it anyway
            return Err(HashUtilityError::InvalidArguments {
                message: "Cannot specify both file and text arguments".to_string(),
            });
        }
    };
    
    // Format output based on json flag
    let output_content = if json {
        // JSON output
        #[derive(serde::Serialize)]
        struct HashOutput {
            files: Vec<hash::HashResult>,
            metadata: HashMetadata,
        }
        
        #[derive(serde::Serialize)]
        struct HashMetadata {
            timestamp: String,
            algorithms: Vec<String>,
            file_count: usize,
            fast_mode: bool,
        }
        
        let output = HashOutput {
            files: results.clone(),
            metadata: HashMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                algorithms: algorithms.to_vec(),
                file_count: results.len(),
                fast_mode: fast,
            },
        };
        
        serde_json::to_string_pretty(&output).map_err(|e| {
            HashUtilityError::InvalidArguments {
                message: format!("Failed to serialize JSON: {}", e),
            }
        })?
    } else {
        // Plain text output
        let mut output_lines = Vec::new();
        for result in results {
            output_lines.push(format!("{}  {}", result.hash, result.file_path.display()));
        }
        output_lines.join("\n") + "\n"
    };
    
    // Write to output destination
    if let Some(output_path) = output {
        // Write to file with better error context
        std::fs::write(output_path, output_content).map_err(|e| {
            HashUtilityError::from_io_error(e, "writing output", Some(output_path.to_path_buf()))
        })?;
    } else {
        // Write to stdout
        print!("{}", output_content);
    }
    
    Ok(())
}

/// Handle the scan command: scan directory and write database
fn handle_scan_command(
    directory: &std::path::Path,
    algorithm: &str,
    output: &std::path::Path,
    parallel: bool,
    fast: bool,
    format_str: &str,
    json: bool,
    compress: bool,
) -> Result<(), HashUtilityError> {
    // Parse format string
    let format = match format_str.to_lowercase().as_str() {
        "standard" => DatabaseFormat::Standard,
        "hashdeep" => DatabaseFormat::Hashdeep,
        _ => {
            return Err(HashUtilityError::InvalidArguments {
                message: format!("Invalid format '{}'. Valid formats are: standard, hashdeep", format_str),
            });
        }
    };
    
    let engine = ScanEngine::with_parallel(parallel)
        .with_fast_mode(fast)
        .with_format(format);
    
    // Scan directory and write database
    let stats = engine.scan_directory(directory, algorithm, output)?;
    
    // Compress the database if requested
    let final_output = if compress {
        use database::DatabaseHandler;
        
        println!("Compressing database...");
        let compressed_path = DatabaseHandler::compress_database(output)?;
        
        // Remove the uncompressed file
        std::fs::remove_file(output).map_err(|e| {
            HashUtilityError::from_io_error(e, "removing uncompressed database", Some(output.to_path_buf()))
        })?;
        
        println!("Database compressed to: {}", compressed_path.display());
        compressed_path
    } else {
        output.to_path_buf()
    };
    
    // Output results in JSON if requested
    if json {
        #[derive(serde::Serialize)]
        struct ScanOutput {
            stats: scan::ScanStats,
            metadata: ScanMetadata,
        }
        
        #[derive(serde::Serialize)]
        struct ScanMetadata {
            timestamp: String,
            directory: std::path::PathBuf,
            algorithm: String,
            output_file: std::path::PathBuf,
            parallel: bool,
            fast_mode: bool,
            format: String,
        }
        
        let output = ScanOutput {
            stats,
            metadata: ScanMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                directory: directory.to_path_buf(),
                algorithm: algorithm.to_string(),
                output_file: final_output,
                parallel,
                fast_mode: fast,
                format: format_str.to_string(),
            },
        };
        
        let json_output = serde_json::to_string_pretty(&output).map_err(|e| {
            HashUtilityError::InvalidArguments {
                message: format!("Failed to serialize JSON: {}", e),
            }
        })?;
        
        println!("{}", json_output);
    }
    
    Ok(())
}

/// Handle the verify command: compare database with directory
fn handle_verify_command(
    database: &std::path::Path,
    directory: &std::path::Path,
    json: bool,
) -> Result<(), HashUtilityError> {
    let engine = VerifyEngine::new();
    
    // Run verification
    let report = engine.verify(database, directory)?;
    
    // Output results based on format
    if json {
        #[derive(serde::Serialize)]
        struct VerifyOutput {
            report: verify::VerifyReport,
            metadata: VerifyMetadata,
        }
        
        #[derive(serde::Serialize)]
        struct VerifyMetadata {
            timestamp: String,
            database: std::path::PathBuf,
            directory: std::path::PathBuf,
        }
        
        let output = VerifyOutput {
            report,
            metadata: VerifyMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                database: database.to_path_buf(),
                directory: directory.to_path_buf(),
            },
        };
        
        let json_output = serde_json::to_string_pretty(&output).map_err(|e| {
            HashUtilityError::InvalidArguments {
                message: format!("Failed to serialize JSON: {}", e),
            }
        })?;
        
        println!("{}", json_output);
    } else {
        // Display report in plain text
        report.display();
    }
    
    Ok(())
}

/// Handle the benchmark command: run performance tests
fn handle_benchmark_command(size_mb: usize, json: bool) -> Result<(), HashUtilityError> {
    let engine = BenchmarkEngine::new();
    
    if !json {
        println!("Running benchmarks with {} MB of test data...", size_mb);
    }
    
    // Run benchmarks
    let results = engine.run_benchmarks(size_mb)?;
    
    // Output results based on format
    if json {
        #[derive(serde::Serialize)]
        struct BenchmarkOutput {
            results: Vec<benchmark::BenchmarkResult>,
            metadata: BenchmarkMetadata,
        }
        
        #[derive(serde::Serialize)]
        struct BenchmarkMetadata {
            timestamp: String,
            data_size_mb: usize,
            algorithm_count: usize,
        }
        
        let output = BenchmarkOutput {
            results: results.clone(),
            metadata: BenchmarkMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                data_size_mb: size_mb,
                algorithm_count: results.len(),
            },
        };
        
        let json_output = serde_json::to_string_pretty(&output).map_err(|e| {
            HashUtilityError::InvalidArguments {
                message: format!("Failed to serialize JSON: {}", e),
            }
        })?;
        
        println!("{}", json_output);
    } else {
        // Display results in plain text
        engine.display_results(&results);
    }
    
    Ok(())
}

/// Handle the list command: display available algorithms
fn handle_list_command(json: bool) -> Result<(), HashUtilityError> {
    let algorithms = HashRegistry::list_algorithms();
    
    if json {
        #[derive(serde::Serialize)]
        struct ListOutput {
            algorithms: Vec<hash::AlgorithmInfo>,
            metadata: ListMetadata,
        }
        
        #[derive(serde::Serialize)]
        struct ListMetadata {
            timestamp: String,
            algorithm_count: usize,
        }
        
        let output = ListOutput {
            algorithms: algorithms.clone(),
            metadata: ListMetadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                algorithm_count: algorithms.len(),
            },
        };
        
        let json_output = serde_json::to_string_pretty(&output).map_err(|e| {
            HashUtilityError::InvalidArguments {
                message: format!("Failed to serialize JSON: {}", e),
            }
        })?;
        
        println!("{}", json_output);
    } else {
        println!("\nAvailable Hash Algorithms:\n");
        println!("{:<20} {:>12} {:>15} {:>15}", "Algorithm", "Output Bits", "Post-Quantum", "Cryptographic");
        println!("{}", "-".repeat(65));
        
        for algo in algorithms {
            let pq_status = if algo.post_quantum { "Yes" } else { "No" };
            let crypto_status = if algo.cryptographic { "Yes" } else { "No" };
            println!("{:<20} {:>12} {:>15} {:>15}", algo.name, algo.output_bits, pq_status, crypto_status);
        }
        
        println!();
    }
    
    Ok(())
}
