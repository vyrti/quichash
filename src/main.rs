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
        Some(Command::Scan { directory, algorithm, output, parallel, fast, format }) => {
            handle_scan_command(&directory, &algorithm, &output, parallel, fast, &format)
        }
        Some(Command::Verify { database, directory }) => {
            handle_verify_command(&database, &directory)
        }
        Some(Command::Benchmark { size_mb }) => {
            handle_benchmark_command(size_mb)
        }
        Some(Command::List) => {
            handle_list_command()
        }
        None => {
            // No subcommand means hash mode (default)
            handle_hash_command(cli.file.as_deref(), cli.text.as_deref(), &cli.algorithms, cli.output.as_deref(), cli.fast)
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
    
    // Format output
    let mut output_lines = Vec::new();
    for result in results {
        output_lines.push(format!("{}  {}", result.hash, result.file_path.display()));
    }
    
    // Write to output destination
    if let Some(output_path) = output {
        // Write to file with better error context
        std::fs::write(output_path, output_lines.join("\n") + "\n").map_err(|e| {
            HashUtilityError::from_io_error(e, "writing output", Some(output_path.to_path_buf()))
        })?;
    } else {
        // Write to stdout
        for line in output_lines {
            println!("{}", line);
        }
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
    let _stats = engine.scan_directory(directory, algorithm, output)?;
    
    Ok(())
}

/// Handle the verify command: compare database with directory
fn handle_verify_command(
    database: &std::path::Path,
    directory: &std::path::Path,
) -> Result<(), HashUtilityError> {
    let engine = VerifyEngine::new();
    
    // Run verification
    let report = engine.verify(database, directory)?;
    
    // Display report
    report.display();
    
    Ok(())
}

/// Handle the benchmark command: run performance tests
fn handle_benchmark_command(size_mb: usize) -> Result<(), HashUtilityError> {
    let engine = BenchmarkEngine::new();
    
    println!("Running benchmarks with {} MB of test data...", size_mb);
    
    // Run benchmarks
    let results = engine.run_benchmarks(size_mb)?;
    
    // Display results
    engine.display_results(&results);
    
    Ok(())
}

/// Handle the list command: display available algorithms
fn handle_list_command() -> Result<(), HashUtilityError> {
    let algorithms = HashRegistry::list_algorithms();
    
    println!("\nAvailable Hash Algorithms:\n");
    println!("{:<20} {:>12} {:>15} {:>15}", "Algorithm", "Output Bits", "Post-Quantum", "Cryptographic");
    println!("{}", "-".repeat(65));
    
    for algo in algorithms {
        let pq_status = if algo.post_quantum { "Yes" } else { "No" };
        let crypto_status = if algo.cryptographic { "Yes" } else { "No" };
        println!("{:<20} {:>12} {:>15} {:>15}", algo.name, algo.output_bits, pq_status, crypto_status);
    }
    
    println!();
    
    Ok(())
}
