use crate::models::*;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct RelationshipBuilder;

impl RelationshipBuilder {
    /// Build contract relationships from external calls
    pub fn build_relationships(
        contracts: &[ContractInfo],
        external_calls: Vec<ExternalCall>,
    ) -> Vec<ContractRelation> {
        let mut relations = Vec::new();

        // Create a map of contract name -> contract info for quick lookup
        let contract_map: HashMap<String, &ContractInfo> = contracts
            .iter()
            .map(|c| (c.name.clone(), c))
            .collect();

        for call in external_calls {
            // Try to find the target contract
            let target_contract = call.target_contract.as_ref()
                .and_then(|name| contract_map.get(name));

            if let Some(target_contract) = target_contract {
                // Find the target function in the contract
                let target_func = target_contract.functions.iter()
                    .find(|f| f.name == call.target_function);

                if let Some(target_func) = target_func {
                    // Get modified variables
                    let modified_variables = target_func.modifies_states.clone();

                    // Find other functions that modify the same variables
                    let mut other_modifiers = Vec::new();
                    for var_name in &modified_variables {
                        for func in &target_contract.functions {
                            if func.name != call.target_function
                                && func.modifies_states.contains(var_name) {
                                if !other_modifiers.contains(&func.name) {
                                    other_modifiers.push(func.name.clone());
                                }
                            }
                        }
                    }

                    // Build full call chain with modifications
                    let full_call_chain = Self::build_full_chain_with_modifications(
                        target_func,
                        target_contract,
                        contracts
                    );

                    relations.push(ContractRelation {
                        external_call: call,
                        modified_variables,
                        other_modifiers,
                        full_call_chain,
                    });
                }
            }
        }

        relations
    }

    /// Build the full call chain with state modifications for a function
    fn build_full_chain_with_modifications(
        func: &FunctionDef,
        contract: &ContractInfo,
        all_contracts: &[ContractInfo],
    ) -> Vec<CallChainStep> {
        let mut chain = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // Helper function to find other modifiers for given variables
        let find_other_modifiers = |var_names: &[String], exclude_func: &str, search_contract: &ContractInfo| -> Vec<String> {
            let mut other_mods = Vec::new();
            for var_name in var_names {
                for f in &search_contract.functions {
                    if f.name != exclude_func
                        && f.modifies_states.contains(var_name)
                        && !other_mods.contains(&f.name) {
                        other_mods.push(f.name.clone());
                    }
                }
            }
            other_mods
        };

        // Add the main function with its modifications (but skip it in output)
        visited.insert(format!("{}::{}", contract.name, func.name));

        // Recursively build the call chain
        Self::build_call_chain_recursive(
            func,
            contract,
            all_contracts,
            &mut chain,
            &mut visited,
            &find_other_modifiers,
            1, // depth level
        );

        chain
    }

    /// Recursively build call chain with state modifications
    fn build_call_chain_recursive(
        current_func: &FunctionDef,
        current_contract: &ContractInfo,
        all_contracts: &[ContractInfo],
        chain: &mut Vec<CallChainStep>,
        visited: &mut std::collections::HashSet<String>,
        find_other_modifiers: &dyn Fn(&[String], &str, &ContractInfo) -> Vec<String>,
        depth: usize,
    ) {
        use crate::models::{CallType, ExternalCallInfo};

        let indent = "  ".repeat(depth);

        // Process internal function calls first
        for called_func_name in &current_func.calls_functions {
            let visit_key = format!("{}::{}", current_contract.name, called_func_name);

            // Avoid infinite recursion
            if visited.contains(&visit_key) {
                continue;
            }

            if let Some(called_func) = current_contract.functions.iter().find(|f| f.name == *called_func_name) {
                visited.insert(visit_key);

                // Collect all modifications from this function and its callees
                let mut all_modifications = called_func.modifies_states.clone();
                Self::collect_all_modifications(called_func, current_contract, &mut all_modifications, &mut visited.clone());

                let other_mods = find_other_modifiers(&all_modifications, &called_func.name, current_contract);

                chain.push(CallChainStep {
                    function_name: format!("{}└─> {} ({})", indent, called_func.name, called_func.visibility),
                    call_type: CallType::Internal,
                    modified_variables: all_modifications,
                    other_modifiers: other_mods,
                    target_info: None,
                });

                // Recursively process this function's calls
                Self::build_call_chain_recursive(
                    called_func,
                    current_contract,
                    all_contracts,
                    chain,
                    visited,
                    find_other_modifiers,
                    depth + 1,
                );
            }
        }

        // Process external calls
        for ext_call in &current_func.external_calls {
            let target_contract_name = ext_call.target_contract.as_ref();
            let visit_key = if let Some(tc_name) = target_contract_name {
                format!("{}::{}", tc_name, ext_call.target_function)
            } else {
                format!("unknown::{}", ext_call.target_function)
            };

            // Check if we've already visited this external call
            if visited.contains(&visit_key) {
                continue;
            }

            let call_info = ExternalCallInfo {
                target_variable: ext_call.target_variable.clone(),
                target_type: ext_call.target_type.clone(),
                target_contract: ext_call.target_contract.clone(),
                state_mutability: ext_call.state_mutability.clone(),
            };

            // Try to resolve and expand the external call
            if let Some(tc_name) = target_contract_name {
                if let Some(target_contract) = all_contracts.iter().find(|c| &c.name == tc_name) {
                    if let Some(target_func) = target_contract.functions.iter().find(|f| f.name == ext_call.target_function) {
                        visited.insert(visit_key);

                        // Collect modifications
                        let mut all_modifications = target_func.modifies_states.clone();
                        Self::collect_all_modifications(target_func, target_contract, &mut all_modifications, &mut visited.clone());

                        let other_mods = find_other_modifiers(&all_modifications, &target_func.name, target_contract);

                        chain.push(CallChainStep {
                            function_name: format!("{}└─> {}.{}()", indent, ext_call.target_variable, ext_call.target_function),
                            call_type: CallType::External,
                            modified_variables: all_modifications,
                            other_modifiers: other_mods,
                            target_info: Some(call_info),
                        });

                        // Recursively expand this external call
                        Self::build_call_chain_recursive(
                            target_func,
                            target_contract,
                            all_contracts,
                            chain,
                            visited,
                            find_other_modifiers,
                            depth + 1,
                        );
                        continue;
                    }
                }
            }

            // If we can't resolve, add as unresolved external call
            chain.push(CallChainStep {
                function_name: format!("{}└─> {}.{}()", indent, ext_call.target_variable, ext_call.target_function),
                call_type: CallType::ExternalUnresolved,
                modified_variables: Vec::new(),
                other_modifiers: Vec::new(),
                target_info: Some(call_info),
            });
        }
    }

