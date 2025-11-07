use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod models;
mod scanner;
mod parser;
mod analyzer;
mod output;
mod relations;

use scanner::FileScanner;
use parser::SolidityParser;
use output::OutputFormatter;
use relations::RelationshipBuilder;
use analyzer::StateModificationAnalyzer;

#[derive(Parser)]
#[command(name = "sol-analyzer")]
#[command(about = "Analyze Solidity contracts and extract metadata", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze contracts in the specified directory
    Analyze {
        /// Path to contracts directory
        #[arg(short, long, default_value = "./contracts")]
        path: PathBuf,

        /// Output format (json, table, detailed)
        #[arg(short, long, default_value = "detailed")]
        format: String,

        /// Export to JSON file
        #[arg(short, long)]
        export: Option<PathBuf>,

        /// Save markdown reports for each contract
        #[arg(long, default_value = "true")]
        save_md: bool,

        /// Output directory for markdown files (default: ./reports)
        #[arg(long, default_value = "./reports")]
        md_output: PathBuf,

        /// Generate contract relationships map
        #[arg(long, default_value = "true")]
        relations: bool,

        /// Output file for relations (default: ./reports/0_relations/relations.md)
        #[arg(long, default_value = "./reports/0_relations/relations.md")]
        relations_output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, format, export, save_md, md_output, relations, relations_output } => {
            analyze_contracts(path, format, export, save_md, md_output, relations, relations_output)?;
        }
    }

    Ok(())
}

fn analyze_contracts(
    path: PathBuf,
    format: String,
    export: Option<PathBuf>,
    save_md: bool,
    md_output: PathBuf,
    relations: bool,
    relations_output: PathBuf,
) -> Result<()> {
    println!("{}", "🚀 Starting Solidity contract analysis...".bold());
    println!();

    // Scan for .sol files
    let scanner = FileScanner::new(path.clone());
    let sol_files = scanner.scan_contracts()?;

    if sol_files.is_empty() {
        println!("{}", "⚠️  No Solidity files found!".yellow());
        println!("Looking in: {}", path.display());
        return Ok(());
    }

    println!("{} {} Solidity file(s)", "📂 Found".green(), sol_files.len());

    // Parse each file and store ASTs for relationship analysis
    let mut all_contracts = Vec::new();
    let mut contract_asts = Vec::new();
    let mut errors = Vec::new();

    for file in &sol_files {
        match SolidityParser::parse_file_with_ast(file) {
            Ok((contracts, asts)) => {
                println!("  {} {}", "✓".green(), file.display());
                all_contracts.extend(contracts);
                contract_asts.extend(asts);
            }
            Err(e) => {
                println!("  {} {} - {}", "✗".red(), file.display(), e);
                errors.push((file.clone(), e));
            }
        }
    }

    println!();

    if all_contracts.is_empty() {
        println!("{}", "⚠️  No contracts found or all files had errors!".yellow());
        return Ok(());
    }

    // Output results
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&all_contracts)?;
            println!("{}", json);
        }
        "detailed" => {
            OutputFormatter::print_detailed(&all_contracts);
        }
        _ => {
            OutputFormatter::print_summary(&all_contracts);
        }
    }

    // Export if requested
    if let Some(export_path) = export {
        OutputFormatter::export_json(&all_contracts, &export_path)?;
    }

    // Save markdown reports if requested
    if save_md {
        OutputFormatter::save_markdown_reports(&all_contracts, &md_output)?;
    }

    // Generate contract relationships if requested
    if relations {
        println!();
        println!("{}", "🔗 Analyzing contract relationships...".bold());

        // Detect all external calls
        let mut all_external_calls = Vec::new();
        for i in 0..all_contracts.len() {
            if i < contract_asts.len() {
                // Create a temporary snapshot of all_contracts for lookup
                let contracts_snapshot: Vec<_> = all_contracts.iter()
                    .map(|c| c.clone())
                    .collect();

                let external_calls = StateModificationAnalyzer::detect_external_calls(
                    &mut all_contracts[i],
                    &contract_asts[i],
                    &contracts_snapshot,
                );
                all_external_calls.extend(external_calls);
            }
        }

        if all_external_calls.is_empty() {
            println!("  {} No external contract calls detected", "ℹ️".blue());
        } else {
            println!("  {} {} external call(s) detected", "✓".green(), all_external_calls.len());

            // Build relationships
            let relationships = RelationshipBuilder::build_relationships(&all_contracts, all_external_calls);

            // Generate markdown
            RelationshipBuilder::generate_relations_markdown(&relationships, &all_contracts, &relations_output)?;
            println!("{} {}", "✅ Relations saved to:".green(), relations_output.display());
        }
    }

    // Print error summary if any
    if !errors.is_empty() {
        println!();
        println!("{} {} file(s) had errors", "⚠️".yellow(), errors.len());
    }

    Ok(())
}
