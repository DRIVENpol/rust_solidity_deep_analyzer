use crate::models::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub struct GraphGenerator;

impl GraphGenerator {
    /// Generate DOT file for function call graph
    /// Shows internal and external function calls for each contract
    pub fn generate_function_call_graph(contracts: &[ContractInfo], output_path: &Path) -> anyhow::Result<()> {
        let mut dot = String::new();

        dot.push_str("digraph FunctionCalls {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box, style=rounded];\n");
        dot.push_str("  graph [fontname=\"Arial\", fontsize=12];\n");
        dot.push_str("  node [fontname=\"Arial\", fontsize=10];\n");
        dot.push_str("  edge [fontname=\"Arial\", fontsize=9];\n\n");

        for contract in contracts {
            // Create a subgraph for each contract
            dot.push_str(&format!("  subgraph cluster_{} {{\n", contract.name));
            dot.push_str(&format!("    label=\"{}\";\n", contract.name));
            dot.push_str("    style=filled;\n");
            dot.push_str("    color=lightgrey;\n");
            dot.push_str("    node [style=filled,color=white];\n\n");

            // Add nodes for each function
            for func in &contract.functions {
                let color = match func.visibility.as_str() {
                    "external" => "lightblue",
                    "public" => "lightgreen",
                    "internal" => "lightyellow",
                    "private" => "lightpink",
                    _ => "white",
                };

                let node_id = format!("{}_{}", contract.name, func.name);
                dot.push_str(&format!("    \"{}\" [label=\"{}\", fillcolor={}];\n",
                    node_id, func.name, color));
            }

            dot.push('\n');

            // Add edges for internal calls
            for func in &contract.functions {
                let from_id = format!("{}_{}", contract.name, func.name);

                for called_func in &func.calls_functions {
                    let to_id = format!("{}_{}", contract.name, called_func);
                    dot.push_str(&format!("    \"{}\" -> \"{}\" [color=blue, label=\"internal\"];\n",
                        from_id, to_id));
                }
            }

            dot.push_str("  }\n\n");

            // Add edges for external calls (between contracts)
            for func in &contract.functions {
                for ext_call in &func.external_calls {
                    if let Some(ref target_contract) = ext_call.target_contract {
                        let from_id = format!("{}_{}", contract.name, func.name);
                        let to_id = format!("{}_{}", target_contract, ext_call.target_function);

                        let color = if ext_call.state_mutability == "payable" {
                            "red"
                        } else if ext_call.state_mutability == "view" {
                            "green"
                        } else {
                            "orange"
                        };

                        dot.push_str(&format!("  \"{}\" -> \"{}\" [color={}, style=dashed, label=\"{}\"];\n",
                            from_id, to_id, color, ext_call.target_function));
                    }
                }
            }
        }

        dot.push_str("}\n");

        fs::write(output_path, dot)?;
        Ok(())
    }

    /// Generate DOT file for state variable dependency graph
    /// Shows which functions read/write each state variable
    pub fn generate_state_variable_graph(contracts: &[ContractInfo], output_path: &Path) -> anyhow::Result<()> {
        let mut dot = String::new();

        dot.push_str("digraph StateVariables {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  compound=true;\n");
        dot.push_str("  newrank=true;\n");
        dot.push_str("  ranksep=1.5;\n");
        dot.push_str("  nodesep=0.5;\n");
        dot.push_str("  graph [fontname=\"Arial\", fontsize=12];\n");
        dot.push_str("  node [fontname=\"Arial\", fontsize=10];\n");
        dot.push_str("  edge [fontname=\"Arial\", fontsize=9];\n\n");

        for contract in contracts {
            // Create a subgraph for each contract
            dot.push_str(&format!("  subgraph cluster_{} {{\n", contract.name));
            dot.push_str(&format!("    label=\"{}\";\n", contract.name));
            dot.push_str("    style=filled;\n");
            dot.push_str("    color=lightgrey;\n");
            dot.push_str("    rankdir=LR;\n\n");

            // Create a subgraph for state variables to keep them together
            dot.push_str("    subgraph {\n");
            dot.push_str("      rank=same;\n");
            dot.push_str("      // State Variables\n");
            for var in &contract.state_variables {
                let shape = if var.is_constant || var.is_immutable {
                    "hexagon"
                } else {
                    "cylinder"
                };

                let color = if var.is_constant {
                    "lightgray"
                } else if var.is_immutable {
                    "lightblue"
                } else {
                    "lightyellow"
                };

                let node_id = format!("{}_{}", contract.name, var.name);
                dot.push_str(&format!("      \"{}\" [label=\"{}\", shape={}, style=filled, fillcolor={}];\n",
                    node_id, var.name, shape, color));
            }
            dot.push_str("    }\n\n");

            // Create a subgraph for functions to keep them together
            dot.push_str("    subgraph {\n");
            dot.push_str("      rank=same;\n");
            dot.push_str("      // Functions\n");
            for func in &contract.functions {
                let node_id = format!("{}_{}", contract.name, func.name);
                let color = match func.visibility.as_str() {
                    "external" => "lightgreen",
                    "public" => "palegreen",
                    "internal" => "wheat",
                    "private" => "lightpink",
                    _ => "white",
                };

                dot.push_str(&format!("      \"{}\" [label=\"{}\", shape=box, style=\"rounded,filled\", fillcolor={}];\n",
                    node_id, func.name, color));
            }
            dot.push_str("    }\n");

            dot.push_str("  }\n\n");

            // Build call graph for this contract
            let call_graph = Self::build_call_graph(&contract.functions);

            // Add edges for state variable access
            for var in &contract.state_variables {
                let var_id = format!("{}_{}", contract.name, var.name);

                // Find readers and writers
                let (direct_readers, indirect_readers) = Self::find_readers(
                    &var.name,
                    &contract.functions,
                    &call_graph
                );
                let (direct_writers, indirect_writers) = Self::find_writers(
                    &var.name,
                    &contract.functions,
                    &call_graph
                );

                // Add edges for direct readers
                for reader in &direct_readers {
                    let func_id = format!("{}_{}", contract.name, reader);
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [color=blue, label=\"read\", weight=2];\n",
                        var_id, func_id));
                }

                // Add edges for indirect readers
                for (reader, _chain) in &indirect_readers {
                    let func_id = format!("{}_{}", contract.name, reader);
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [color=blue, style=dashed, label=\"read (indirect)\", weight=1];\n",
                        var_id, func_id));
                }

                // Add edges for direct writers
                for writer in &direct_writers {
                    let func_id = format!("{}_{}", contract.name, writer);
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [color=red, label=\"write\", weight=2];\n",
                        func_id, var_id));
                }

