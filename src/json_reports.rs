use crate::models::*;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub struct JsonReportGenerator;

impl JsonReportGenerator {
    /// Generate function calls JSON report
    /// Structure: Contract -> Function -> { internal_calls: [...], external_calls: [...] }
    pub fn generate_function_calls_json(contracts: &[ContractInfo]) -> Value {
        let mut report = serde_json::Map::new();

        for contract in contracts {
            let mut contract_funcs = serde_json::Map::new();

            for func in &contract.functions {
                let mut func_data = serde_json::Map::new();

                // Internal calls (calls to private/internal functions within the same contract)
                let internal_calls: Vec<Value> = func.calls_functions.iter()
                    .filter_map(|called_func_name| {
                        // Find the called function to get its details
                        contract.functions.iter()
                            .find(|f| &f.name == called_func_name)
                            .map(|called_func| {
                                json!({
                                    "function_name": called_func.name,
                                    "parameters": called_func.parameters,
                                    "returns": if called_func.returns.is_empty() {
                                        Value::Null
                                    } else {
                                        json!(called_func.returns.join(", "))
                                    }
                                })
                            })
                    })
                    .collect();

                // External calls (calls to external contracts)
                let external_calls: Vec<Value> = func.external_calls.iter()
                    .map(|ext_call| {
                        json!({
                            "target_contract": ext_call.target_contract.clone().unwrap_or_else(|| "unknown".to_string()),
                            "target_variable": ext_call.target_variable,
                            "function_name": ext_call.target_function,
                            "state_mutability": ext_call.state_mutability,
                            "is_value_transfer": ext_call.state_mutability == "payable"
                        })
                    })
                    .collect();

                func_data.insert("internal_calls".to_string(), json!(internal_calls));
                func_data.insert("external_calls".to_string(), json!(external_calls));

                contract_funcs.insert(func.name.clone(), Value::Object(func_data));
            }

            report.insert(contract.name.clone(), Value::Object(contract_funcs));
        }

        Value::Object(report)
    }

    /// Generate state variables JSON report
    /// Structure: Contract -> Variable -> { type, visibility, readers: [...], writers: [...] }
    pub fn generate_state_variables_json(contracts: &[ContractInfo]) -> Value {
        let mut report = serde_json::Map::new();

        for contract in contracts {
            let mut contract_vars = serde_json::Map::new();

            // Build call graph for this contract
            let call_graph = Self::build_call_graph(&contract.functions);

            for var in &contract.state_variables {
                // Find writers (direct and via call chains)
                let (direct_writers, indirect_writers) = Self::find_writers(
                    &var.name,
                    &contract.functions,
                    &call_graph
                );

                // Find readers (direct and via call chains)
                let (direct_readers, indirect_readers) = Self::find_readers(
                    &var.name,
                    &contract.functions,
                    &call_graph
                );

                // Build direct readers JSON
                let readers_direct: Vec<Value> = direct_readers.iter()
                    .map(|func_name| {
                        let vis = Self::get_func_visibility(func_name, &contract.functions);
                        json!({
                            "function": func_name,
                            "visibility": vis
                        })
                    })
                    .collect();

                // Build readers via call chains JSON
                let readers_via_calls: Vec<Value> = indirect_readers.iter()
                    .map(|(caller, chain)| {
                        let vis = Self::get_func_visibility(caller, &contract.functions);
                        json!({
                            "function": caller,
                            "visibility": vis,
                            "call_chain": chain
                        })
                    })
                    .collect();

                // Build direct writers JSON
                let writers_direct: Vec<Value> = direct_writers.iter()
                    .map(|func_name| {
                        let vis = Self::get_func_visibility(func_name, &contract.functions);
                        json!({
                            "function": func_name,
                            "visibility": vis
                        })
                    })
                    .collect();

                // Build writers via call chains JSON
                let writers_via_calls: Vec<Value> = indirect_writers.iter()
                    .map(|(caller, chain)| {
                        let vis = Self::get_func_visibility(caller, &contract.functions);
                        json!({
                            "function": caller,
                            "visibility": vis,
                            "call_chain": chain
                        })
                    })
                    .collect();

                let var_data = json!({
                    "type": var.var_type,
                    "visibility": var.visibility,
                    "is_constant": var.is_constant,
                    "is_immutable": var.is_immutable,
                    "readers": {
                        "direct": readers_direct,
                        "via_call_chains": readers_via_calls
                    },
                    "writers": {
                        "direct": writers_direct,
                        "via_call_chains": writers_via_calls
                    }
                });

                contract_vars.insert(var.name.clone(), var_data);
            }

            report.insert(contract.name.clone(), Value::Object(contract_vars));
        }

        Value::Object(report)
    }

    /// Build call graph: function -> list of functions it calls
    fn build_call_graph(functions: &[FunctionDef]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for func in functions {
            if !func.calls_functions.is_empty() {
                graph.insert(func.name.clone(), func.calls_functions.clone());
            }
        }
        graph
    }

    /// Find all functions that write to a state variable (direct and via call chains)
    fn find_writers(
        var_name: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> (Vec<String>, Vec<(String, Vec<String>)>) {
        let mut direct = Vec::new();
        let mut indirect = Vec::new();

        // Find direct writers
        for func in functions {
            if func.modifies_states.contains(&var_name.to_string())
                || func.modifies_state_fields.iter().any(|f| f.starts_with(&format!("{}.", var_name))) {
                direct.push(func.name.clone());
            }
        }

        // Find indirect writers (functions that call direct writers)
        for func in functions {
            if !direct.contains(&func.name) {
                if let Some(chain) = Self::find_call_chain_to_targets(&func.name, &direct, call_graph) {
                    indirect.push((func.name.clone(), chain));
                }
            }
        }

        (direct, indirect)
    }

    /// Find all functions that read from a state variable (direct and via call chains)
    fn find_readers(
        var_name: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> (Vec<String>, Vec<(String, Vec<String>)>) {
        let mut direct = Vec::new();
        let mut indirect = Vec::new();

        // Find direct readers
        for func in functions {
            if func.reads_states.contains(&var_name.to_string()) {
                direct.push(func.name.clone());
            }
        }

        // Find indirect readers (functions that call direct readers)
        for func in functions {
            if !direct.contains(&func.name) {
                if let Some(chain) = Self::find_call_chain_to_targets(&func.name, &direct, call_graph) {
                    indirect.push((func.name.clone(), chain));
                }
            }
        }

        (direct, indirect)
    }

    /// Generic function to find call chain to any target function
    fn find_call_chain_to_targets(
        start: &str,
        targets: &[String],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut queue = vec![(start.to_string(), vec![])];

        while let Some((current, path)) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if let Some(callees) = call_graph.get(&current) {
                for callee in callees {
                    if targets.contains(callee) {
                        let mut chain = path.clone();
                        chain.push(callee.clone());
                        return Some(chain);
                    }

                    let mut new_path = path.clone();
                    new_path.push(callee.clone());
                    queue.push((callee.clone(), new_path));
                }
            }
        }

        None
    }

    /// Get function visibility
    fn get_func_visibility(func_name: &str, functions: &[FunctionDef]) -> String {
        functions
            .iter()
            .find(|f| f.name == func_name)
            .map(|f| f.visibility.clone())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Save JSON report to file
    pub fn save_json(data: &Value, path: &std::path::Path) -> anyhow::Result<()> {
        let json_str = serde_json::to_string_pretty(data)?;
        std::fs::write(path, json_str)?;
        Ok(())
    }
}
