use crate::models::*;
use anyhow::Result;
use colored::*;
use prettytable::{Cell, Row, Table};
use std::fs;
use std::path::Path;

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn print_summary(contracts: &[ContractInfo]) {
        println!("\n{}", "🔍 Solidity Contract Analysis Summary".bold().green());
        println!();

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Contract").style_spec("Fb"),
            Cell::new("Variables").style_spec("Fb"),
            Cell::new("Structs").style_spec("Fb"),
            Cell::new("Functions").style_spec("Fb"),
            Cell::new("Events").style_spec("Fb"),
            Cell::new("Upgradeable").style_spec("Fb"),
        ]));

        for contract in contracts {
            let upgradeable_status = if contract.upgradeable_storage.is_some() {
                "✓ ERC-7201".green()
            } else {
                "✗".normal()
            };

            table.add_row(Row::new(vec![
                Cell::new(&contract.name),
                Cell::new(&contract.state_variables.len().to_string()),
                Cell::new(&contract.structs.len().to_string()),
                Cell::new(&contract.functions.len().to_string()),
                Cell::new(&contract.events.len().to_string()),
                Cell::new(&upgradeable_status.to_string()),
            ]));
        }

        table.printstd();
        println!();

        // Print details for upgradeable contracts
        for contract in contracts {
            if let Some(ref upgradeable) = contract.upgradeable_storage {
                println!("{} {}", "🔄 Upgradeable Storage Detected:".bold().cyan(), contract.name.bold());
                println!("   Namespace: {}", upgradeable.namespace);
                println!("   Storage Struct: {}", upgradeable.storage_struct);
                println!("   Storage Slot: {}", upgradeable.storage_slot);
                println!("   Accessor: {}", upgradeable.accessor_function);
                println!("   Fields: {}", upgradeable.struct_fields.len());
                println!();
            }
        }
    }

    pub fn export_json(contracts: &[ContractInfo], path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(contracts)?;
        fs::write(path, json)?;
        println!("{} {}", "✅ Exported to:".green(), path.display());
        Ok(())
    }

    pub fn save_markdown_reports(contracts: &[ContractInfo], output_dir: &Path) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        println!();
        println!("{}", "📝 Saving markdown reports...".bold().green());

        for contract in contracts {
            let filename = format!("{}.md", contract.name);
            let filepath = output_dir.join(&filename);

            let markdown = Self::generate_markdown(contract);
            fs::write(&filepath, markdown)?;

            println!("  {} {}", "✓".green(), filename);
        }

        println!("{} {}", "✅ Reports saved to:".green(), output_dir.display());
        Ok(())
    }

    fn generate_markdown(contract: &ContractInfo) -> String {
        let mut md = String::new();
        let separator = "━".repeat(80);
        let double_sep = "═".repeat(80);

        // Header
        md.push_str(&double_sep);
        md.push('\n');
        md.push_str(&format!("                           **CONTRACT: `{}`**\n", contract.name));
        md.push_str(&double_sep);
        md.push_str("\n\n");
        md.push_str(&format!("**File:** `{}`\n", contract.file_path));

        // Upgradeable Storage Info (if present)
        if let Some(ref upgradeable) = contract.upgradeable_storage {
            md.push_str("\n🔄 **UPGRADEABLE STORAGE DETECTED (ERC-7201)**\n");
            md.push_str(&format!("   **Namespace:** `{}`\n", upgradeable.namespace));
            md.push_str(&format!("   **Storage Struct:** `{}`\n", upgradeable.storage_struct));
            md.push_str(&format!("   **Storage Slot:** `{}`\n", upgradeable.storage_slot));
            md.push_str(&format!("   **Accessor Function:** `{}`\n", upgradeable.accessor_function));
        }

        md.push_str("\n\n");

        // Add note about call chain analysis
        md.push_str(&separator);
        md.push('\n');
        md.push_str("**NOTE:** Call chains show all potential modification paths through static analysis.\n");
        md.push_str("Functions may only modify fields conditionally based on runtime values.\n");
        md.push_str(&separator);
        md.push_str("\n\n");

        // 1. STATE VARIABLES
        if !contract.state_variables.is_empty() {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**STATE VARIABLES**\n");
            md.push_str(&separator);
            md.push_str("\n\n");

            for (i, var) in contract.state_variables.iter().enumerate() {
                if i > 0 {
                    md.push('\n');
                }

                md.push_str(&format!("**`{}`**\n", var.name));

                // Type and attributes with better formatting
                let mut badges = vec![var.visibility.clone()];

                if var.is_constant {
                    badges.push("constant".to_string());
                }
                if var.is_immutable {
                    badges.push("immutable".to_string());
                }

                md.push_str(&format!("   **Type:** `{}`\n", var.var_type));
                md.push_str(&format!("   **Visibility:** {}\n", badges.join(", ")));

                // Modifications
                if !var.modification_chains.is_empty() {
                    md.push_str("\n   **Modified by:**\n");
                    for (j, chain) in var.modification_chains.iter().enumerate() {
                        let is_last = j == var.modification_chains.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };

                        if chain.call_chain.is_empty() {
                            md.push_str(&format!("      {} `{}` *({})*\n",
                                prefix,
                                chain.direct_modifier,
                                chain.direct_modifier_visibility
                            ));
                        } else {
                            let mut parts = vec![format!("`{}` *({})*",
                                chain.direct_modifier,
                                chain.direct_modifier_visibility
                            )];
                            for caller in &chain.call_chain {
                                parts.push(format!("`{}` *({})*",
                                    caller.function_name,
                                    caller.visibility
                                ));
                            }
                            md.push_str(&format!("      {} {}\n", prefix, parts.join(" ← ")));
                        }
                    }
                    md.push('\n');
                } else if !var.is_constant && !var.is_immutable {
                    md.push_str("\n   **Modified by:** *None*\n\n");
                }
            }

            md.push('\n');
        }

        // 2. EVENTS
        if !contract.events.is_empty() {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**EVENTS**\n");
            md.push_str(&separator);
            md.push_str("\n\n");

            for (i, event) in contract.events.iter().enumerate() {
                if i > 0 {
                    md.push('\n');
                }

                let params: Vec<String> = event.parameters.iter()
                    .map(|p| {
                        let indexed = if p.indexed { " *(indexed)*" } else { "" };
                        format!("`{}` {}{}", p.param_type, p.name, indexed)
                    })
                    .collect();

                md.push_str(&format!("**`{}`**\n", event.name));

                if !params.is_empty() {
                    md.push_str(&format!("   **Parameters:** {}\n", params.join(", ")));
                }

                if !event.emitted_in.is_empty() {
                    md.push_str("\n   **Emitted in:**\n");
                    for (j, func) in event.emitted_in.iter().enumerate() {
                        let is_last = j == event.emitted_in.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };
                        md.push_str(&format!("      {} `{}`\n", prefix, func));
                    }
                    md.push('\n');
                } else {
                    md.push_str("\n   **Emitted in:** *None*\n\n");
                }
            }

            md.push('\n');
        }

        // 3. MODIFIERS
        if !contract.modifiers.is_empty() {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**MODIFIERS**\n");
            md.push_str(&separator);
            md.push_str("\n\n");

            for (i, modifier) in contract.modifiers.iter().enumerate() {
                if i > 0 {
                    md.push('\n');
                }

                md.push_str(&format!("**`{}({})`**\n",
                    modifier.name,
                    modifier.parameters.join(", ")
                ));

                if !modifier.used_in.is_empty() {
                    md.push_str("\n   **Used in:**\n");
                    for (j, func) in modifier.used_in.iter().enumerate() {
                        let is_last = j == modifier.used_in.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };
                        md.push_str(&format!("      {} `{}`\n", prefix, func));
                    }
                    md.push('\n');
                } else {
                    md.push_str("\n   **Used in:** *None*\n\n");
                }
            }

            md.push('\n');
        }

        // 4. CUSTOM ERRORS
        if !contract.errors.is_empty() {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**CUSTOM ERRORS**\n");
            md.push_str(&separator);
            md.push_str("\n\n");

            for (i, error) in contract.errors.iter().enumerate() {
                if i > 0 {
                    md.push('\n');
                }

                let params: Vec<String> = error.parameters.iter()
                    .map(|p| format!("`{}` {}", p.param_type, p.name))
                    .collect();

                // Add marker for inherited errors
                if error.is_inherited {
                    md.push_str(&format!("**`{}`** *(inherited)*\n", error.name));
                } else {
                    md.push_str(&format!("**`{}`**\n", error.name));
                }

                if !params.is_empty() {
                    md.push_str(&format!("   **Parameters:** {}\n", params.join(", ")));
                }

                if !error.used_in.is_empty() {
                    md.push_str("\n   **Used in:**\n");
                    for (j, func) in error.used_in.iter().enumerate() {
                        let is_last = j == error.used_in.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };
                        md.push_str(&format!("      {} `{}`\n", prefix, func));
                    }
                    md.push('\n');
                } else {
                    md.push_str("\n   **Used in:** *None*\n\n");
                }
            }

            md.push('\n');
        }

        // 5. FUNCTIONS
        if !contract.functions.is_empty() {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**FUNCTIONS**\n");
            md.push_str(&separator);
            md.push_str("\n\n");

            for (i, func) in contract.functions.iter().enumerate() {
                if i > 0 {
                    md.push('\n');
                }

                let returns_str = if func.returns.is_empty() {
                    String::new()
                } else {
                    format!(" → `{}`", func.returns.join(", "))
                };

                let unchecked = if func.has_unchecked { " *[unchecked]*" } else { "" };

                md.push_str(&format!("**`{}({})`**{}\n",
                    func.name,
                    func.parameters.join(", "),
                    returns_str
                ));

                md.push_str(&format!("   **Visibility:** {}\n", func.visibility));
                md.push_str(&format!("   **State Mutability:** {}{}\n",
                    func.state_mutability,
                    unchecked
                ));
                md.push_str(&format!("   **Line:** {}\n", func.line_number));

                if !func.uses_modifiers.is_empty() {
                    md.push_str("\n   **Modifiers:**\n");
                    for (j, modifier) in func.uses_modifiers.iter().enumerate() {
                        let is_last = j == func.uses_modifiers.len() - 1;
                        let prefix = if is_last { "└─" } else { "├─" };
                        md.push_str(&format!("      {} `{}`\n", prefix, modifier));
                    }
                }

                md.push('\n');
            }
        }

        md.push_str(&double_sep);
        md.push('\n');
        md.push_str("*Generated by MainnetReady - Solidity Enhanced Analyzer*\n");

        md
    }

    pub fn print_detailed(contracts: &[ContractInfo]) {
        for contract in contracts {
            println!("\n{}", "═".repeat(70).bright_blue());
            println!("{} {}", "📜 Contract:".bold(), contract.name.bright_cyan().bold());
            println!("{} {}", "📁 File:".bold(), contract.file_path.dimmed());

            // 1. STATE VARIABLES & THEIR MODIFYING FUNCTIONS
            if !contract.state_variables.is_empty() {
                println!("\n{}", "📦 STATE VARIABLES & MODIFICATIONS:".bold().yellow());
                for var in &contract.state_variables {
                    let mut attrs = Vec::new();
                    attrs.push(var.var_type.clone());
                    attrs.push(var.visibility.clone());
                    if var.is_constant {
                        attrs.push("constant".to_string());
                    }
                    if var.is_immutable {
                        attrs.push("immutable".to_string());
                    }
                    println!("\n  {} {}", "•".bright_blue().bold(), var.name.white().bold());
                    println!("    Type: {}", attrs.join(", ").green());

                    // Display modification chains
                    if !var.modification_chains.is_empty() {
                        println!("    {}",  "Modified by:".dimmed());
                        Self::print_modification_chains(&var.modification_chains);
                    } else if !var.is_constant && !var.is_immutable {
                        println!("    {} {}", "└─".dimmed(), "No modifications detected".dimmed());
                    }
                }
            }

            // 2. EVENTS & WHERE THEY'RE EMITTED
            if !contract.events.is_empty() {
                println!("\n{}", "📢 EVENTS & EMISSIONS:".bold().yellow());
                for event in &contract.events {
                    let params: Vec<String> = event.parameters.iter()
                        .map(|p| {
                            let indexed = if p.indexed { " indexed" } else { "" };
                            format!("{}{} {}", p.param_type, indexed, p.name)
                        })
                        .collect();
                    println!("\n  {} {}({})",
                        "•".bright_blue().bold(),
                        event.name.white().bold(),
                        params.join(", ").cyan()
                    );

                    if !event.emitted_in.is_empty() {
                        println!("    {}","Emitted in:".dimmed());
                        for func_name in &event.emitted_in {
                            println!("      {} {}", "→".cyan(), func_name.white());
                        }
                    } else {
                        println!("    {} {}", "└─".dimmed(), "Never emitted".dimmed());
                    }
                }
            }

            // 3. MODIFIERS & FUNCTIONS USING THEM
            if !contract.modifiers.is_empty() {
                println!("\n{}", "🛡️  MODIFIERS & USAGE:".bold().yellow());
                for modifier in &contract.modifiers {
                    println!("\n  {} {}({})",
                        "•".bright_blue().bold(),
                        modifier.name.white().bold(),
                        modifier.parameters.join(", ").cyan()
                    );

                    if !modifier.used_in.is_empty() {
                        println!("    {}", "Used in:".dimmed());
                        for func_name in &modifier.used_in {
                            println!("      {} {}", "→".cyan(), func_name.white());
                        }
                    } else {
                        println!("    {} {}", "└─".dimmed(), "Never used".dimmed());
                    }
                }
            }

            // 4. CUSTOM ERRORS & FUNCTIONS USING THEM
            if !contract.errors.is_empty() {
                println!("\n{}", "⚠️  CUSTOM ERRORS & USAGE:".bold().yellow());
                for error in &contract.errors {
                    let params: Vec<String> = error.parameters.iter()
                        .map(|p| format!("{} {}", p.param_type, p.name))
                        .collect();
                    println!("\n  {} {}({})",
                        "•".bright_blue().bold(),
                        error.name.white().bold(),
                        params.join(", ").red()
                    );

                    if !error.used_in.is_empty() {
                        println!("    {}", "Used in:".dimmed());
                        for func_name in &error.used_in {
                            println!("      {} {}", "→".cyan(), func_name.white());
                        }
                    } else {
                        println!("    {} {}", "└─".dimmed(), "Never used".dimmed());
                    }
                }
            }

            // 5. ALL FUNCTIONS LIST
            if !contract.functions.is_empty() {
                println!("\n{}", "⚙️  FUNCTIONS:".bold().yellow());
                for func in &contract.functions {
                    let returns_str = if func.returns.is_empty() {
                        String::new()
                    } else {
                        format!(" → {}", func.returns.join(", "))
                    };

                    let unchecked_mark = if func.has_unchecked {
                        " [unchecked]".red().to_string()
                    } else {
                        String::new()
                    };

                    println!("\n  {} {}({}){}",
                        "•".bright_blue().bold(),
                        func.name.white().bold(),
                        func.parameters.join(", ").cyan(),
                        returns_str.green()
                    );

                    println!("    {} {} | {}{}",
                        "Visibility:".dimmed(),
                        func.visibility.magenta(),
                        func.state_mutability.yellow(),
                        unchecked_mark
                    );

                    if !func.uses_modifiers.is_empty() {
                        println!("    {} {}",
                            "Modifiers:".dimmed(),
                            func.uses_modifiers.join(", ").cyan()
                        );
                    }
                }
            }
        }
        println!();
    }

    fn print_modification_chains(chains: &[ModificationChain]) {
        for (i, chain) in chains.iter().enumerate() {
            let is_last = i == chains.len() - 1;
            let prefix = if is_last { "└─" } else { "├─" };

            if chain.call_chain.is_empty() {
                // Direct modification only
                println!("      {} {} ({})",
                    prefix.cyan(),
                    chain.direct_modifier.white(),
                    chain.direct_modifier_visibility.magenta()
                );
            } else {
                // Build the full chain: DirectModifier ← Caller1 ← Caller2
                let mut parts = vec![
                    format!("{} ({})",
                        chain.direct_modifier.white(),
                        chain.direct_modifier_visibility.magenta()
                    )
                ];

                for caller in &chain.call_chain {
                    parts.push(format!("{} ({})",
                        caller.function_name.white(),
                        caller.visibility.magenta()
                    ));
                }

                println!("      {} {}",
                    prefix.cyan(),
                    parts.join(&format!(" {} ", "←".dimmed()))
                );
            }
        }
    }
}