                // Add edges for indirect writers
                for (writer, _chain) in &indirect_writers {
                    let func_id = format!("{}_{}", contract.name, writer);
                    dot.push_str(&format!("  \"{}\" -> \"{}\" [color=red, style=dashed, label=\"write (indirect)\", weight=1];\n",
                        func_id, var_id));
                }
            }
        }

        dot.push_str("}\n");

        fs::write(output_path, dot)?;
        Ok(())
    }

    /// Generate DOT file for contract interaction graph
    /// Shows which contracts interact with each other
    pub fn generate_contract_interaction_graph(contracts: &[ContractInfo], output_path: &Path) -> anyhow::Result<()> {
        let mut dot = String::new();

        dot.push_str("digraph ContractInteractions {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=component, style=filled, fillcolor=lightblue];\n");
        dot.push_str("  graph [fontname=\"Arial\", fontsize=14];\n");
        dot.push_str("  node [fontname=\"Arial\", fontsize=12];\n");
        dot.push_str("  edge [fontname=\"Arial\", fontsize=10];\n\n");

        // Add contract nodes
        for contract in contracts {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", contract.name, contract.name));
        }

        dot.push('\n');

        // Build interface to implementation mapping
        let mut interface_map: HashMap<String, String> = HashMap::new();
        for contract in contracts {
            // Check contract name patterns - if a contract implements an interface,
            // we can infer it from common patterns or explicit inheritance
            // For now, we'll check if contract name contains the interface name without "I"
            for other_contract in contracts {
                if other_contract.name != contract.name {
                    // Check if contract implements interface (simple heuristic)
                    // E.g., GuaranteedMinimumPayoutCalculator implements IPayoutCalculator
                    let interface_candidates = vec![
                        format!("I{}", contract.name), // IJackpot
                        format!("I{}", contract.name.replace("Guaranteed", "").replace("Minimum", "")), // Various patterns
                    ];

                    for candidate in interface_candidates {
                        if other_contract.name.contains(candidate.trim_start_matches('I')) {
                            interface_map.insert(candidate, contract.name.clone());
                        }
                    }
                }
            }
        }

        // Track interactions between contracts
        let mut interactions: HashMap<(String, String), Vec<String>> = HashMap::new();

        for contract in contracts {
            for func in &contract.functions {
                for ext_call in &func.external_calls {
                    // Try to resolve target contract
                    let target = if let Some(ref target_contract) = ext_call.target_contract {
                        // Already resolved
                        target_contract.clone()
                    } else {
                        // Not resolved - try to infer from interface type
                        // Strip "I" prefix from interface name and search for implementation
                        let interface_name = ext_call.target_type.trim_start_matches('I');
                        let mut found_impl = None;

                        for impl_contract in contracts {
                            // Check if contract name contains the interface name (case-insensitive)
                            // E.g., "GuaranteedMinimumPayoutCalculator" contains "PayoutCalculator"
                            if impl_contract.name.to_lowercase().contains(&interface_name.to_lowercase()) {
                                found_impl = Some(impl_contract.name.clone());
                                break;
                            }
                        }

                        found_impl.unwrap_or_else(|| ext_call.target_type.clone())
                    };

                    if target != contract.name {
                        let key = (contract.name.clone(), target.clone());
                        interactions.entry(key)
                            .or_default()
                            .push(ext_call.target_function.clone());
                    }
                }
            }
        }

        // Add edges for contract interactions
        for ((from, to), functions) in interactions {
            // Show all function calls, separated by newlines for readability
            let label = functions.join("\\n");

            dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                from, to, label));
        }

        dot.push_str("}\n");

        fs::write(output_path, dot)?;
        Ok(())
    }

    // Helper functions (same as in json_reports.rs)

    fn build_call_graph(functions: &[FunctionDef]) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for func in functions {
            if !func.calls_functions.is_empty() {
                graph.insert(func.name.clone(), func.calls_functions.clone());
            }
        }
        graph
    }

    fn find_writers(
        var_name: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> (Vec<String>, Vec<(String, Vec<String>)>) {
        let mut direct = Vec::new();
        let mut indirect = Vec::new();

        for func in functions {
            if func.modifies_states.contains(&var_name.to_string())
                || func.modifies_state_fields.iter().any(|f| f.starts_with(&format!("{}.", var_name))) {
                direct.push(func.name.clone());
            }
        }

        for func in functions {
            if !direct.contains(&func.name) {
                if let Some(chain) = Self::find_call_chain_to_targets(&func.name, &direct, call_graph) {
                    indirect.push((func.name.clone(), chain));
                }
            }
        }

        (direct, indirect)
    }

    fn find_readers(
        var_name: &str,
        functions: &[FunctionDef],
        call_graph: &HashMap<String, Vec<String>>,
    ) -> (Vec<String>, Vec<(String, Vec<String>)>) {
        let mut direct = Vec::new();
        let mut indirect = Vec::new();

        for func in functions {
            if func.reads_states.contains(&var_name.to_string()) {
                direct.push(func.name.clone());
            }
        }

        for func in functions {
            if !direct.contains(&func.name) {
                if let Some(chain) = Self::find_call_chain_to_targets(&func.name, &direct, call_graph) {
                    indirect.push((func.name.clone(), chain));
                }
            }
        }

        (direct, indirect)
    }

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

    /// Generate DOT file for cross-contract state variable dependencies
    /// Shows how state variables are accessed/modified across contract boundaries
    pub fn generate_cross_contract_state_dependencies(contracts: &[ContractInfo], output_path: &Path) -> anyhow::Result<()> {
        let mut dot = String::new();

        dot.push_str("digraph CrossContractStateDependencies {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  compound=true;\n");
        dot.push_str("  ranksep=2.0;\n");
        dot.push_str("  nodesep=1.0;\n");
        dot.push_str("  graph [fontname=\"Arial\", fontsize=14];\n");
        dot.push_str("  node [fontname=\"Arial\", fontsize=11];\n");
        dot.push_str("  edge [fontname=\"Arial\", fontsize=9];\n\n");

        // Add all state variables as nodes (labeled with Contract.variable)
        dot.push_str("  // State Variables\n");
        for contract in contracts {
            for var in &contract.state_variables {
                let shape = if var.is_constant || var.is_immutable {
                    "hexagon"
                } else {
                    "box"
                };

                let color = if var.is_constant {
                    "lightgray"
                } else if var.is_immutable {
                    "lightblue"
                } else {
                    "lightyellow"
                };

                let node_id = format!("{}_{}", contract.name, var.name);
                let label = format!("{}.{}", contract.name, var.name);
                dot.push_str(&format!("  \"{}\" [label=\"{}\", shape={}, style=filled, fillcolor={}];\n",
                    node_id, label, shape, color));
            }
        }

        dot.push_str("\n  // Cross-Contract Dependencies\n");

        // For each contract, check external calls and their state modifications
        for contract in contracts {
            for func in &contract.functions {
                for ext_call in &func.external_calls {
                    // Only process if we know the target contract
                    if let Some(ref target_contract_name) = ext_call.target_contract {

                        // Create edges for state variables that the external function modifies
                        for modified_var in &ext_call.target_modifies_states {
                            let target_node_id = format!("{}_{}", target_contract_name, modified_var);
                            let source_label = format!("{}.{}", contract.name, func.name);

                            dot.push_str(&format!(
                                "  \"{}\" [label=\"{}\", shape=ellipse, style=filled, fillcolor=lightgreen];\n",
                                source_label.replace(".", "_"), source_label
                            ));

                            dot.push_str(&format!(
                                "  \"{}\" -> \"{}\" [color=red, label=\"modifies via {}\", penwidth=2.0];\n",
                                source_label.replace(".", "_"),
                                target_node_id,
                                ext_call.target_function
                            ));
                        }

                        // Create edges for state variables that the external function reads
                        for read_var in &ext_call.target_reads_states {
                            let target_node_id = format!("{}_{}", target_contract_name, read_var);
                            let source_label = format!("{}.{}", contract.name, func.name);

                            dot.push_str(&format!(
                                "  \"{}\" [label=\"{}\", shape=ellipse, style=filled, fillcolor=lightgreen];\n",
                                source_label.replace(".", "_"), source_label
                            ));

                            dot.push_str(&format!(
                                "  \"{}\" -> \"{}\" [color=blue, label=\"reads via {}\", style=dashed];\n",
                                source_label.replace(".", "_"),
                                target_node_id,
                                ext_call.target_function
                            ));
                        }
                    }
                }
            }
        }

        dot.push_str("}\n");

        fs::write(output_path, dot)?;
        Ok(())
    }
}
