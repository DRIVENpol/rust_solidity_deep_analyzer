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

        // ANALYSIS SUMMARY
        md.push_str(&separator);
        md.push('\n');
        md.push_str("**ANALYSIS SUMMARY**\n");
        md.push_str(&separator);
        md.push_str("\n\n");

        // Count statistics
        let total_functions = contract.functions.len();
        let public_external = contract.functions.iter()
            .filter(|f| matches!(f.visibility.as_str(), "public" | "external"))
            .count();
        let total_state_vars = contract.state_variables.len();
        let mutable_vars = contract.state_variables.iter()
            .filter(|v| !v.is_constant && !v.is_immutable)
            .count();

        // Security findings count
        let mut total_security_findings = 0;
        let mut high_severity_findings = 0;

        // Count ignored returns
        for func in &contract.functions {
            for ignored in &func.ignored_returns {
                total_security_findings += 1;
                if ignored.severity == crate::models::IgnoredReturnSeverity::High {
                    high_severity_findings += 1;
                }
            }
        }

        // Count taint flows
        if let Some(dataflow) = &contract.dataflow_analysis {
            total_security_findings += dataflow.taint_flows.len();
            high_severity_findings += dataflow.taint_flows.iter()
                .filter(|f| matches!(f.severity, crate::dataflow::TaintSeverity::Critical | crate::dataflow::TaintSeverity::High))
                .count();
        }

        md.push_str("📊 **Contract Metrics:**\n");
        md.push_str(&format!("   • Functions: {} ({} public/external entry points)\n", total_functions, public_external));
        md.push_str(&format!("   • State Variables: {} ({} mutable)\n", total_state_vars, mutable_vars));
        md.push_str(&format!("   • Events: {}\n", contract.events.len()));
        md.push_str(&format!("   • Modifiers: {}\n", contract.modifiers.len()));
        md.push_str(&format!("   • Custom Errors: {}\n", contract.errors.len()));

        if total_security_findings > 0 {
            md.push_str("\n🔒 **Security Findings:**\n");
            if high_severity_findings > 0 {
                md.push_str(&format!("   • 🔴 {} HIGH/CRITICAL severity issue(s)\n", high_severity_findings));
            }
            md.push_str(&format!("   • Total: {} finding(s) detected\n", total_security_findings));
        } else {
            md.push_str("\n✅ **Security:** No high-severity issues detected\n");
        }

        md.push('\n');
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

                // Read access
                if !var.read_chains.is_empty() {
                    md.push_str("   **Read by:**\n");
                    for (j, chain) in var.read_chains.iter().enumerate() {
                        let is_last = j == var.read_chains.len() - 1;
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

        // SECURITY ANALYSIS
        if let Some(dataflow) = &contract.dataflow_analysis {
            if !dataflow.taint_flows.is_empty() || !dataflow.parameter_influences.is_empty() {
                md.push_str(&separator);
                md.push('\n');
                md.push_str("**SECURITY ANALYSIS**\n");
                md.push_str(&separator);
                md.push('\n');
                md.push('\n');

                // Parameter Influences
                if !dataflow.parameter_influences.is_empty() {
                    md.push_str("### Parameter → State Variable Influences\n\n");
                    md.push_str("Shows how function parameters affect state variables:\n\n");

                    for influence in &dataflow.parameter_influences {
                        md.push_str(&format!("**`{}`** - Parameter `{}`:\n",
                            influence.function_name, influence.param_name));
                        md.push_str("   Influences:\n");
                        for var in &influence.influenced_state_vars {
                            md.push_str(&format!("      • `{}`\n", var));
                        }
                        md.push('\n');
                    }
                }

                // Taint Flows
                if !dataflow.taint_flows.is_empty() {
                    md.push_str("### Data Flow Security Findings\n\n");

                    // Group by severity
                    use std::collections::BTreeMap;
                    let mut by_severity: BTreeMap<String, Vec<&crate::dataflow::TaintFlow>> = BTreeMap::new();

                    for flow in &dataflow.taint_flows {
                        by_severity.entry(flow.severity.as_str().to_string())
                            .or_default()
                            .push(flow);
                    }

                    // Display in severity order (Critical -> Info)
                    for severity in ["CRITICAL", "HIGH", "MEDIUM", "LOW", "INFO"] {
                        if let Some(flows) = by_severity.get(severity) {
                            md.push_str(&format!("#### {} {} Severity\n\n",
                                flows[0].severity.emoji(), severity));

                            for (i, flow) in flows.iter().enumerate() {
                                md.push_str(&format!("{}. **Function:** `{}`\n", i + 1, flow.function_name));
                                md.push_str(&format!("   - **Source:** {}\n", Self::format_taint_source(&flow.source)));
                                md.push_str(&format!("   - **Sink:** {}\n", Self::format_taint_sink(&flow.sink)));
                                if flow.is_validated {
                                    md.push_str("   - **Status:** ✅ Validated\n");
                                } else {
                                    md.push_str("   - **Status:** ⚠️ No validation detected\n");
                                }
                                md.push('\n');
                            }
                        }
                    }
                }
            }
        }

        // IGNORED RETURN VALUES
        let has_ignored_returns = contract.functions.iter()
            .any(|f| !f.ignored_returns.is_empty());

        if has_ignored_returns {
            md.push_str(&separator);
            md.push('\n');
            md.push_str("**IGNORED RETURN VALUES**\n");
            md.push_str(&separator);
            md.push('\n');
            md.push('\n');
            md.push_str("⚠️ **Warning:** The following function calls have return values that are not checked.\n");
            md.push_str("Ignoring return values can lead to silent failures and security vulnerabilities.\n\n");

            // Group by severity
            use std::collections::BTreeMap;
            let mut by_severity: BTreeMap<String, Vec<(&str, &crate::models::IgnoredReturn)>> = BTreeMap::new();

            for func in &contract.functions {
                for ignored in &func.ignored_returns {
                    by_severity.entry(ignored.severity.as_str().to_string())
                        .or_default()
                        .push((&func.name, ignored));
                }
            }

            // Display in severity order (High -> Info)
            for severity in ["HIGH", "MEDIUM", "LOW", "INFO"] {
                if let Some(items) = by_severity.get(severity) {
                    md.push_str(&format!("### {} {} Severity\n\n",
                        items[0].1.severity.emoji(), severity));

                    for (i, (func_name, ignored)) in items.iter().enumerate() {
                        md.push_str(&format!("{}. **In function:** `{}`\n", i + 1, func_name));
                        if ignored.is_external_call {
                            if let Some(target) = &ignored.target_contract {
                                md.push_str(&format!("   - **Ignored call:** `{}.{}()`\n",
                                    target, ignored.called_function));
                            }
                        } else {
                            md.push_str(&format!("   - **Ignored call:** `{}()`\n",
                                ignored.called_function));
                        }

                        // Provide specific recommendations based on function name
                        if ignored.severity == crate::models::IgnoredReturnSeverity::High {
                            md.push_str("   - **Risk:** 🔴 **HIGH** - This can lead to silent failures\n");
                            md.push_str(&format!("   - **Recommendation:** Always check the return value of `{}`\n",
                                ignored.called_function));
                        }
                        md.push('\n');
                    }
                }
            }
        }

        md.push_str(&double_sep);
        md.push('\n');
        md.push_str("*Generated by MainnetReady - Solidity Enhanced Analyzer*\n");

        md
    }

    fn format_taint_source(source: &crate::dataflow::TaintSource) -> String {
        match source {
            crate::dataflow::TaintSource::FunctionParameter { param_name, .. } => {
                format!("Function parameter `{}`", param_name)
            }
            crate::dataflow::TaintSource::MsgSender => "msg.sender".to_string(),
            crate::dataflow::TaintSource::MsgValue => "msg.value".to_string(),
            crate::dataflow::TaintSource::MsgData => "msg.data".to_string(),
            crate::dataflow::TaintSource::ExternalCallReturn { contract_var, function_name } => {
                format!("Return value from `{}.{}()`", contract_var, function_name)
            }
            crate::dataflow::TaintSource::TaintedArrayAccess { base_var } => {
                format!("Array access on `{}`", base_var)
            }
        }
    }

    fn format_taint_sink(sink: &crate::dataflow::TaintSink) -> String {
        match sink {
            crate::dataflow::TaintSink::StateModification { var_name, field_path } => {
                if let Some(path) = field_path {
                    format!("State modification: `{}`", path)
                } else {
                    format!("State modification: `{}`", var_name)
                }
            }
            crate::dataflow::TaintSink::ExternalCall { target_var, function_name } => {
                format!("External call: `{}.{}()`", target_var, function_name)
            }
            crate::dataflow::TaintSink::ValueTransfer { target_expr } => {
                format!("Value transfer to `{}`", target_expr)
            }
            crate::dataflow::TaintSink::DelegateCall { target_expr } => {
                format!("⚠️ DELEGATECALL to `{}`", target_expr)
            }
            crate::dataflow::TaintSink::SelfDestruct { target_expr } => {
                format!("⚠️ SELFDESTRUCT sending to `{}`", target_expr)
            }
            crate::dataflow::TaintSink::ArrayIndex { array_var } => {
                format!("Array index access on `{}`", array_var)
            }
        }
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

                    // Display read chains
                    if !var.read_chains.is_empty() {
                        println!("    {}",  "Read by:".dimmed());
                        Self::print_modification_chains(&var.read_chains);
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
