use solang_parser::pt;
use std::collections::{HashMap, HashSet};

use crate::models::*;

pub struct StateModificationAnalyzer;

impl StateModificationAnalyzer {
    /// Analyze which functions modify which state variables and build call chains
    pub fn analyze(contract_info: &mut ContractInfo, ast: &pt::ContractDefinition) {
        // Step 1: Extract function and modifier bodies
        let mut function_bodies: HashMap<String, &pt::FunctionDefinition> = HashMap::new();
        let mut modifier_bodies: HashMap<String, &pt::FunctionDefinition> = HashMap::new();

        for part in &ast.parts {
            if let pt::ContractPart::FunctionDefinition(f) = part {
                let func_name = f.name.as_ref()
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|| "constructor".to_string());

                if matches!(f.ty, pt::FunctionTy::Modifier) {
                    modifier_bodies.insert(func_name, f);
                } else {
                    function_bodies.insert(func_name, f);
                }
            }
        }

        // Step 2: For each function, find which state variables it modifies
        let state_var_names: HashSet<String> = contract_info.state_variables
            .iter()
            .map(|v| v.name.clone())
            .collect();

        // Collect event and error names
        let event_names: HashSet<String> = contract_info.events
            .iter()
            .map(|e| e.name.clone())
            .collect();

        let error_names: HashSet<String> = contract_info.errors
            .iter()
            .map(|e| e.name.clone())
            .collect();

        for func in &mut contract_info.functions {
            if let Some(body) = function_bodies.get(&func.name) {
                // Extract storage reference parameters
                func.storage_params = Self::extract_storage_params(body);

                // Find state modifications (excluding storage parameters)
                func.modifies_states = Self::find_state_modifications(
                    body,
                    &state_var_names,
                    &func.storage_params
                );

                // Find field-level state modifications
                func.modifies_state_fields = Self::find_field_modifications(
                    body,
                    &state_var_names,
                    &func.storage_params
                );

                // Find function calls
                func.calls_functions = Self::find_function_calls(body);

                // Find event emissions
                func.emits_events = Self::find_event_emissions(body, &event_names);

                // Find error usage
                func.uses_errors = Self::find_error_usage(body, &error_names);

                // Check for unchecked blocks
                func.has_unchecked = Self::has_unchecked_blocks(body);
            }
        }

        // Step 2b: Collect errors used in modifiers
        let mut modifier_error_usage: HashMap<String, Vec<String>> = HashMap::new();

        for (modifier_name, modifier_body) in &modifier_bodies {
            let errors_used = Self::find_error_usage(modifier_body, &error_names);
            modifier_error_usage.insert(modifier_name.clone(), errors_used);
        }

        // Step 2c: Propagate modifications through storage parameters (inter-procedural analysis)
        Self::propagate_storage_param_modifications(
            &mut contract_info.functions,
            &function_bodies,
            &state_var_names
        );

        // Step 3: Build call graph
        let call_graph = Self::build_call_graph(&contract_info.functions);

        // Step 4: For each state variable, build modification chains
        for state_var in &mut contract_info.state_variables {
            state_var.modification_chains = Self::build_modification_chains(
                &state_var.name,
                &contract_info.functions,
                &call_graph,
            );
        }

        // Step 5: Build reverse lookups for events (which functions emit them)
        for event in &mut contract_info.events {
            event.emitted_in = contract_info.functions
                .iter()
                .filter(|f| f.emits_events.contains(&event.name))
                .map(|f| f.name.clone())
                .collect();
        }

        // Step 6: Build reverse lookups for errors (which functions use them)
        // Include both direct usage in functions and indirect usage through modifiers
        for error in &mut contract_info.errors {
            let mut used_in = HashSet::new();

            // Direct usage in functions
            for func in &contract_info.functions {
                if func.uses_errors.contains(&error.name) {
                    used_in.insert(func.name.clone());
                }
            }

            // Indirect usage through modifiers
            for func in &contract_info.functions {
                for modifier_name in &func.uses_modifiers {
                    if let Some(modifier_errors) = modifier_error_usage.get(modifier_name) {
                        if modifier_errors.contains(&error.name) {
                            used_in.insert(modifier_name.clone());
                        }
                    }
                }
            }

            error.used_in = used_in.into_iter().collect();
        }

        // Step 6b: Add inherited/imported errors that are used but not defined
        let defined_error_names: HashSet<String> = contract_info.errors
            .iter()
            .map(|e| e.name.clone())
            .collect();

        // Collect all used errors from functions and modifiers
        let mut all_used_errors: HashSet<String> = HashSet::new();
        for func in &contract_info.functions {
            all_used_errors.extend(func.uses_errors.clone());
        }
        for modifier_errors in modifier_error_usage.values() {
            all_used_errors.extend(modifier_errors.clone());
        }

        // Find errors that are used but not defined (inherited/imported)
        for error_name in all_used_errors {
            if !defined_error_names.contains(&error_name) {
                // This error is used but not defined - it's inherited or imported
                let mut used_in = HashSet::new();

                // Find which functions use this error
                for func in &contract_info.functions {
                    if func.uses_errors.contains(&error_name) {
                        used_in.insert(func.name.clone());
                    }
                }

                // Find which modifiers use this error
                for (modifier_name, modifier_errors) in &modifier_error_usage {
                    if modifier_errors.contains(&error_name) {
                        used_in.insert(modifier_name.clone());
                    }
                }

                // Create an ErrorDef for this inherited error
                let inherited_error = ErrorDef {
                    name: error_name.clone(),
                    parameters: Vec::new(), // We don't know the parameters
                    line_number: 0,         // Not defined in this contract
                    used_in: used_in.into_iter().collect(),
                    is_inherited: true,
                };

                contract_info.errors.push(inherited_error);
            }
        }

        // Step 7: Build reverse lookups for modifiers (which functions use them)
        for modifier in &mut contract_info.modifiers {
            modifier.used_in = contract_info.functions
                .iter()
                .filter(|f| f.uses_modifiers.contains(&modifier.name))
                .map(|f| f.name.clone())
                .collect();
        }

        // Step 8: Resolve storage struct fields to actual field names (for upgradeable contracts)
        Self::resolve_storage_struct_fields(contract_info);