    /// Collect all state modifications from a function and all functions it calls (recursively)
    fn collect_all_modifications(
        func: &FunctionDef,
        contract: &ContractInfo,
        modifications: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        for called_func_name in &func.calls_functions {
            if visited.contains(called_func_name) {
                continue;
            }
            visited.insert(called_func_name.clone());

            if let Some(called_func) = contract.functions.iter().find(|f| f.name == *called_func_name) {
                // Add this function's modifications
                for var in &called_func.modifies_states {
                    if !modifications.contains(var) {
                        modifications.push(var.clone());
                    }
                }

                // Recursively collect from functions this one calls
                Self::collect_all_modifications(called_func, contract, modifications, visited);
            }
        }
    }

    /// Add state variable modification summary showing all entry points
    fn add_state_modification_summary(md: &mut String, relations: &[ContractRelation]) {
        use std::collections::HashMap;

        // Group by (target_contract, state_variable) -> list of (source_contract, source_function)
        let mut state_var_map: HashMap<(String, String), Vec<(String, String, String)>> = HashMap::new();

        for relation in relations {
            let call = &relation.external_call;

            // Get target contract name
            if let Some(target_contract) = &call.target_contract {
                // For each state variable modified by this call
                for state_var in &relation.modified_variables {
                    let key = (target_contract.clone(), state_var.clone());

                    state_var_map.entry(key)
                        .or_insert_with(Vec::new)
                        .push((
                            call.source_contract.clone(),
                            call.source_function.clone(),
                            call.target_function.clone(),
                        ));
                }
            }
        }

        // Filter to only show state variables modified by multiple entry points
        let multi_entry_vars: Vec<_> = state_var_map.iter()
            .filter(|(_, entry_points)| entry_points.len() > 1)
            .collect();

        if multi_entry_vars.is_empty() {
            return;
        }

        md.push_str("\n");
        md.push_str("## State Variables with Multiple Entry Points\n");
        md.push_str("\n");
        md.push_str("The following state variables can be modified through multiple cross-contract call paths, which may indicate important access control patterns:\n");
        md.push_str("\n");

        // Sort by number of entry points (descending)
        let mut sorted_vars = multi_entry_vars;
        sorted_vars.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for ((target_contract, state_var), entry_points) in sorted_vars {
            md.push_str(&format!("`{}` in contract `{}`\n", state_var, target_contract));
            md.push_str(&format!("   {} entry point(s):\n", entry_points.len()));

            for (source_contract, source_func, target_func) in entry_points {
                md.push_str(&format!("      ├─ `{}.{}()` → `{}()`\n",
                    source_contract, source_func, target_func));
            }
            md.push_str("\n");
        }
    }

