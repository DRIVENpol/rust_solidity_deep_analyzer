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

                    // Get field-level modifications
                    let modified_fields = target_func.modifies_state_fields.clone();

                    // Find other functions that modify the same variables (old global list)
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

                    // Build per-field "also modified by" map
                    let field_modifiers = Self::build_field_modifiers_map(
                        &modified_fields,
                        &call.target_function,
                        target_contract
                    );

                    // Build full call chain with modifications
                    let full_call_chain = Self::build_full_chain_with_modifications(
                        target_func,
                        target_contract,
                        contracts
                    );

                    relations.push(ContractRelation {
                        external_call: call,
                        modified_variables,
                        modified_fields,
                        field_modifiers,
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

                // Collect field-level modifications and resolve storage parameter fields
                let mut modified_fields = called_func.modifies_state_fields.clone();

                // Resolve @param to actual state variables at call site
                if !called_func.storage_params.is_empty() {
                    modified_fields = Self::resolve_storage_param_fields(
                        &modified_fields,
                        current_func,
                        &called_func.name,
                        current_contract
                    );
                }

                // Build per-field "also modified by" map
                let field_modifiers = Self::build_field_modifiers_map(
                    &modified_fields,
                    &called_func.name,
                    current_contract
                );

                chain.push(CallChainStep {
                    function_name: format!("{}└─> {} ({})", indent, called_func.name, called_func.visibility),
                    call_type: CallType::Internal,
                    modified_variables: all_modifications,
                    modified_fields,
                    field_modifiers,
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

                        // Collect field-level modifications
                        let modified_fields = target_func.modifies_state_fields.clone();

                        // Build per-field "also modified by" map
                        let field_modifiers = Self::build_field_modifiers_map(
                            &modified_fields,
                            &target_func.name,
                            target_contract
                        );

                        chain.push(CallChainStep {
                            function_name: format!("{}└─> {}.{}()", indent, ext_call.target_variable, ext_call.target_function),
                            call_type: CallType::External,
                            modified_variables: all_modifications,
                            modified_fields,
                            field_modifiers,
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
                modified_fields: Vec::new(),
                field_modifiers: std::collections::HashMap::new(),
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

    /// Add field-level modification summary showing all entry points
    fn add_state_modification_summary(md: &mut String, relations: &[ContractRelation], contracts: &[ContractInfo]) {
        use std::collections::HashMap;

        // Group by (target_contract, field) -> list of (source_contract, source_function, target_function)
        let mut state_var_map: HashMap<(String, String), Vec<(String, String, String)>> = HashMap::new();

        for relation in relations {
            let call = &relation.external_call;

            // Get target contract name
            if let Some(target_contract) = &call.target_contract {
                // Collect all fields modified in this call chain (including internal functions)
                let mut all_fields = std::collections::HashSet::new();

                // Add fields from the top-level function
                if !relation.modified_fields.is_empty() {
                    for field in &relation.modified_fields {
                        all_fields.insert(field.clone());
                    }
                } else {
                    for var in &relation.modified_variables {
                        all_fields.insert(var.clone());
                    }
                }

                // Add fields from all steps in the call chain (internal functions)
                for step in &relation.full_call_chain {
                    if !step.modified_fields.is_empty() {
                        for field in &step.modified_fields {
                            all_fields.insert(field.clone());
                        }
                    } else {
                        for var in &step.modified_variables {
                            all_fields.insert(var.clone());
                        }
                    }
                }

                // Track each field with its entry point
                for field in all_fields {
                    let key = (target_contract.clone(), field);

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

        // Add upgradeable storage fields with multiple entry points
        let upgradeable_entry_points = Self::collect_upgradeable_storage_entry_points(contracts);

        // Merge upgradeable storage entry points into state_var_map
        for ((contract_name, field_name), entry_points) in upgradeable_entry_points {
            let key = (contract_name, field_name);
            state_var_map.insert(key, entry_points);
        }

        // Filter to only show fields modified by multiple entry points
        let multi_entry_fields: Vec<_> = state_var_map.iter()
            .filter(|(_, entry_points)| entry_points.len() > 1)
            .collect();

        if multi_entry_fields.is_empty() {
            return;
        }

        md.push_str("\n");
        md.push_str("## **Fields with Multiple Entry Points**\n");
        md.push_str("\n");
        md.push_str("*The following fields can be modified through multiple call paths, which may indicate important access control patterns:*\n");
        md.push_str("\n");

        // Sort by number of entry points (descending)
        let mut sorted_fields = multi_entry_fields;
        sorted_fields.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for ((target_contract, field), entry_points) in sorted_fields {
            md.push_str(&format!("**`{}`** in contract **`{}`**\n", field, target_contract));
            md.push_str(&format!("   *{} entry point(s):*\n", entry_points.len()));

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
        contracts: &[ContractInfo],
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
        md.push_str("**NOTE:** *Call chains show all potential modification paths identified through static analysis. ");
        md.push_str("Some functions may only modify fields conditionally based on runtime values (e.g., `if (from == address(0))`).*\n");
        md.push_str("\n");

        // Section 1: Cross-Contract Call Chains (including upgradeable storage internal chains)
        md.push_str("## **Cross-Contract Call Chains**\n");
        md.push_str("\n");

        // Add upgradeable storage internal call chains first
        let upgradeable_contracts: Vec<_> = contracts.iter()
            .filter(|c| c.upgradeable_storage.is_some())
            .collect();

        if !upgradeable_contracts.is_empty() {
            for contract in &upgradeable_contracts {
                // Get all state variables that are from the storage struct
                let storage_vars: Vec<_> = contract.state_variables.iter()
                    .filter(|v| v.var_type.contains("(upgradeable storage)"))
                    .collect();

                for var in storage_vars {
                    // Add emoji directly to the field name for unified format
                    md.push_str(&format!("**`{}.{}`** 🔄\n", contract.name, var.name));

                    if var.modification_chains.is_empty() {
                        md.push_str("   └─> *No modifications detected*\n");
                    } else {
                        for (idx, chain) in var.modification_chains.iter().enumerate() {
                            let is_last = idx == var.modification_chains.len() - 1;
                            let prefix = if is_last { "└─>" } else { "├─>" };

                            // Build the call chain display
                            let mut chain_parts = vec![
                                format!("`{}` *({})*", chain.direct_modifier, chain.direct_modifier_visibility)
                            ];

                            for caller in &chain.call_chain {
                                chain_parts.push(format!("`{}` *({})*", caller.function_name, caller.visibility));
                            }

                            md.push_str(&format!("   {} {}\n", prefix, chain_parts.join(" ← ")));
                        }
                    }

                    md.push_str("\n");
                }
            }
        }

        // Add external contract calls
        if relations.is_empty() {
            if upgradeable_contracts.is_empty() {
                md.push_str("⚠️  No contract calls detected\n");
            }
            md.push_str("\n");
            fs::write(output_path, md)?;
            return Ok(());
        }

        for relation in relations {
            let call = &relation.external_call;

            md.push_str(&format!("**`{}.{}()`**\n", call.source_contract, call.source_function));

            let target_display = if let Some(ref contract) = call.target_contract {
                format!("{} → {}", call.target_type, contract)
            } else {
                call.target_type.clone()
            };

            md.push_str(&format!("   └─> `{}.{}()` *[{}]*\n",
                call.target_variable,
                call.target_function,
                target_display
            ));

            // Show modifications made by the target function in tree format
            if !relation.modified_fields.is_empty() {
                md.push_str("          └─> **modifies:**\n");

                let field_count = relation.modified_fields.len();
                for (idx, field) in relation.modified_fields.iter().enumerate() {
                    let is_last = idx == field_count - 1;
                    let tree_char = if is_last { "└─" } else { "├─" };
                    let continuation = if is_last { "   " } else { "│  " };

                    md.push_str(&format!("              {} `{}`\n", tree_char, field));

                    // Show per-field "also modified by" list
                    if let Some(modifiers) = relation.field_modifiers.get(field) {
                        if !modifiers.is_empty() {
                            let mods_formatted = modifiers.iter()
                                .map(|m| format!("`{}`", m))
                                .collect::<Vec<_>>()
                                .join(", ");
                            md.push_str(&format!("              {} └─ *also modified by:* {}\n", continuation, mods_formatted));
                        }
                    }
                }
            } else if !relation.modified_variables.is_empty() {
                // Fallback to base state variables if no field-level info
                let vars_formatted = relation.modified_variables.iter()
                    .map(|v| format!("`{}`", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                md.push_str(&format!("          └─> **modifies:** {}\n", vars_formatted));

                // Show other modifiers
                if !relation.other_modifiers.is_empty() {
                    let mods_formatted = relation.other_modifiers.iter()
                        .map(|m| format!("`{}`", m))
                        .collect::<Vec<_>>()
                        .join(", ");
                    md.push_str(&format!("          └─> *also modified by:* {}\n", mods_formatted));
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

                            // Show field-level modifications in tree format if available
                            if !step.modified_fields.is_empty() {
                                md.push_str("                └─> **modifies:**\n");

                                let field_count = step.modified_fields.len();
                                for (idx, field) in step.modified_fields.iter().enumerate() {
                                    let is_last = idx == field_count - 1;
                                    let tree_char = if is_last { "└─" } else { "├─" };
                                    let continuation = if is_last { "   " } else { "│  " };

                                    md.push_str(&format!("                    {} `{}`\n", tree_char, field));

                                    // Show per-field "also modified by" list
                                    if let Some(modifiers) = step.field_modifiers.get(field) {
                                        if !modifiers.is_empty() {
                                            let mods_formatted = modifiers.iter()
                                                .map(|m| format!("`{}`", m))
                                                .collect::<Vec<_>>()
                                                .join(", ");
                                            md.push_str(&format!("                    {} └─ *also modified by:* {}\n", continuation, mods_formatted));
                                        }
                                    }
                                }
                            } else if !step.modified_variables.is_empty() {
                                // Fallback to base state variable if no field info
                                let vars_formatted = step.modified_variables.iter()
                                    .map(|v| format!("`{}`", v))
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                md.push_str(&format!("                └─> **modifies:** {}\n", vars_formatted));

                                // Show other modifiers
                                if !step.other_modifiers.is_empty() {
                                    let mods_formatted = step.other_modifiers.iter()
                                        .map(|m| format!("`{}`", m))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    md.push_str(&format!("                └─> *also modified by:* {}\n", mods_formatted));
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

                                // Show field-level modifications in tree format if available
                                if !step.modified_fields.is_empty() {
                                    md.push_str("                └─> modifies:\n");

                                    let field_count = step.modified_fields.len();
                                    for (idx, field) in step.modified_fields.iter().enumerate() {
                                        let is_last = idx == field_count - 1;
                                        let tree_char = if is_last { "└─" } else { "├─" };
                                        let continuation = if is_last { "   " } else { "│  " };

                                        md.push_str(&format!("                    {} `{}`\n", tree_char, field));

                                        // Show per-field "also modified by" list
                                        if let Some(modifiers) = step.field_modifiers.get(field) {
                                            if !modifiers.is_empty() {
                                                let mods_formatted = modifiers.iter()
                                                    .map(|m| format!("`{}`", m))
                                                    .collect::<Vec<_>>()
                                                    .join(", ");
                                                md.push_str(&format!("                    {} └─ also modified by: {}\n", continuation, mods_formatted));
                                            }
                                        }
                                    }
                                } else if !step.modified_variables.is_empty() {
                                    // Fallback to base state variable if no field info
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
        Self::add_state_modification_summary(&mut md, &relations, contracts);

        // Footer
        md.push_str("\n");
        md.push_str("---\n");
        md.push_str("\n");
        md.push_str("*Generated by MainnetReady - Solidity Enhanced Analyzer*\n");

        fs::write(output_path, md)?;
        Ok(())
    }

    /// Build per-field "also modified by" map
    /// For each field, find which other functions also modify that specific field
    fn build_field_modifiers_map(
        fields: &[String],
        exclude_func: &str,
        contract: &ContractInfo,
    ) -> std::collections::HashMap<String, Vec<String>> {
        use std::collections::HashMap;

        let mut field_map: HashMap<String, Vec<String>> = HashMap::new();

        for field in fields {
            let mut modifiers = Vec::new();

            // Find all functions that modify this specific field
            for func in &contract.functions {
                if func.name != exclude_func && func.modifies_state_fields.contains(field) {
                    modifiers.push(func.name.clone());
                }
            }

            if !modifiers.is_empty() {
                field_map.insert(field.clone(), modifiers);
            }
        }

        field_map
    }

    /// Resolve storage parameter field modifications to actual state variables
    /// Converts "@param.field" to "stateVar.field" based on call site analysis
    fn resolve_storage_param_fields(
        fields: &[String],
        caller_func: &FunctionDef,
        callee_name: &str,
        contract: &ContractInfo,
    ) -> Vec<String> {
        // Find the callee function to get parameter info
        let callee_func = match contract.functions.iter().find(|f| f.name == callee_name) {
            Some(f) => f,
            None => return fields.to_vec(),
        };

        if callee_func.storage_params.is_empty() {
            return fields.to_vec();
        }

        // Parse caller function body to find call sites
        // We need to extract storage variable assignments like: LP storage lp = lpInfo[addr];
        // This is a simplified approach - we'll try to find common patterns

        let resolved: Vec<String> = fields.iter().filter_map(|field| {
            if field.starts_with("@param.") {
                let field_suffix = field.strip_prefix("@param.").unwrap_or("");

                // Strategy 1: Look for exact field path match
                // If callee modifies @param.lastDeposit and caller modifies lpInfo.lastDeposit.*,
                // extract lpInfo.lastDeposit as the match
                for caller_field in &caller_func.modifies_state_fields {
                    // Extract the field path (e.g., "lastDeposit" from "lpInfo.lastDeposit.amount")
                    let parts: Vec<&str> = caller_field.split('.').collect();
                    if parts.len() >= 2 {
                        let param_field_path = format!("@param.{}", field_suffix);

                        if param_field_path == format!("@param.{}", parts[1]) ||
                           caller_field.ends_with(field_suffix) {
                            // Extract base variable and use it
                            let resolved_field = field.replace("@param", parts[0]);
                            return Some(resolved_field);
                        }
                    }
                }

                // Strategy 2: Find the best matching base variable by counting field overlaps
                // Collect all base variables from caller's field modifications that are mappings
                use std::collections::HashMap;
                let mut base_var_counts: HashMap<String, usize> = HashMap::new();

                for caller_field in &caller_func.modifies_state_fields {
                    if let Some(base_var) = caller_field.split('.').next() {
                        if caller_func.modifies_states.contains(&base_var.to_string()) {
                            if let Some(state_def) = contract.state_variables.iter().find(|sv| &sv.name == base_var) {
                                if state_def.var_type.contains("mapping") {
                                    *base_var_counts.entry(base_var.to_string()).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }

                // Use the mapping with the most field modifications (most likely to be the storage param source)
                if let Some((best_base_var, _)) = base_var_counts.iter().max_by_key(|(_, &count)| count) {
                    let resolved_field = field.replace("@param", best_base_var);
                    return Some(resolved_field);
                }

                // Strategy 3: Fallback to first mapping state variable
                for state_var in &caller_func.modifies_states {
                    if let Some(state_def) = contract.state_variables.iter().find(|sv| &sv.name == state_var) {
                        if state_def.var_type.contains("mapping") {
                            let resolved_field = field.replace("@param", state_var);
                            return Some(resolved_field);
                        }
                    }
                }

                // If we can't resolve, keep the @param notation
                Some(field.clone())
            } else {
                Some(field.clone())
            }
        }).collect();

        resolved
    }

    /// Collect upgradeable storage fields with multiple entry points
    /// Returns in the same format as cross-contract entry points for unified display
    fn collect_upgradeable_storage_entry_points(
        contracts: &[ContractInfo]
    ) -> HashMap<(String, String), Vec<(String, String, String)>> {
        use std::collections::HashMap;

        let mut field_entry_points: HashMap<(String, String), Vec<(String, String, String)>> = HashMap::new();

        let upgradeable_contracts: Vec<_> = contracts.iter()
            .filter(|c| c.upgradeable_storage.is_some())
            .collect();

        for contract in upgradeable_contracts {
            let storage_vars: Vec<_> = contract.state_variables.iter()
                .filter(|v| v.var_type.contains("(upgradeable storage)"))
                .collect();

            for var in storage_vars {
                let key = (contract.name.clone(), var.name.clone());
                let mut entry_points_set: std::collections::HashSet<(String, String, String)> = std::collections::HashSet::new();

                for chain in &var.modification_chains {
                    // Find ALL public/external entry points in the chain
                    for func_call in &chain.call_chain {
                        if func_call.visibility == "public" || func_call.visibility == "external" {
                            // Format: (contract, entry_func, direct_modifier)
                            // For internal chains, source = target contract
                            entry_points_set.insert((
                                contract.name.clone(),
                                func_call.function_name.clone(),
                                chain.direct_modifier.clone(),
                            ));
                        }
                    }

                    // Also check if the direct modifier itself is public/external
                    if chain.direct_modifier_visibility == "public" || chain.direct_modifier_visibility == "external" {
                        entry_points_set.insert((
                            contract.name.clone(),
                            chain.direct_modifier.clone(),
                            chain.direct_modifier.clone(),
                        ));
                    }
                }

                if entry_points_set.len() > 1 {
                    field_entry_points.insert(key, entry_points_set.into_iter().collect());
                }
            }
        }

        field_entry_points
    }
}