        // Step 9: Create virtual state variables from storage struct fields (for upgradeable contracts)
        Self::create_virtual_state_variables(contract_info, &call_graph);
    }

    /// Find all state variables that are modified in a function body
    /// Excludes storage parameters - those are handled by inter-procedural analysis
    fn find_state_modifications(
        func: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
        storage_params: &[StorageParamInfo],
    ) -> Vec<String> {
        let mut modified = HashSet::new();
        let mut storage_var_mapping: HashMap<String, String> = HashMap::new();

        // Build set of storage parameter names to exclude
        let param_names: HashSet<String> = storage_params.iter()
            .map(|p| p.param_name.clone())
            .collect();

        if let Some(body) = &func.body {
            Self::scan_statement_for_modifications(body, state_vars, &mut modified, &mut storage_var_mapping);
        }

        // Filter out storage parameters - they will be resolved at call sites
        modified.into_iter()
            .filter(|var| !param_names.contains(var))
            .collect()
    }

    /// Find field-level state modifications (e.g., "lpInfo.consolidatedShares")
    /// Tracks granular modifications down to struct fields
    fn find_field_modifications(
        func: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
        storage_params: &[StorageParamInfo],
    ) -> Vec<String> {
        let mut modified_fields = HashSet::new();
        let mut storage_var_mapping: HashMap<String, String> = HashMap::new();

        // Build set of storage parameter names
        let param_names: HashSet<String> = storage_params.iter()
            .map(|p| p.param_name.clone())
            .collect();

        if let Some(body) = &func.body {
            Self::scan_statement_for_field_modifications(
                body,
                state_vars,
                &mut modified_fields,
                &mut storage_var_mapping,
                &param_names
            );
        }

        modified_fields.into_iter().collect()
    }

    /// Recursively scan statements for state variable modifications
    fn scan_statement_for_modifications(
        stmt: &pt::Statement,
        state_vars: &HashSet<String>,
        modified: &mut HashSet<String>,
        storage_var_mapping: &mut HashMap<String, String>,
    ) {
        match stmt {
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_modifications(expr, state_vars, modified, storage_var_mapping);
            }
            pt::Statement::Block { statements, .. } => {
                // Create a new scope for storage struct variables
                let mut local_storage_mapping = storage_var_mapping.clone();
                for s in statements {
                    Self::scan_statement_for_modifications(s, state_vars, modified, &mut local_storage_mapping);
                }
                // Merge back any new state variables discovered
                for (k, v) in local_storage_mapping.iter() {
                    if state_vars.contains(v) && !storage_var_mapping.contains_key(k) {
                        storage_var_mapping.insert(k.clone(), v.clone());
                    }
                }
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::scan_statement_for_modifications(if_branch, state_vars, modified, storage_var_mapping);
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_modifications(else_stmt, state_vars, modified, storage_var_mapping);
                }
            }
            pt::Statement::While(_, _, body) => {
                Self::scan_statement_for_modifications(body, state_vars, modified, storage_var_mapping);
            }
            pt::Statement::For(_, init, _, update, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_statement_for_modifications(init_stmt, state_vars, modified, storage_var_mapping);
                }
                if let Some(update_expr) = update {
                    Self::scan_expression_for_modifications(update_expr, state_vars, modified, storage_var_mapping);
                }
                if let Some(body_stmt) = body {
                    Self::scan_statement_for_modifications(body_stmt, state_vars, modified, storage_var_mapping);
                }
            }
            pt::Statement::DoWhile(_, body, _) => {
                Self::scan_statement_for_modifications(body, state_vars, modified, storage_var_mapping);
            }
            pt::Statement::Return(_, Some(e)) => {
                Self::scan_expression_for_modifications(e, state_vars, modified, storage_var_mapping);
            }
            pt::Statement::VariableDefinition(_, decl, Some(expr)) => {
                // Track storage local variables: e.g., LP storage lp = lpInfo[addr];
                // Also track storage struct patterns: e.g., ERC20Storage storage $ = _getERC20Storage();
                if let Some(var_name) = &decl.name {
                    let local_var_name = var_name.name.clone();

                    // Check if this is a function call (potentially a storage accessor)
                    if let pt::Expression::FunctionCall(_, func_expr, _) = expr {
                        if let pt::Expression::Variable(func_ident) = &**func_expr {
                            let func_name = func_ident.name.clone();
                            // Detect storage accessor pattern (e.g., _getERC20Storage)
                            if func_name.contains("get") && func_name.contains("Storage") {
                                // Map $ -> special marker for storage struct
                                storage_var_mapping.insert(local_var_name.clone(), "@storage_struct".to_string());
                            }
                        }
                    }
                    // Check if this is a storage reference to a state variable
                    else if let Some(base_var) = Self::extract_base_variable(expr) {
                        if state_vars.contains(&base_var) {
                            // Map local_var -> state_var
                            storage_var_mapping.insert(local_var_name, base_var);
                        }
                    }
                }
                Self::scan_expression_for_modifications(expr, state_vars, modified, storage_var_mapping);
            }
            _ => {}
        }
    }

    /// Scan expressions for state variable assignments
    fn scan_expression_for_modifications(
        expr: &pt::Expression,
        state_vars: &HashSet<String>,
        modified: &mut HashSet<String>,
        storage_var_mapping: &HashMap<String, String>,
    ) {
        match expr {
            // Direct assignment: stateVar = value
            pt::Expression::Assign(_, left, right) => {
                if let Some(var_name) = Self::extract_base_variable(left) {
                    if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                        modified.insert(state_var);
                    }
                }
                Self::scan_expression_for_modifications(right, state_vars, modified, storage_var_mapping);
            }
            // Compound assignments: +=, -=, *=, etc.
            pt::Expression::AssignAdd(_, left, right)
            | pt::Expression::AssignSubtract(_, left, right)
            | pt::Expression::AssignMultiply(_, left, right)
            | pt::Expression::AssignDivide(_, left, right)
            | pt::Expression::AssignModulo(_, left, right)
            | pt::Expression::AssignOr(_, left, right)
            | pt::Expression::AssignAnd(_, left, right)
            | pt::Expression::AssignXor(_, left, right)
            | pt::Expression::AssignShiftLeft(_, left, right)
            | pt::Expression::AssignShiftRight(_, left, right) => {
                if let Some(var_name) = Self::extract_base_variable(left) {
                    if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                        modified.insert(state_var);
                    }
                }
                Self::scan_expression_for_modifications(right, state_vars, modified, storage_var_mapping);
            }
            // Pre/Post increment/decrement: stateVar++, ++stateVar
            pt::Expression::PreIncrement(_, expr)
            | pt::Expression::PostIncrement(_, expr)
            | pt::Expression::PreDecrement(_, expr)
            | pt::Expression::PostDecrement(_, expr) => {
                if let Some(var_name) = Self::extract_base_variable(expr) {
                    if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                        modified.insert(state_var);
                    }
                }
            }
            // Delete operator: delete stateVar or delete stateVar.field
            pt::Expression::Delete(_, expr) => {
                if let Some(var_name) = Self::extract_base_variable(expr) {
                    if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                        modified.insert(state_var);
                    }
                }
            }
            // Function calls - check if it's a method call on a state variable (like array.push())
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Check if it's a member access (e.g., stateVar.push())
                if let pt::Expression::MemberAccess(_, base, member) = &**func_expr {
                    if let Some(var_name) = Self::extract_base_variable(base) {
                        // Detect mutating methods
                        if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                            let method_name = member.name.as_str();
                            if matches!(method_name, "push" | "pop" | "delete") {
                                modified.insert(state_var);
                            }
                        }
                    }
                }

                // Recursively check arguments
                for arg in args {
                    Self::scan_expression_for_modifications(arg, state_vars, modified, storage_var_mapping);
                }
            }
            // List - skip for now (contains Parameters, not Expressions)
            _ => {}
        }
    }

    /// Extract the base variable name from an expression (handles mappings and arrays)
    fn extract_base_variable(expr: &pt::Expression) -> Option<String> {
        match expr {
            pt::Expression::Variable(ident) => Some(ident.name.clone()),
            pt::Expression::MemberAccess(_, base, _) => Self::extract_base_variable(base),
            pt::Expression::ArraySubscript(_, base, _) => Self::extract_base_variable(base),
            _ => None,
        }
    }

    /// Extract the full member access path (e.g., "lpInfo.consolidatedShares")
    /// Returns None if the expression is not a simple member access chain
    fn extract_member_path(expr: &pt::Expression) -> Option<String> {
        match expr {
            pt::Expression::Variable(ident) => Some(ident.name.clone()),
            pt::Expression::MemberAccess(_, base, member) => {
                let base_path = Self::extract_member_path(base)?;
                Some(format!("{}.{}", base_path, member.name))
            }
            // For array subscripts, just track up to the array itself
            pt::Expression::ArraySubscript(_, base, _) => Self::extract_member_path(base),
            _ => None,
        }
    }

    /// Resolve variable name to ultimate state variable (following storage references)
    fn resolve_to_state_var(
        var_name: &str,
        state_vars: &HashSet<String>,
        storage_var_mapping: &HashMap<String, String>,
    ) -> Option<String> {
        // Check if it's a direct state variable
        if state_vars.contains(var_name) {
            return Some(var_name.to_string());
        }
        // Check if it's a storage reference to a state variable
        if let Some(state_var) = storage_var_mapping.get(var_name) {
            // If it's a storage struct marker, return a special indicator
            if state_var == "@storage_struct" {
                return Some("@storage_struct".to_string());
            }
            return Some(state_var.clone());
        }
        None
    }

    /// Recursively scan statements for field-level state modifications
    fn scan_statement_for_field_modifications(
        stmt: &pt::Statement,
        state_vars: &HashSet<String>,
        modified_fields: &mut HashSet<String>,
        storage_var_mapping: &mut HashMap<String, String>,
        param_names: &HashSet<String>,
    ) {
        match stmt {
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_field_modifications(
                    expr,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_statement_for_field_modifications(
                        s,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::scan_statement_for_field_modifications(
                    if_branch,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_field_modifications(
                        else_stmt,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
            }
            pt::Statement::While(_, _, body) => {
                Self::scan_statement_for_field_modifications(
                    body,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            pt::Statement::For(_, init, _, update, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_statement_for_field_modifications(
                        init_stmt,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
                if let Some(update_expr) = update {
                    Self::scan_expression_for_field_modifications(
                        update_expr,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
                if let Some(body_stmt) = body {
                    Self::scan_statement_for_field_modifications(
                        body_stmt,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
            }
            pt::Statement::DoWhile(_, body, _) => {
                Self::scan_statement_for_field_modifications(
                    body,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            pt::Statement::Return(_, Some(e)) => {
                Self::scan_expression_for_field_modifications(
                    e,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            pt::Statement::VariableDefinition(_, decl, Some(expr)) => {
                // Track storage local variables: e.g., LP storage lp = lpInfo[addr];
                // Also track storage struct patterns: e.g., ERC20Storage storage $ = _getERC20Storage();
                if let Some(var_name) = &decl.name {
                    let local_var_name = var_name.name.clone();

                    // Check if this is a function call (potentially a storage accessor)
                    if let pt::Expression::FunctionCall(_, func_expr, _) = expr {
                        if let pt::Expression::Variable(func_ident) = &**func_expr {
                            let func_name = func_ident.name.clone();
                            // Detect storage accessor pattern (e.g., _getERC20Storage)
                            if func_name.contains("get") && func_name.contains("Storage") {
                                // Map $ -> special marker for storage struct
                                storage_var_mapping.insert(local_var_name.clone(), "@storage_struct".to_string());
                            }
                        }
                    }
                    // Check if this is a storage reference to a state variable
                    else if let Some(base_var) = Self::extract_base_variable(expr) {
                        if state_vars.contains(&base_var) {
                            // Map local_var -> state_var
                            storage_var_mapping.insert(local_var_name, base_var);
                        }
                    }
                }
                Self::scan_expression_for_field_modifications(
                    expr,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            _ => {}
        }
    }

    /// Scan expressions for field-level state variable modifications
    fn scan_expression_for_field_modifications(
        expr: &pt::Expression,
        state_vars: &HashSet<String>,
        modified_fields: &mut HashSet<String>,
        storage_var_mapping: &HashMap<String, String>,
        param_names: &HashSet<String>,
    ) {
        match expr {
            // Direct assignment: stateVar.field = value or lp.field = value or $._balances[from] = value
            pt::Expression::Assign(_, left, right) => {
                if let Some(full_path) = Self::extract_member_path(left) {
                    if let Some(base_var) = Self::extract_base_variable(left) {
                        // Check if it's a storage parameter modification
                        if param_names.contains(&base_var) {
                            // Track as storage param modification (format: "@param.field")
                            let field_path = full_path.replacen(&base_var, "@param", 1);
                            modified_fields.insert(field_path);
                        } else if let Some(state_var) = Self::resolve_to_state_var(&base_var, state_vars, storage_var_mapping) {
                            // Check if it's a storage struct modification
                            if state_var == "@storage_struct" {
                                // Extract field name from path (e.g., "$._balances" -> "_balances")
                                let field_path = if let Some(dot_pos) = full_path.find('.') {
                                    format!("@storage_struct{}", &full_path[dot_pos..])
                                } else {
                                    format!("@storage_struct.{}", full_path)
                                };
                                modified_fields.insert(field_path);
                            } else {
                                // Replace the base variable with the resolved state variable
                                let field_path = full_path.replacen(&base_var, &state_var, 1);
                                modified_fields.insert(field_path);
                            }
                        }
                    }
                }
                Self::scan_expression_for_field_modifications(
                    right,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            // Compound assignments: +=, -=, *=, etc.
            pt::Expression::AssignAdd(_, left, right)
            | pt::Expression::AssignSubtract(_, left, right)
            | pt::Expression::AssignMultiply(_, left, right)
            | pt::Expression::AssignDivide(_, left, right)
            | pt::Expression::AssignModulo(_, left, right)
            | pt::Expression::AssignOr(_, left, right)
            | pt::Expression::AssignAnd(_, left, right)
            | pt::Expression::AssignXor(_, left, right)
            | pt::Expression::AssignShiftLeft(_, left, right)
            | pt::Expression::AssignShiftRight(_, left, right) => {
                if let Some(full_path) = Self::extract_member_path(left) {
                    if let Some(base_var) = Self::extract_base_variable(left) {
                        if param_names.contains(&base_var) {
                            let field_path = full_path.replacen(&base_var, "@param", 1);
                            modified_fields.insert(field_path);
                        } else if let Some(state_var) = Self::resolve_to_state_var(&base_var, state_vars, storage_var_mapping) {
                            if state_var == "@storage_struct" {
                                let field_path = if let Some(dot_pos) = full_path.find('.') {
                                    format!("@storage_struct{}", &full_path[dot_pos..])
                                } else {
                                    format!("@storage_struct.{}", full_path)
                                };
                                modified_fields.insert(field_path);
                            } else {
                                let field_path = full_path.replacen(&base_var, &state_var, 1);
                                modified_fields.insert(field_path);
                            }
                        }
                    }
                }
                Self::scan_expression_for_field_modifications(
                    right,
                    state_vars,
                    modified_fields,
                    storage_var_mapping,
                    param_names
                );
            }
            // Pre/Post increment/decrement
            pt::Expression::PreIncrement(_, expr)
            | pt::Expression::PostIncrement(_, expr)
            | pt::Expression::PreDecrement(_, expr)
            | pt::Expression::PostDecrement(_, expr) => {
                if let Some(full_path) = Self::extract_member_path(expr) {
                    if let Some(base_var) = Self::extract_base_variable(expr) {
                        if param_names.contains(&base_var) {
                            let field_path = full_path.replacen(&base_var, "@param", 1);
                            modified_fields.insert(field_path);
                        } else if let Some(state_var) = Self::resolve_to_state_var(&base_var, state_vars, storage_var_mapping) {
                            if state_var == "@storage_struct" {
                                let field_path = if let Some(dot_pos) = full_path.find('.') {
                                    format!("@storage_struct{}", &full_path[dot_pos..])
                                } else {
                                    format!("@storage_struct.{}", full_path)
                                };
                                modified_fields.insert(field_path);
                            } else {
                                let field_path = full_path.replacen(&base_var, &state_var, 1);
                                modified_fields.insert(field_path);
                            }
                        }
                    }
                }
            }
            // Delete operator
            pt::Expression::Delete(_, expr) => {
                if let Some(full_path) = Self::extract_member_path(expr) {
                    if let Some(base_var) = Self::extract_base_variable(expr) {
                        if param_names.contains(&base_var) {
                            let field_path = full_path.replacen(&base_var, "@param", 1);
                            modified_fields.insert(field_path);
                        } else if let Some(state_var) = Self::resolve_to_state_var(&base_var, state_vars, storage_var_mapping) {
                            if state_var == "@storage_struct" {
                                let field_path = if let Some(dot_pos) = full_path.find('.') {
                                    format!("@storage_struct{}", &full_path[dot_pos..])
                                } else {
                                    format!("@storage_struct.{}", full_path)
                                };
                                modified_fields.insert(field_path);
                            } else {
                                let field_path = full_path.replacen(&base_var, &state_var, 1);
                                modified_fields.insert(field_path);
                            }
                        }
                    }
                }
            }
            // Function calls - check if it's a method call on a state variable
            pt::Expression::FunctionCall(_, func_expr, args) => {
                if let pt::Expression::MemberAccess(_, base, member) = &**func_expr {
                    if let Some(full_path) = Self::extract_member_path(base) {
                        if let Some(base_var) = Self::extract_base_variable(base) {
                            // Detect mutating methods
                            let mutating_methods = ["push", "pop", "add", "remove", "delete"];
                            if mutating_methods.contains(&member.name.as_str()) {
                                if param_names.contains(&base_var) {
                                    let field_path = full_path.replacen(&base_var, "@param", 1);
                                    modified_fields.insert(field_path);
                                } else if let Some(state_var) = Self::resolve_to_state_var(&base_var, state_vars, storage_var_mapping) {
                                    if state_var == "@storage_struct" {
                                        let field_path = if let Some(dot_pos) = full_path.find('.') {
                                            format!("@storage_struct{}", &full_path[dot_pos..])
                                        } else {
                                            format!("@storage_struct.{}", full_path)
                                        };
                                        modified_fields.insert(field_path);
                                    } else {
                                        let field_path = full_path.replacen(&base_var, &state_var, 1);
                                        modified_fields.insert(field_path);
                                    }
                                }
                            }
                        }
                    }
                }
                // Scan arguments
                for arg in args {
                    Self::scan_expression_for_field_modifications(
                        arg,
                        state_vars,
                        modified_fields,
                        storage_var_mapping,
                        param_names
                    );
                }
            }
            // Recursively scan other expressions
            _ => {}
        }
    }

    /// Extract storage reference parameters from a function definition
    fn extract_storage_params(func: &pt::FunctionDefinition) -> Vec<StorageParamInfo> {
        let mut storage_params = Vec::new();

        for (index, (_loc, param_opt)) in func.params.iter().enumerate() {
            if let Some(param) = param_opt {
                // Check if this parameter has storage location = Storage
                if let Some(storage_loc) = &param.storage {
                    if matches!(storage_loc, pt::StorageLocation::Storage(_)) {
                        // Extract parameter name
                        if let Some(param_name) = &param.name {
                            storage_params.push(StorageParamInfo {
                                param_index: index,
                                param_name: param_name.name.clone(),
                            });
                        }
                    }
                }
            }
        }

        storage_params
    }

    /// Propagate state modifications through storage parameters (inter-procedural analysis)
    /// This performs a fixed-point iteration to handle nested calls
    fn propagate_storage_param_modifications(
        functions: &mut [FunctionDef],
        function_bodies: &HashMap<String, &pt::FunctionDefinition>,
        state_vars: &HashSet<String>,
    ) {
        // Build index for quick lookup
        let mut func_map: HashMap<String, usize> = HashMap::new();
        for (i, func) in functions.iter().enumerate() {
            func_map.insert(func.name.clone(), i);
        }

        // Iterate until no new modifications are found (fixed-point)
        let mut changed = true;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 10;

        while changed && iteration < MAX_ITERATIONS {
            changed = false;
            iteration += 1;

            // For each function, check if it calls other functions with storage args
            for func_idx in 0..functions.len() {
                let func_name = functions[func_idx].name.clone();

                if let Some(body) = function_bodies.get(&func_name) {
                    // Find all function calls with their arguments
                    let call_info = Self::extract_function_calls_with_args(body, state_vars);

                    // For each call, check if we're passing storage references
                    let mut new_modifications = HashSet::new();

                    for (callee_name, args) in call_info {
                        // Look up the callee function
                        if let Some(&callee_idx) = func_map.get(&callee_name) {
                            let callee = &functions[callee_idx];

                            // For each storage parameter of the callee
                            for storage_param in &callee.storage_params {
                                // Check if we have an argument at that position
                                if let Some(arg_var) = args.get(storage_param.param_index) {
                                    // Check if this storage parameter is modified by the callee
                                    // (check if any of callee's modifications match the parameter name)
                                    let param_modified = callee.modifies_states.iter().any(|mod_var| {
                                        // The callee modifies the parameter if it modifies a variable
                                        // that resolves to the parameter name
                                        mod_var == &storage_param.param_name ||
                                        mod_var.starts_with(&format!("{}.", storage_param.param_name))
                                    });

                                    if param_modified {
                                        // The argument is being modified through the callee
                                        // Add this state variable to caller's modifications
                                        new_modifications.insert(arg_var.clone());
                                    }
                                }
                            }
                        }
                    }

                    // Add new modifications to the function
                    for new_mod in new_modifications {
                        if !functions[func_idx].modifies_states.contains(&new_mod) {
                            functions[func_idx].modifies_states.push(new_mod);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    /// Extract function calls along with their arguments resolved to state variables
    /// Returns: Vec<(callee_name, Vec<state_var_name>)> where index = parameter position
    fn extract_function_calls_with_args(
        func: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
    ) -> Vec<(String, Vec<String>)> {
        let mut calls_with_args = Vec::new();

        // Build storage variable mapping for this function
        let mut storage_var_mapping = HashMap::new();
        if let Some(body) = &func.body {
            Self::build_storage_mapping(body, state_vars, &mut storage_var_mapping);
        }

        // Now extract calls with arguments
        if let Some(body) = &func.body {
            Self::scan_for_calls_with_args(body, state_vars, &storage_var_mapping, &mut calls_with_args);
        }

        calls_with_args
    }

    /// Build storage variable mapping from function body
    fn build_storage_mapping(
        stmt: &pt::Statement,
        state_vars: &HashSet<String>,
        storage_var_mapping: &mut HashMap<String, String>,
    ) {
        match stmt {
            pt::Statement::VariableDefinition(_, decl, Some(expr)) => {
                if let Some(base_var) = Self::extract_base_variable(expr) {
                    if state_vars.contains(&base_var) {
                        if let Some(var_name) = &decl.name {
                            storage_var_mapping.insert(var_name.name.clone(), base_var);
                        }
                    }
                }
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::build_storage_mapping(s, state_vars, storage_var_mapping);
                }
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::build_storage_mapping(if_branch, state_vars, storage_var_mapping);
                if let Some(else_stmt) = else_branch {
                    Self::build_storage_mapping(else_stmt, state_vars, storage_var_mapping);
                }
            }
            pt::Statement::While(_, _, body) |
            pt::Statement::DoWhile(_, body, _) => {
                Self::build_storage_mapping(body, state_vars, storage_var_mapping);
            }
            pt::Statement::For(_, init, _, _, body) => {
                if let Some(init_stmt) = init {
                    Self::build_storage_mapping(init_stmt, state_vars, storage_var_mapping);
                }
                if let Some(body_stmt) = body {
                    Self::build_storage_mapping(body_stmt, state_vars, storage_var_mapping);
                }
            }
            _ => {}
        }
    }

    /// Scan for function calls with their arguments resolved to state variables
    fn scan_for_calls_with_args(
        stmt: &pt::Statement,
        state_vars: &HashSet<String>,
        storage_var_mapping: &HashMap<String, String>,
        calls_with_args: &mut Vec<(String, Vec<String>)>,
    ) {
        match stmt {
            pt::Statement::Expression(_, expr) => {
                Self::scan_expr_for_calls_with_args(expr, state_vars, storage_var_mapping, calls_with_args);
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_for_calls_with_args(s, state_vars, storage_var_mapping, calls_with_args);
                }
            }
            pt::Statement::If(_, cond, if_branch, else_branch) => {
                Self::scan_expr_for_calls_with_args(cond, state_vars, storage_var_mapping, calls_with_args);
                Self::scan_for_calls_with_args(if_branch, state_vars, storage_var_mapping, calls_with_args);
                if let Some(else_stmt) = else_branch {
                    Self::scan_for_calls_with_args(else_stmt, state_vars, storage_var_mapping, calls_with_args);
                }
            }
            pt::Statement::While(_, cond, body) => {
                Self::scan_expr_for_calls_with_args(cond, state_vars, storage_var_mapping, calls_with_args);
                Self::scan_for_calls_with_args(body, state_vars, storage_var_mapping, calls_with_args);
            }
            pt::Statement::For(_, init, cond, update, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_for_calls_with_args(init_stmt, state_vars, storage_var_mapping, calls_with_args);
                }
                if let Some(cond_expr) = cond {
                    Self::scan_expr_for_calls_with_args(cond_expr, state_vars, storage_var_mapping, calls_with_args);
                }
                if let Some(update_expr) = update {
                    Self::scan_expr_for_calls_with_args(update_expr, state_vars, storage_var_mapping, calls_with_args);
                }
                if let Some(body_stmt) = body {
                    Self::scan_for_calls_with_args(body_stmt, state_vars, storage_var_mapping, calls_with_args);
                }
            }
            pt::Statement::Return(_, Some(e)) => {
                Self::scan_expr_for_calls_with_args(e, state_vars, storage_var_mapping, calls_with_args);
            }
            _ => {}
        }
    }

    /// Scan expression for function calls with arguments
    fn scan_expr_for_calls_with_args(
        expr: &pt::Expression,
        state_vars: &HashSet<String>,
        storage_var_mapping: &HashMap<String, String>,
        calls_with_args: &mut Vec<(String, Vec<String>)>,
    ) {
        match expr {
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Extract function name (only for simple internal calls)
                if let pt::Expression::Variable(ident) = &**func_expr {
                    let func_name = ident.name.clone();

                    // Resolve each argument to a state variable if possible
                    let mut resolved_args = Vec::new();
                    for arg in args {
                        if let Some(var_name) = Self::extract_base_variable(arg) {
                            // Try to resolve to state variable
                            if let Some(state_var) = Self::resolve_to_state_var(&var_name, state_vars, storage_var_mapping) {
                                resolved_args.push(state_var);
                            } else {
                                // Argument doesn't resolve to a state variable
                                resolved_args.push(String::new());
                            }
                        } else {
                            // Not a simple variable
                            resolved_args.push(String::new());
                        }
                    }

                    calls_with_args.push((func_name, resolved_args));
                }

                // Recursively scan arguments for nested calls
                for arg in args {
                    Self::scan_expr_for_calls_with_args(arg, state_vars, storage_var_mapping, calls_with_args);
                }
            }
            pt::Expression::MemberAccess(_, base, _) => {
                Self::scan_expr_for_calls_with_args(base, state_vars, storage_var_mapping, calls_with_args);
            }
            _ => {}
        }
    }

    /// Find all function calls within a function body
    fn find_function_calls(func: &pt::FunctionDefinition) -> Vec<String> {
        let mut calls = HashSet::new();

        if let Some(body) = &func.body {
            Self::scan_statement_for_calls(body, &mut calls);
        }

        calls.into_iter().collect()
    }

    /// Recursively scan statements for function calls
    fn scan_statement_for_calls(stmt: &pt::Statement, calls: &mut HashSet<String>) {
        match stmt {
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_calls(expr, calls);
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_statement_for_calls(s, calls);
                }
            }
            pt::Statement::If(_, cond, if_branch, else_branch) => {
                Self::scan_expression_for_calls(cond, calls);
                Self::scan_statement_for_calls(if_branch, calls);
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_calls(else_stmt, calls);
                }
            }
            pt::Statement::While(_, cond, body) => {
                Self::scan_expression_for_calls(cond, calls);
                Self::scan_statement_for_calls(body, calls);
            }
            pt::Statement::For(_, init, cond, update, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_statement_for_calls(init_stmt, calls);
                }
                if let Some(cond_expr) = cond {
                    Self::scan_expression_for_calls(cond_expr, calls);
                }
                if let Some(update_expr) = update {
                    Self::scan_expression_for_calls(update_expr, calls);
                }
                if let Some(body_stmt) = body {
                    Self::scan_statement_for_calls(body_stmt, calls);
                }
            }
            pt::Statement::Return(_, Some(e)) => {
                Self::scan_expression_for_calls(e, calls);
            }
            _ => {}
        }
    }

    /// Scan expressions for function calls
    fn scan_expression_for_calls(expr: &pt::Expression, calls: &mut HashSet<String>) {
        match expr {
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Extract function name
                if let pt::Expression::Variable(ident) = &**func_expr {
                    calls.insert(ident.name.clone());
                }
                // Scan arguments
                for arg in args {
                    Self::scan_expression_for_calls(arg, calls);
                }
            }
            pt::Expression::MemberAccess(_, base, _) => {
                Self::scan_expression_for_calls(base, calls);
            }
            _ => {}
        }
    }

    /// Build a call graph: function -> list of functions it calls
    fn build_call_graph(functions: &[FunctionDef]) -> HashMap<String, Vec<String>> {
        functions
            .iter()
            .map(|f| (f.name.clone(), f.calls_functions.clone()))
            .collect()
    }

    /// Build modification chains for a specific state variable
    fn build_modification_chains(
        state_var: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> Vec<ModificationChain> {
        let mut chains = Vec::new();

        // Find all functions that directly modify this state variable
        let direct_modifiers: Vec<&FunctionDef> = functions
            .iter()
            .filter(|f| f.modifies_states.contains(&state_var.to_string()))
            .collect();

        for modifier_func in direct_modifiers {
            // Build the call chain for this direct modifier
            let call_chain = Self::build_reverse_call_chain(
                &modifier_func.name,
                functions,
                call_graph,
            );

            chains.push(ModificationChain {
                direct_modifier: modifier_func.name.clone(),
                direct_modifier_visibility: modifier_func.visibility.clone(),
                call_chain,
            });
        }

        chains
    }

    /// Build reverse call chain: who calls this function?
    fn build_reverse_call_chain(
        target_func: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> Vec<FunctionCall> {
        let mut chain = Vec::new();
        let mut visited = HashSet::new();

        // Find all functions that call the target function
        Self::find_callers(
            target_func,
            functions,
            call_graph,
            &mut chain,
            &mut visited,
        );

        chain
    }

    /// Recursively find all callers of a function
    fn find_callers(
        target: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
        chain: &mut Vec<FunctionCall>,
        visited: &mut HashSet<String>,
    ) {
        // Avoid infinite recursion
        if visited.contains(target) {
            return;
        }
        visited.insert(target.to_string());

        // Find functions that call the target
        for func in functions {
            if let Some(calls) = call_graph.get(&func.name) {
                if calls.contains(&target.to_string()) {
                    chain.push(FunctionCall {
                        function_name: func.name.clone(),
                        visibility: func.visibility.clone(),
                    });

                    // Recursively find callers of this function
                    Self::find_callers(&func.name, functions, call_graph, chain, visited);
                }
            }
        }
    }

    /// Find all events emitted in a function
    fn find_event_emissions(func: &pt::FunctionDefinition, event_names: &HashSet<String>) -> Vec<String> {
        let mut emitted = HashSet::new();

        if let Some(body) = &func.body {
            Self::scan_statement_for_events(body, event_names, &mut emitted);
        }

        emitted.into_iter().collect()
    }

    fn scan_statement_for_events(stmt: &pt::Statement, event_names: &HashSet<String>, emitted: &mut HashSet<String>) {
        match stmt {
            pt::Statement::Emit(_, pt::Expression::FunctionCall(_, func_expr, _)) => {
                if let pt::Expression::Variable(ident) = &**func_expr {
                    if event_names.contains(&ident.name) {
                        emitted.insert(ident.name.clone());
                    }
                }
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_statement_for_events(s, event_names, emitted);
                }
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::scan_statement_for_events(if_branch, event_names, emitted);
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_events(else_stmt, event_names, emitted);
                }
            }
            pt::Statement::While(_, _, body) | pt::Statement::DoWhile(_, body, _) => {
                Self::scan_statement_for_events(body, event_names, emitted);
            }
            pt::Statement::For(_, init, _, _, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_statement_for_events(init_stmt, event_names, emitted);
                }
                if let Some(body_stmt) = body {
                    Self::scan_statement_for_events(body_stmt, event_names, emitted);
                }
            }
            _ => {}
        }
    }

    /// Find all custom errors used in a function
    fn find_error_usage(func: &pt::FunctionDefinition, error_names: &HashSet<String>) -> Vec<String> {
        let mut used = HashSet::new();

        if let Some(body) = &func.body {
            Self::scan_statement_for_errors(body, error_names, &mut used);
        }

        used.into_iter().collect()
    }

    fn scan_statement_for_errors(stmt: &pt::Statement, _error_names: &HashSet<String>, used: &mut HashSet<String>) {
        match stmt {
            pt::Statement::Revert(_, Some(path), _args) => {
                // error_path is the IdentifierPath for the custom error
                // Get the error name from the identifier path (e.g., "ErrorName" or "Library.ErrorName")
                let error_name = path.identifiers.iter()
                    .map(|id| id.name.clone())
                    .collect::<Vec<_>>()
                    .join(".");

                // Track ALL errors, not just locally defined ones
                used.insert(error_name);
            }
            pt::Statement::Expression(_, expr) => {
                // Check if expression contains a revert call with custom error
                Self::scan_expression_for_errors(expr, _error_names, used);
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_statement_for_errors(s, _error_names, used);
                }
            }
            pt::Statement::If(_, cond, if_branch, else_branch) => {
                // Also scan the condition for error calls
                Self::scan_expression_for_errors(cond, _error_names, used);
                Self::scan_statement_for_errors(if_branch, _error_names, used);
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_errors(else_stmt, _error_names, used);
                }
            }
            pt::Statement::While(_, _, body) | pt::Statement::DoWhile(_, body, _) => {
                Self::scan_statement_for_errors(body, _error_names, used);
            }
            pt::Statement::For(_, init, _, _, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_statement_for_errors(init_stmt, _error_names, used);
                }
                if let Some(body_stmt) = body {
                    Self::scan_statement_for_errors(body_stmt, _error_names, used);
                }
            }
            _ => {}
        }
    }

    fn scan_expression_for_errors(_expr: &pt::Expression, _error_names: &HashSet<String>, _used: &mut HashSet<String>) {
        // This function is no longer needed - we only track errors from explicit revert statements
        // Regular function calls in expressions are not errors
    }

    /// Check if function contains unchecked blocks
    fn has_unchecked_blocks(func: &pt::FunctionDefinition) -> bool {
        if let Some(body) = &func.body {
            Self::scan_for_unchecked(body)
        } else {
            false
        }
    }

    fn scan_for_unchecked(stmt: &pt::Statement) -> bool {
        match stmt {
            pt::Statement::Block { unchecked, statements, .. } => {
                if *unchecked {
                    return true;
                }
                for s in statements {
                    if Self::scan_for_unchecked(s) {
                        return true;
                    }
                }
                false
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                if Self::scan_for_unchecked(if_branch) {
                    return true;
                }
                if let Some(else_stmt) = else_branch {
                    if Self::scan_for_unchecked(else_stmt) {
                        return true;
                    }
                }
                false
            }
            pt::Statement::While(_, _, body) | pt::Statement::DoWhile(_, body, _) => {
                Self::scan_for_unchecked(body)
            }
            pt::Statement::For(_, _, _, _, Some(body_stmt)) => {
                Self::scan_for_unchecked(body_stmt)
            }
            _ => false,
        }
    }

    /// Detect external contract calls in a contract
    pub fn detect_external_calls(
        contract_info: &mut ContractInfo,
        ast: &pt::ContractDefinition,
        all_contracts: &[ContractInfo],
    ) -> Vec<ExternalCall> {
        let mut external_calls = Vec::new();
        let mut function_external_calls: HashMap<String, Vec<ExternalCall>> = HashMap::new();

        // Build a map of state variable name -> type
        let mut var_types: HashMap<String, String> = HashMap::new();
        for part in &ast.parts {
            if let pt::ContractPart::VariableDefinition(var) = part {
                if let Some(ident) = &var.name {
                    let var_name = ident.name.clone();
                    let var_type = Self::extract_type_name(&var.ty);
                    var_types.insert(var_name, var_type);
                }
            }
        }

        // Build a map of type name -> contract name
        let mut type_to_contract: HashMap<String, String> = HashMap::new();
        for contract in all_contracts {
            type_to_contract.insert(contract.name.clone(), contract.name.clone());
        }

        // Scan each function for external calls
        let mut function_bodies: HashMap<String, &pt::FunctionDefinition> = HashMap::new();
        for part in &ast.parts {
            if let pt::ContractPart::FunctionDefinition(f) = part {
                if !matches!(f.ty, pt::FunctionTy::Modifier) {
                    let func_name = f.name.as_ref()
                        .map(|n| n.name.clone())
                        .unwrap_or_else(|| "constructor".to_string());
                    function_bodies.insert(func_name, f);
                }
            }
        }

        for func in &contract_info.functions {
            if let Some(body) = function_bodies.get(&func.name) {
                if let Some(func_body) = &body.body {
                    Self::scan_for_external_calls(
                        func_body,
                        &contract_info.name,
                        &func.name,
                        &var_types,
                        &type_to_contract,
                        func.line_number,
                        &mut external_calls,
                        all_contracts,
                        &mut function_external_calls,
                    );
                }
            }
        }

        // Populate the external_calls field on each function
        for func in &mut contract_info.functions {
            if let Some(calls) = function_external_calls.get(&func.name) {
                func.external_calls = calls.clone();
            }
        }

        external_calls
    }

    fn extract_type_name(ty: &pt::Expression) -> String {
        match ty {
            pt::Expression::Type(_, ty) => Self::type_expr_to_string(ty),
            pt::Expression::Variable(ident) => ident.name.clone(),
            _ => "unknown".to_string(),
        }
    }

    fn type_expr_to_string(ty: &pt::Type) -> String {
        match ty {
            pt::Type::Address => "address".to_string(),
            pt::Type::Bool => "bool".to_string(),
            pt::Type::String => "string".to_string(),
            pt::Type::Uint(size) => format!("uint{}", size),
            pt::Type::Int(size) => format!("int{}", size),
            pt::Type::Bytes(size) => format!("bytes{}", size),
            _ => "unknown".to_string(),
        }
    }

    fn scan_for_external_calls(
        stmt: &pt::Statement,
        contract_name: &str,
        function_name: &str,
        var_types: &HashMap<String, String>,
        type_to_contract: &HashMap<String, String>,
        line_number: usize,
        external_calls: &mut Vec<ExternalCall>,
        all_contracts: &[ContractInfo],
        function_external_calls: &mut HashMap<String, Vec<ExternalCall>>,
    ) {
        match stmt {
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_external_calls(
                    expr,
                    contract_name,
                    function_name,
                    var_types,
                    type_to_contract,
                    line_number,
                    external_calls,
                    all_contracts,
                    function_external_calls,
                );
            }
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_for_external_calls(
                        s,
                        contract_name,
                        function_name,
                        var_types,
                        type_to_contract,
                        line_number,
                        external_calls,
                    all_contracts,
                    function_external_calls,
                    );
                }
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::scan_for_external_calls(
                    if_branch,
                    contract_name,
                    function_name,
                    var_types,
                    type_to_contract,
                    line_number,
                    external_calls,
                    all_contracts,
                    function_external_calls,
                );
                if let Some(else_stmt) = else_branch {
                    Self::scan_for_external_calls(
                        else_stmt,
                        contract_name,
                        function_name,
                        var_types,
                        type_to_contract,
                        line_number,
                        external_calls,
                    all_contracts,
                    function_external_calls,
                    );
                }
            }
            pt::Statement::While(_, _, body) | pt::Statement::DoWhile(_, body, _) => {
                Self::scan_for_external_calls(
                    body,
                    contract_name,
                    function_name,
                    var_types,
                    type_to_contract,
                    line_number,
                    external_calls,
                    all_contracts,
                    function_external_calls,
                );
            }
            pt::Statement::For(_, init, _, _, body) => {
                if let Some(init_stmt) = init {
                    Self::scan_for_external_calls(
                        init_stmt,
                        contract_name,
                        function_name,
                        var_types,
                        type_to_contract,
                        line_number,
                        external_calls,
                    all_contracts,
                    function_external_calls,
                    );
                }
                if let Some(body_stmt) = body {
                    Self::scan_for_external_calls(
                        body_stmt,
                        contract_name,
                        function_name,
                        var_types,
                        type_to_contract,
                        line_number,
                        external_calls,
                    all_contracts,
                    function_external_calls,
                    );
                }
            }
            _ => {}
        }
    }

    /// Resolve a type name to a contract name, with fallback for interface names
    fn resolve_contract_from_type(
        var_type: &str,
        type_to_contract: &HashMap<String, String>
    ) -> Option<String> {
        // If the type name looks like an interface (starts with I + uppercase),
        // try removing the "I" prefix first to find the implementation
        if var_type.starts_with('I') && var_type.len() > 1 {
            if let Some(second_char) = var_type.chars().nth(1) {
                if second_char.is_uppercase() {
                    let without_i = &var_type[1..];
                    if let Some(contract) = type_to_contract.get(without_i) {
                        return Some(contract.clone());
                    }
                }
            }
        }

        // Fall back to direct lookup
        if let Some(contract) = type_to_contract.get(var_type) {
            return Some(contract.clone());
        }

        None
    }

    fn scan_expression_for_external_calls(
        expr: &pt::Expression,
        contract_name: &str,
        function_name: &str,
        var_types: &HashMap<String, String>,
        type_to_contract: &HashMap<String, String>,
        line_number: usize,
        external_calls: &mut Vec<ExternalCall>,
        all_contracts: &[ContractInfo],
        function_external_calls: &mut HashMap<String, Vec<ExternalCall>>,
    ) {
        match expr {
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Check if this is a member access call (e.g., token.transfer())
                if let pt::Expression::MemberAccess(_, obj_expr, member_ident) = func_expr.as_ref() {
                    // Get the variable name
                    if let pt::Expression::Variable(var_ident) = obj_expr.as_ref() {
                        let var_name = var_ident.name.clone();
                        let target_func = member_ident.name.clone();

                        // Look up the variable type
                        if let Some(var_type) = var_types.get(&var_name) {
                            // Try to match to a known contract (with interface name resolution)
                            let target_contract = Self::resolve_contract_from_type(var_type, type_to_contract);

                            // Try to get state mutability from target function
                            let state_mutability = if let Some(ref tc_name) = target_contract {
                                all_contracts.iter()
                                    .find(|c| &c.name == tc_name)
                                    .and_then(|tc| tc.functions.iter().find(|f| f.name == target_func))
                                    .map(|f| f.state_mutability.clone())
                                    .unwrap_or_else(|| "unknown".to_string())
                            } else {
                                "unknown".to_string()
                            };

                            let ext_call = ExternalCall {
                                source_contract: contract_name.to_string(),
                                source_function: function_name.to_string(),
                                target_variable: var_name,
                                target_type: var_type.clone(),
                                target_function: target_func,
                                target_contract,
                                state_mutability,
                                line_number,
                            };

                            external_calls.push(ext_call.clone());
                            function_external_calls
                                .entry(function_name.to_string())
                                .or_default()
                                .push(ext_call);
                        }
                    }
                }

                // Recursively scan arguments for nested external calls
                for arg in args {
                    Self::scan_expression_for_external_calls(
                        arg,
                        contract_name,
                        function_name,
                        var_types,
                        type_to_contract,
                        line_number,
                        external_calls,
                    all_contracts,
                    function_external_calls,
                    );
                }
            }
            // Recursively check subexpressions
            pt::Expression::MemberAccess(_, sub_expr, _) => {
                Self::scan_expression_for_external_calls(
                    sub_expr,
                    contract_name,
                    function_name,
                    var_types,
                    type_to_contract,
                    line_number,
                    external_calls,
                    all_contracts,
                    function_external_calls,
                );
            }
            _ => {}
        }
    }

    /// Resolve storage struct field modifications to actual field names
    /// Converts "@storage_struct._balances" to "_balances" (actual storage field)
    fn resolve_storage_struct_fields(contract_info: &mut ContractInfo) {
        // Check if this contract uses upgradeable storage
        if contract_info.upgradeable_storage.is_none() {
            return;
        }

        // For each function, resolve storage struct field modifications
        for func in &mut contract_info.functions {
            // Process modifies_state_fields
            let resolved_fields: Vec<String> = func.modifies_state_fields.iter()
                .map(|field| {
                    if field.starts_with("@storage_struct.") {
                        // Extract the actual field name (e.g., "@storage_struct._balances" -> "_balances")
                        field.replacen("@storage_struct.", "", 1)
                    } else {
                        field.clone()
                    }
                })
                .collect();

            func.modifies_state_fields = resolved_fields;

            // Remove @storage_struct marker from modifies_states
            func.modifies_states.retain(|state| state != "@storage_struct");

            // Also track these as general state modifications
            let storage_state_mods: Vec<String> = func.modifies_state_fields.iter()
                .map(|field| {
                    // Extract base field name before any array access or dots
                    let base_field = field.split('[').next()
                                          .unwrap_or(field)
                                          .split('.').next()
                                          .unwrap_or(field);
                    base_field.to_string()
                })
                .filter(|base_field| !base_field.is_empty() && !func.modifies_states.contains(base_field))
                .collect();

            func.modifies_states.extend(storage_state_mods);
        }
    }

    /// Create virtual state variables from storage struct fields for upgradeable contracts
    /// These will be tracked just like regular state variables with modification chains
    fn create_virtual_state_variables(
        contract_info: &mut ContractInfo,
        call_graph: &HashMap<String, Vec<String>>,
    ) {
        use crate::models::StateVariable;

        // Check if this contract uses upgradeable storage
        let upgradeable_storage = match &contract_info.upgradeable_storage {
            Some(storage) => storage.clone(),
            None => return,
        };

        // Create a virtual state variable for each field in the storage struct
        for field in &upgradeable_storage.struct_fields {
            let field_name = field.name.clone();

            // Build modification chains for this field
            let modification_chains = Self::build_modification_chains(
                &field_name,
                &contract_info.functions,
                call_graph,
            );

            // Create the virtual state variable
            let virtual_var = StateVariable {
                name: field_name.clone(),
                var_type: format!("{} (upgradeable storage)", field.member_type),
                visibility: "private".to_string(), // Storage struct fields are private
                is_constant: false,
                is_immutable: false,
                line_number: upgradeable_storage.line_number,
                modification_chains,
            };

            // Add to state variables list
            contract_info.state_variables.push(virtual_var);
        }
    }
}
