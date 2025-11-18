mod cli;
mod hash;
mod scan;
mod verify;
mod benchmark;
mod database;
mod path_utils;
mod error;

use cli::{parse_args, Command};
use hash::{HashComputer, HashRegistry};
use scan::ScanEngine;
use verify::VerifyEngine;
use benchmark::BenchmarkEngine;
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
        Command::Hash { file, algorithms, output } => {
            handle_hash_command(&file, &algorithms, output.as_deref())
        }
        Command::Scan { directory, algorithm, output, parallel } => {
            handle_scan_command(&directory, &algorithm, &output, parallel)
        }
        Command::Verify { database, directory } => {
            handle_verify_command(&database, &directory)
        }
        Command::Benchmark { size_mb } => {
            handle_benchmark_command(size_mb)
        }
        Command::List => {
            handle_list_command()
        }
    };
    
    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Handle the hash command: compute and display hash(es) for a file
fn handle_hash_command(
    file: &std::path::Path,
    algorithms: &[String],
    output: Option<&std::path::Path>,
) -> Result<(), HashUtilityError> {
    let computer = HashComputer::new();
    
    // Compute hashes for all specified algorithms
    let results = computer.compute_multiple_hashes(file, algorithms)?;
    
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
) -> Result<(), HashUtilityError> {
    let engine = ScanEngine::with_parallel(parallel);
    
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
    println!("{:<20} {:>12} {:>15}", "Algorithm", "Output Bits", "Post-Quantum");
    println!("{}", "-".repeat(50));
    
    for algo in algorithms {
        let pq_status = if algo.post_quantum { "Yes" } else { "No" };
        println!("{:<20} {:>12} {:>15}", algo.name, algo.output_bits, pq_status);
    }
    
    println!();
    
    Ok(())
}
