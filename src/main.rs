use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod models;
mod scanner;
mod parser;
mod analyzer;
mod output;
mod dataflow;
mod state_var_report;
mod call_graph_report;
mod json_reports;
mod graph_generator;
mod contract_interaction_reports;

use scanner::FileScanner;
use parser::SolidityParser;
use output::OutputFormatter;
use analyzer::StateModificationAnalyzer;
use state_var_report::StateVarReportGenerator;
use call_graph_report::CallGraphReportGenerator;
use json_reports::JsonReportGenerator;
use graph_generator::GraphGenerator;
use contract_interaction_reports::ContractInteractionReports;

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
        #[arg(short, long, default_value = "table")]
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, format, export, save_md, md_output, relations } => {
            analyze_contracts(path, format, export, save_md, md_output, relations)?;
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
) -> Result<()> {
    println!("{}", "🚀 Starting Solidity contract analysis...".bold());
    println!();

    // Clean up previous reports
    if save_md && md_output.exists() {
        println!("{}", "🧹 Cleaning up old reports...".dimmed());
        std::fs::remove_dir_all(&md_output)?;
        println!("  {} Removed old reports directory", "✓".green());
    }

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
                let contracts_snapshot: Vec<_> = all_contracts.to_vec();

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
        }

        // Set up output directory
        let relations_dir = md_output.join("0_relations");
        std::fs::create_dir_all(&relations_dir)?;

        // Generate state variable access report
        let state_var_report = StateVarReportGenerator::generate_report(&all_contracts);
        let state_var_output = relations_dir.join("state_variables.md");
        std::fs::write(&state_var_output, state_var_report)?;
        println!("{} {}", "✅ State variables report saved to:".green(), state_var_output.display());

        // Generate function call graph report
        let call_graph_report = CallGraphReportGenerator::generate_report(&all_contracts);
        let call_graph_output = relations_dir.join("function_calls.md");
        std::fs::write(&call_graph_output, call_graph_report)?;
        println!("{} {}", "✅ Function calls report saved to:".green(), call_graph_output.display());

        // Generate JSON reports
        // Function calls JSON
        let function_calls_json = JsonReportGenerator::generate_function_calls_json(&all_contracts);
        let function_calls_json_path = relations_dir.join("function_calls.json");
        JsonReportGenerator::save_json(&function_calls_json, &function_calls_json_path)?;
        println!("{} {}", "✅ Function calls JSON saved to:".green(), function_calls_json_path.display());

        // State variables JSON
        let state_vars_json = JsonReportGenerator::generate_state_variables_json(&all_contracts);
        let state_vars_json_path = relations_dir.join("state_variables.json");
        JsonReportGenerator::save_json(&state_vars_json, &state_vars_json_path)?;
        println!("{} {}", "✅ State variables JSON saved to:".green(), state_vars_json_path.display());

        // Generate DOT graph files
        let function_call_dot = relations_dir.join("function_calls.dot");
        GraphGenerator::generate_function_call_graph(&all_contracts, &function_call_dot)?;
        println!("{} {}", "✅ Function call graph (DOT) saved to:".green(), function_call_dot.display());

        let state_var_dot = relations_dir.join("state_variables.dot");
        GraphGenerator::generate_state_variable_graph(&all_contracts, &state_var_dot)?;
        println!("{} {}", "✅ State variable graph (DOT) saved to:".green(), state_var_dot.display());

        let contract_interaction_dot = relations_dir.join("contract_interactions.dot");
        GraphGenerator::generate_contract_interaction_graph(&all_contracts, &contract_interaction_dot)?;
        println!("{} {}", "✅ Contract interaction graph (DOT) saved to:".green(), contract_interaction_dot.display());

        let cross_contract_state_dot = relations_dir.join("cross_contract_state_dependencies.dot");
        GraphGenerator::generate_cross_contract_state_dependencies(&all_contracts, &cross_contract_state_dot)?;
        println!("{} {}", "✅ Cross-contract state dependencies (DOT) saved to:".green(), cross_contract_state_dot.display());

        // Generate contract interaction reports (MD and JSON)
        let contract_interactions_md = ContractInteractionReports::generate_contract_interactions_md(&all_contracts);
        let contract_interactions_md_path = relations_dir.join("contract_interactions.md");
        std::fs::write(&contract_interactions_md_path, contract_interactions_md)?;
        println!("{} {}", "✅ Contract interactions (MD) saved to:".green(), contract_interactions_md_path.display());

        let contract_interactions_json = ContractInteractionReports::generate_contract_interactions_json(&all_contracts);
        let contract_interactions_json_path = relations_dir.join("contract_interactions.json");
        ContractInteractionReports::save_json(&contract_interactions_json, &contract_interactions_json_path)?;
        println!("{} {}", "✅ Contract interactions (JSON) saved to:".green(), contract_interactions_json_path.display());

        // Generate cross-contract state dependency reports (MD and JSON)
        let cross_state_deps_md = ContractInteractionReports::generate_cross_contract_state_dependencies_md(&all_contracts);
        let cross_state_deps_md_path = relations_dir.join("cross_contract_state_dependencies.md");
        std::fs::write(&cross_state_deps_md_path, cross_state_deps_md)?;
        println!("{} {}", "✅ Cross-contract state dependencies (MD) saved to:".green(), cross_state_deps_md_path.display());

        let cross_state_deps_json = ContractInteractionReports::generate_cross_contract_state_dependencies_json(&all_contracts);
        let cross_state_deps_json_path = relations_dir.join("cross_contract_state_dependencies.json");
        ContractInteractionReports::save_json(&cross_state_deps_json, &cross_state_deps_json_path)?;
        println!("{} {}", "✅ Cross-contract state dependencies (JSON) saved to:".green(), cross_state_deps_json_path.display());
    }

    // Print error summary if any
    if !errors.is_empty() {
        println!();
        println!("{} {} file(s) had errors", "⚠️".yellow(), errors.len());
    }

    Ok(())
}