    /// Format function name with backticks for syntax highlighting
    fn format_function_with_backticks(func_str: &str) -> String {
        // Check if the string contains visibility marker like "(internal)" or "(external)"
        if let Some(_paren_pos) = func_str.find('(') {
            if func_str.contains("internal") || func_str.contains("external") ||
               func_str.contains("public") || func_str.contains("private") {
                // Split into function name part and visibility part
                let parts: Vec<&str> = func_str.splitn(2, " (").collect();
                if parts.len() == 2 {
                    let visibility = parts[1].trim_end_matches(')');
                    // Wrap function name in backticks, keep visibility in parentheses
                    return format!("`{}` ({})", parts[0].trim(), visibility);
                }
            }
        }
        // If no visibility marker, just wrap the whole thing
        format!("`{}`", func_str.trim())
    }

    /// Generate relations markdown file
    pub fn generate_relations_markdown(
        relations: &[ContractRelation],
        _contracts: &[ContractInfo],
        output_path: &Path,
    ) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut md = String::new();

        // Simple header
        md.push_str("# Cross-Contract Call Analysis\n");
        md.push_str("\n");

        if relations.is_empty() {
            md.push_str("⚠️  No external contract calls detected\n");
            md.push_str("\n");
            fs::write(output_path, md)?;
            return Ok(());
        }

        md.push_str("## Cross-Contract Call Chains\n");
        md.push_str("\n");

        for relation in relations {
            let call = &relation.external_call;

            md.push_str(&format!("`{}.{}()`\n", call.source_contract, call.source_function));

            let target_display = if let Some(ref contract) = call.target_contract {
                format!("{} → {}", call.target_type, contract)
            } else {
                call.target_type.clone()
            };

            md.push_str(&format!("   └─> `{}.{}()` [{}]\n",
                call.target_variable,
                call.target_function,
                target_display
            ));

            // Show modifications made by the target function
            if !relation.modified_variables.is_empty() {
                let vars_formatted = relation.modified_variables.iter()
                    .map(|v| format!("`{}`", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                md.push_str(&format!("          └─> modifies: {}\n", vars_formatted));

                // Show other modifiers
                if !relation.other_modifiers.is_empty() {
                    let mods_formatted = relation.other_modifiers.iter()
                        .map(|m| format!("`{}`", m))
                        .collect::<Vec<_>>()
                        .join(", ");
                    md.push_str(&format!("          └─> also modified by: {}\n", mods_formatted));
                }
            }

            // Show the full call chain with modifications at each step
            if !relation.full_call_chain.is_empty() {
                use crate::models::CallType;

                for step in &relation.full_call_chain {
                    match step.call_type {
                        CallType::Internal => {
                            // Format internal function call
                            let formatted_step = Self::format_function_with_backticks(&step.function_name);
                            md.push_str(&format!("          {}\n", formatted_step));

                            // Show state modifications
                            if !step.modified_variables.is_empty() {
                                let vars_formatted = step.modified_variables.iter()
                                    .map(|v| format!("`{}`", v))
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                md.push_str(&format!("                └─> modifies: {}\n", vars_formatted));

                                // Show other modifiers
                                if !step.other_modifiers.is_empty() {
                                    let mods_formatted = step.other_modifiers.iter()
                                        .map(|m| format!("`{}`", m))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    md.push_str(&format!("                └─> also modified by: {}\n", mods_formatted));
                                }
                            }
                        }
                        CallType::External | CallType::ExternalUnresolved => {
                            if let Some(ref info) = step.target_info {
                                // Format external call with contract resolution
                                let contract_display = if let Some(ref contract) = info.target_contract {
                                    format!("{} → {}", info.target_type, contract)
                                } else {
                                    info.target_type.clone()
                                };

                                // Add state mutability marker for view/pure functions
                                let mutability_marker = if info.state_mutability == "view" || info.state_mutability == "pure" {
                                    format!(" ({})", info.state_mutability)
                                } else {
                                    String::new()
                                };

                                let formatted_step = Self::format_function_with_backticks(&step.function_name);
                                md.push_str(&format!("          {} [{}]{}\n",
                                    formatted_step,
                                    contract_display,
                                    mutability_marker
                                ));

                                // Show state modifications if any
                                if !step.modified_variables.is_empty() {
                                    let vars_formatted = step.modified_variables.iter()
                                        .map(|v| format!("`{}`", v))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    md.push_str(&format!("                └─> modifies: {}\n", vars_formatted));

                                    // Show other modifiers
                                    if !step.other_modifiers.is_empty() {
                                        let mods_formatted = step.other_modifiers.iter()
                                            .map(|m| format!("`{}`", m))
                                            .collect::<Vec<_>>()
                                            .join(", ");
                                        md.push_str(&format!("                └─> also modified by: {}\n", mods_formatted));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            md.push_str("\n");
        }

        // Add state variable modification summary
        Self::add_state_modification_summary(&mut md, &relations);

        // Footer
        md.push_str("\n");
        md.push_str("---\n");
        md.push_str("\n");
        md.push_str("*Generated by MainnetReady - Solidity Enhanced Analyzer*\n");

        fs::write(output_path, md)?;
        Ok(())
    }
}
