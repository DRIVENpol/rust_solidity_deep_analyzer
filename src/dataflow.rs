use crate::models::{ContractInfo, FunctionDef};
use serde::{Deserialize, Serialize};
use solang_parser::pt;
use std::collections::{HashMap, HashSet};

/// Represents a source of potentially tainted data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaintSource {
    /// Function parameter from external/public function
    FunctionParameter {
        function_name: String,
        param_index: usize,
        param_name: String,
    },
    /// msg.sender global variable
    MsgSender,
    /// msg.value global variable
    MsgValue,
    /// msg.data global variable
    MsgData,
    /// External contract call return value
    ExternalCallReturn {
        contract_var: String,
        function_name: String,
    },
    /// Array/mapping access with tainted index
    TaintedArrayAccess {
        base_var: String,
    },
}

/// Represents a security-sensitive operation (sink)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaintSink {
    /// State variable modification
    StateModification {
        var_name: String,
        field_path: Option<String>, // For struct fields
    },
    /// External contract call
    ExternalCall {
        target_var: String,
        function_name: String,
    },
    /// Value transfer (call, transfer, send)
    ValueTransfer {
        target_expr: String,
    },
    /// Delegatecall (very dangerous)
    DelegateCall {
        target_expr: String,
    },
    /// Selfdestruct
    SelfDestruct {
        target_expr: String,
    },
    /// Array/mapping index (potential out-of-bounds)
    ArrayIndex {
        array_var: String,
    },
}

/// Represents a tainted data flow from source to sink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintFlow {
    pub source: TaintSource,
    pub sink: TaintSink,
    pub function_name: String,
    pub path: Vec<String>, // Variable names in the flow path
    pub is_validated: bool, // Whether there's a check/validation on the path
    pub severity: TaintSeverity,
}

/// Severity level of a taint flow
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaintSeverity {
    Info,     // Low risk, informational
    Low,      // Minor security concern
    Medium,   // Moderate security concern
    High,     // Significant security risk
    Critical, // Critical security vulnerability
}

/// Represents parameter influence on state variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfluence {
    pub function_name: String,
    pub param_index: usize,
    pub param_name: String,
    pub influenced_state_vars: Vec<String>, // State variables affected by this parameter
    pub influence_type: InfluenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfluenceType {
    Direct,       // Parameter directly assigned to state var
    Arithmetic,   // Parameter used in arithmetic operation on state var
    Conditional,  // Parameter affects which state vars are modified
    IndexAccess,  // Parameter used as array/mapping index
}

/// Main data flow analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowAnalysis {
    pub taint_flows: Vec<TaintFlow>,
    pub parameter_influences: Vec<ParameterInfluence>,
    pub unvalidated_inputs: Vec<UnvalidatedInput>,
}

/// Represents an input that reaches a sensitive operation without validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnvalidatedInput {
    pub source: TaintSource,
    pub sink: TaintSink,
    pub function_name: String,
    pub missing_validations: Vec<String>, // Suggested validations (e.g., "require(amount > 0)")
}

/// Context for analyzing influence type
struct InfluenceContext {
    param_name: String,
    influenced_vars: HashSet<String>,
    state_vars: HashSet<String>,
    has_arithmetic: bool,
    has_conditional: bool,
    has_index_access: bool,
    has_direct: bool,
}

impl TaintSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaintSeverity::Info => "INFO",
            TaintSeverity::Low => "LOW",
            TaintSeverity::Medium => "MEDIUM",
            TaintSeverity::High => "HIGH",
            TaintSeverity::Critical => "CRITICAL",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TaintSeverity::Info => "ℹ️",
            TaintSeverity::Low => "⚠️",
            TaintSeverity::Medium => "🟡",
            TaintSeverity::High => "🟠",
            TaintSeverity::Critical => "🔴",
        }
    }
}

pub struct DataFlowAnalyzer;

impl DataFlowAnalyzer {
    /// Perform complete data flow analysis on a contract
    pub fn analyze(contract: &ContractInfo, function_bodies: &HashMap<String, &pt::FunctionDefinition>) -> DataFlowAnalysis {
        let mut taint_flows = Vec::new();
        let mut parameter_influences = Vec::new();
        let mut unvalidated_inputs = Vec::new();

        // Build state variable set for quick lookup
        let state_vars: HashSet<String> = contract.state_variables
            .iter()
            .map(|v| v.name.clone())
            .collect();

        // Analyze each function
        for func in &contract.functions {
            if let Some(body) = function_bodies.get(&func.name) {
                // Find taint sources in this function
                let sources = Self::identify_taint_sources(func, body);

                // Find taint sinks in this function
                let sinks = Self::identify_taint_sinks(func, body, &state_vars);

                // Analyze parameter influence on state variables
                let influences = Self::analyze_parameter_influence(func, body, &state_vars);
                parameter_influences.extend(influences);

                // Build taint flows from sources to sinks
                let flows = Self::build_taint_flows(func, body, &sources, &sinks, &state_vars);
                taint_flows.extend(flows);

                // Identify unvalidated inputs
                let unvalidated = Self::identify_unvalidated_inputs(func, body, &sources, &sinks);
                unvalidated_inputs.extend(unvalidated);
            }
        }

        DataFlowAnalysis {
            taint_flows,
            parameter_influences,
            unvalidated_inputs,
        }
    }

    /// Identify all taint sources in a function
    fn identify_taint_sources(func: &FunctionDef, body: &pt::FunctionDefinition) -> Vec<TaintSource> {
        let mut sources = Vec::new();

        // Only external/public functions have tainted parameters
        if matches!(func.visibility.as_str(), "external" | "public") {
            // Extract parameter names from the parameter strings
            for (i, param_str) in func.parameters.iter().enumerate() {
                // Parse parameter string like "uint256 amount" or "address indexed user"
                let parts: Vec<&str> = param_str.split_whitespace().collect();
                let param_name = parts.last().unwrap_or(&"").to_string();

                if !param_name.is_empty() {
                    sources.push(TaintSource::FunctionParameter {
                        function_name: func.name.clone(),
                        param_index: i,
                        param_name,
                    });
                }
            }
        }

        // Scan body for msg.sender, msg.value, msg.data, external call returns
        if let Some(stmt) = &body.body {
            Self::scan_for_taint_sources(stmt, &mut sources);
        }

        sources
    }

    /// Scan statement tree for additional taint sources
    fn scan_for_taint_sources(stmt: &pt::Statement, sources: &mut Vec<TaintSource>) {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_for_taint_sources(s, sources);
                }
            }
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_taint_sources(expr, sources);
            }
            pt::Statement::If(_, condition, if_branch, else_branch) => {
                Self::scan_expression_for_taint_sources(condition, sources);
                Self::scan_for_taint_sources(if_branch, sources);
                if let Some(else_stmt) = else_branch {
                    Self::scan_for_taint_sources(else_stmt, sources);
                }
            }
            pt::Statement::While(_, condition, body) => {
                Self::scan_expression_for_taint_sources(condition, sources);
                Self::scan_for_taint_sources(body, sources);
            }
            pt::Statement::DoWhile(_, body, condition) => {
                Self::scan_for_taint_sources(body, sources);
                Self::scan_expression_for_taint_sources(condition, sources);
            }
            pt::Statement::For(_, init, condition, next, body_opt) => {
                if let Some(init_stmt) = init {
                    Self::scan_for_taint_sources(init_stmt, sources);
                }
                if let Some(cond_expr) = condition {
                    Self::scan_expression_for_taint_sources(cond_expr, sources);
                }
                if let Some(next_expr) = next {
                    Self::scan_expression_for_taint_sources(next_expr, sources);
                }
                if let Some(body) = body_opt {
                    Self::scan_for_taint_sources(body, sources);
                }
            }
            pt::Statement::VariableDefinition(_, _, Some(init)) => {
                Self::scan_expression_for_taint_sources(init, sources);
            }
            pt::Statement::Return(_, Some(expr)) => {
                Self::scan_expression_for_taint_sources(expr, sources);
            }
            _ => {}
        }
    }

    /// Scan expression for taint sources (msg.*, external calls)
    fn scan_expression_for_taint_sources(expr: &pt::Expression, sources: &mut Vec<TaintSource>) {
        match expr {
            // msg.sender, msg.value, msg.data
            pt::Expression::MemberAccess(_, base, member) => {
                if let pt::Expression::Variable(ident) = base.as_ref() {
                    if ident.name == "msg" {
                        match member.name.as_str() {
                            "sender" => {
                                if !sources.contains(&TaintSource::MsgSender) {
                                    sources.push(TaintSource::MsgSender);
                                }
                            }
                            "value" => {
                                if !sources.contains(&TaintSource::MsgValue) {
                                    sources.push(TaintSource::MsgValue);
                                }
                            }
                            "data" => {
                                if !sources.contains(&TaintSource::MsgData) {
                                    sources.push(TaintSource::MsgData);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                // Recursively scan base
                Self::scan_expression_for_taint_sources(base, sources);
            }
            // External contract calls (e.g., token.balanceOf(user))
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Check if this is a member access call (contract.function())
                if let pt::Expression::MemberAccess(_, contract_expr, function_name) = func_expr.as_ref() {
                    if let pt::Expression::Variable(contract_ident) = contract_expr.as_ref() {
                        // This is an external call: contractVar.functionName()
                        let source = TaintSource::ExternalCallReturn {
                            contract_var: contract_ident.name.clone(),
                            function_name: function_name.name.clone(),
                        };
                        if !sources.contains(&source) {
                            sources.push(source);
                        }
                    }
                }
                // Recursively scan function expression and arguments
                Self::scan_expression_for_taint_sources(func_expr, sources);
                for arg in args {
                    Self::scan_expression_for_taint_sources(arg, sources);
                }
            }
            // Recursively scan other expression types
            pt::Expression::Assign(_, lhs, rhs)
            | pt::Expression::AssignAdd(_, lhs, rhs)
            | pt::Expression::AssignSubtract(_, lhs, rhs)
            | pt::Expression::AssignMultiply(_, lhs, rhs)
            | pt::Expression::AssignDivide(_, lhs, rhs)
            | pt::Expression::AssignModulo(_, lhs, rhs)
            | pt::Expression::AssignOr(_, lhs, rhs)
            | pt::Expression::AssignAnd(_, lhs, rhs)
            | pt::Expression::AssignXor(_, lhs, rhs)
            | pt::Expression::AssignShiftLeft(_, lhs, rhs)
            | pt::Expression::AssignShiftRight(_, lhs, rhs) => {
                Self::scan_expression_for_taint_sources(lhs, sources);
                Self::scan_expression_for_taint_sources(rhs, sources);
            }
            pt::Expression::Add(_, l, r)
            | pt::Expression::Subtract(_, l, r)
            | pt::Expression::Multiply(_, l, r)
            | pt::Expression::Divide(_, l, r)
            | pt::Expression::Modulo(_, l, r)
            | pt::Expression::Power(_, l, r)
            | pt::Expression::BitwiseOr(_, l, r)
            | pt::Expression::BitwiseAnd(_, l, r)
            | pt::Expression::BitwiseXor(_, l, r)
            | pt::Expression::ShiftLeft(_, l, r)
            | pt::Expression::ShiftRight(_, l, r)
            | pt::Expression::Equal(_, l, r)
            | pt::Expression::NotEqual(_, l, r)
            | pt::Expression::Less(_, l, r)
            | pt::Expression::LessEqual(_, l, r)
            | pt::Expression::More(_, l, r)
            | pt::Expression::MoreEqual(_, l, r)
            | pt::Expression::And(_, l, r)
            | pt::Expression::Or(_, l, r) => {
                Self::scan_expression_for_taint_sources(l, sources);
                Self::scan_expression_for_taint_sources(r, sources);
            }
            pt::Expression::UnaryPlus(_, e)
            | pt::Expression::Negate(_, e)
            | pt::Expression::Not(_, e)
            | pt::Expression::BitwiseNot(_, e)
            | pt::Expression::PreIncrement(_, e)
            | pt::Expression::PreDecrement(_, e)
            | pt::Expression::PostIncrement(_, e)
            | pt::Expression::PostDecrement(_, e) => {
                Self::scan_expression_for_taint_sources(e, sources);
            }
            pt::Expression::Parenthesis(_, e) => {
                Self::scan_expression_for_taint_sources(e, sources);
            }
            pt::Expression::ConditionalOperator(_, cond, true_expr, false_expr) => {
                Self::scan_expression_for_taint_sources(cond, sources);
                Self::scan_expression_for_taint_sources(true_expr, sources);
                Self::scan_expression_for_taint_sources(false_expr, sources);
            }
            pt::Expression::ArraySubscript(_, base, index) => {
                Self::scan_expression_for_taint_sources(base, sources);
                if let Some(idx) = index {
                    Self::scan_expression_for_taint_sources(idx, sources);
                }
            }
            _ => {}
        }
    }

    /// Identify all taint sinks in a function
    fn identify_taint_sinks(func: &FunctionDef, body: &pt::FunctionDefinition, state_vars: &HashSet<String>) -> Vec<TaintSink> {
        let mut sinks = Vec::new();

        // State modifications are sinks
        for var_name in &func.modifies_states {
            sinks.push(TaintSink::StateModification {
                var_name: var_name.clone(),
                field_path: None,
            });
        }

        // Field modifications are sinks
        for field_path in &func.modifies_state_fields {
            // Extract base variable name
            let base_var = field_path.split('.').next().unwrap_or(field_path).to_string();
            if state_vars.contains(&base_var) {
                sinks.push(TaintSink::StateModification {
                    var_name: base_var,
                    field_path: Some(field_path.clone()),
                });
            }
        }

        // External calls are sinks
        for ext_call in &func.external_calls {
            sinks.push(TaintSink::ExternalCall {
                target_var: ext_call.target_variable.clone(),
                function_name: ext_call.target_function.clone(),
            });
        }

        // Scan body for delegatecall, selfdestruct, transfer/send
        if let Some(stmt) = &body.body {
            Self::scan_for_taint_sinks(stmt, &mut sinks);
        }

        sinks
    }

    /// Scan statement tree for dangerous sinks
    fn scan_for_taint_sinks(stmt: &pt::Statement, sinks: &mut Vec<TaintSink>) {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_for_taint_sinks(s, sinks);
                }
            }
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_taint_sinks(expr, sinks);
            }
            pt::Statement::If(_, condition, if_branch, else_branch) => {
                Self::scan_expression_for_taint_sinks(condition, sinks);
                Self::scan_for_taint_sinks(if_branch, sinks);
                if let Some(else_stmt) = else_branch {
                    Self::scan_for_taint_sinks(else_stmt, sinks);
                }
            }
            pt::Statement::While(_, condition, body) => {
                Self::scan_expression_for_taint_sinks(condition, sinks);
                Self::scan_for_taint_sinks(body, sinks);
            }
            pt::Statement::DoWhile(_, body, condition) => {
                Self::scan_for_taint_sinks(body, sinks);
                Self::scan_expression_for_taint_sinks(condition, sinks);
            }
            pt::Statement::For(_, init, condition, next, body_opt) => {
                if let Some(init_stmt) = init {
                    Self::scan_for_taint_sinks(init_stmt, sinks);
                }
                if let Some(cond_expr) = condition {
                    Self::scan_expression_for_taint_sinks(cond_expr, sinks);
                }
                if let Some(next_expr) = next {
                    Self::scan_expression_for_taint_sinks(next_expr, sinks);
                }
                if let Some(body) = body_opt {
                    Self::scan_for_taint_sinks(body, sinks);
                }
            }
            pt::Statement::VariableDefinition(_, _, Some(init)) => {
                Self::scan_expression_for_taint_sinks(init, sinks);
            }
            pt::Statement::Return(_, Some(expr)) => {
                Self::scan_expression_for_taint_sinks(expr, sinks);
            }
            _ => {}
        }
    }

    /// Scan expression for dangerous sinks (delegatecall, selfdestruct, transfers)
    fn scan_expression_for_taint_sinks(expr: &pt::Expression, sinks: &mut Vec<TaintSink>) {
        match expr {
            pt::Expression::FunctionCall(_, func_expr, args) => {
                // Check for dangerous function calls
                match func_expr.as_ref() {
                    // Direct calls: selfdestruct(addr), suicide(addr)
                    pt::Expression::Variable(ident) => {
                        match ident.name.as_str() {
                            "selfdestruct" | "suicide" => {
                                let target = if let Some(arg) = args.first() {
                                    Self::expression_to_string(arg)
                                } else {
                                    "unknown".to_string()
                                };
                                sinks.push(TaintSink::SelfDestruct {
                                    target_expr: target,
                                });
                            }
                            _ => {}
                        }
                    }
                    // Member access calls: addr.delegatecall(...), addr.call{value: ...}(...)
                    pt::Expression::MemberAccess(_, base, member) => {
                        let base_expr = Self::expression_to_string(base);

                        match member.name.as_str() {
                            "delegatecall" => {
                                sinks.push(TaintSink::DelegateCall {
                                    target_expr: base_expr,
                                });
                            }
                            "call" | "staticcall" | "callcode" => {
                                // Check if this is a value transfer (has {value: ...})
                                // For now, treat all .call() as potential sinks
                                sinks.push(TaintSink::ValueTransfer {
                                    target_expr: base_expr.clone(),
                                });
                            }
                            "transfer" | "send" => {
                                sinks.push(TaintSink::ValueTransfer {
                                    target_expr: base_expr,
                                });
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                // Recursively scan function expression and arguments
                Self::scan_expression_for_taint_sinks(func_expr, sinks);
                for arg in args {
                    Self::scan_expression_for_taint_sinks(arg, sinks);
                }
            }
            // Recursively scan other expression types
            pt::Expression::Assign(_, lhs, rhs)
            | pt::Expression::AssignAdd(_, lhs, rhs)
            | pt::Expression::AssignSubtract(_, lhs, rhs)
            | pt::Expression::AssignMultiply(_, lhs, rhs)
            | pt::Expression::AssignDivide(_, lhs, rhs)
            | pt::Expression::AssignModulo(_, lhs, rhs)
            | pt::Expression::AssignOr(_, lhs, rhs)
            | pt::Expression::AssignAnd(_, lhs, rhs)
            | pt::Expression::AssignXor(_, lhs, rhs)
            | pt::Expression::AssignShiftLeft(_, lhs, rhs)
            | pt::Expression::AssignShiftRight(_, lhs, rhs) => {
                Self::scan_expression_for_taint_sinks(lhs, sinks);
                Self::scan_expression_for_taint_sinks(rhs, sinks);
            }
            pt::Expression::Add(_, l, r)
            | pt::Expression::Subtract(_, l, r)
            | pt::Expression::Multiply(_, l, r)
            | pt::Expression::Divide(_, l, r)
            | pt::Expression::Modulo(_, l, r)
            | pt::Expression::Power(_, l, r)
            | pt::Expression::BitwiseOr(_, l, r)
            | pt::Expression::BitwiseAnd(_, l, r)
            | pt::Expression::BitwiseXor(_, l, r)
            | pt::Expression::ShiftLeft(_, l, r)
            | pt::Expression::ShiftRight(_, l, r)
            | pt::Expression::Equal(_, l, r)
            | pt::Expression::NotEqual(_, l, r)
            | pt::Expression::Less(_, l, r)
            | pt::Expression::LessEqual(_, l, r)
            | pt::Expression::More(_, l, r)
            | pt::Expression::MoreEqual(_, l, r)
            | pt::Expression::And(_, l, r)
            | pt::Expression::Or(_, l, r) => {
                Self::scan_expression_for_taint_sinks(l, sinks);
                Self::scan_expression_for_taint_sinks(r, sinks);
            }
            pt::Expression::UnaryPlus(_, e)
            | pt::Expression::Negate(_, e)
            | pt::Expression::Not(_, e)
            | pt::Expression::BitwiseNot(_, e)
            | pt::Expression::PreIncrement(_, e)
            | pt::Expression::PreDecrement(_, e)
            | pt::Expression::PostIncrement(_, e)
            | pt::Expression::PostDecrement(_, e) => {
                Self::scan_expression_for_taint_sinks(e, sinks);
            }
            pt::Expression::Parenthesis(_, e) => {
                Self::scan_expression_for_taint_sinks(e, sinks);
            }
            pt::Expression::ConditionalOperator(_, cond, true_expr, false_expr) => {
                Self::scan_expression_for_taint_sinks(cond, sinks);
                Self::scan_expression_for_taint_sinks(true_expr, sinks);
                Self::scan_expression_for_taint_sinks(false_expr, sinks);
            }
            pt::Expression::ArraySubscript(_, base, index) => {
                Self::scan_expression_for_taint_sinks(base, sinks);
                if let Some(idx) = index {
                    Self::scan_expression_for_taint_sinks(idx, sinks);
                }
            }
            pt::Expression::MemberAccess(_, base, _) => {
                Self::scan_expression_for_taint_sinks(base, sinks);
            }
            _ => {}
        }
    }

    /// Convert expression to string for display
    fn expression_to_string(expr: &pt::Expression) -> String {
        match expr {
            pt::Expression::Variable(ident) => ident.name.clone(),
            pt::Expression::MemberAccess(_, base, member) => {
                format!("{}.{}", Self::expression_to_string(base), member.name)
            }
            pt::Expression::ArraySubscript(_, base, _) => {
                format!("{}[...]", Self::expression_to_string(base))
            }
            _ => "complex_expr".to_string(),
        }
    }

    /// Analyze how parameters influence state variable modifications
    fn analyze_parameter_influence(
        func: &FunctionDef,
        body: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
    ) -> Vec<ParameterInfluence> {
        let mut influences = Vec::new();

        // Extract parameter names
        let param_names: Vec<String> = func.parameters.iter()
            .map(|param_str| {
                let parts: Vec<&str> = param_str.split_whitespace().collect();
                parts.last().unwrap_or(&"").to_string()
            })
            .filter(|name| !name.is_empty())
            .collect();

        // Build variable flow graph
        let flow_graph = Self::build_variable_flow_graph(body, state_vars);

        // For each parameter, trace which state variables it influences
        for (i, param_name) in param_names.iter().enumerate() {
            let influenced_vars = Self::trace_variable_influence(param_name, &flow_graph, state_vars);

            if !influenced_vars.is_empty() {
                // Determine influence type
                let influence_type = Self::determine_influence_type(param_name, &influenced_vars, body, state_vars);

                influences.push(ParameterInfluence {
                    function_name: func.name.clone(),
                    param_index: i,
                    param_name: param_name.clone(),
                    influenced_state_vars: influenced_vars,
                    influence_type,
                });
            }
        }

        influences
    }

    /// Build taint flows from sources to sinks
    fn build_taint_flows(
        func: &FunctionDef,
        body: &pt::FunctionDefinition,
        sources: &[TaintSource],
        sinks: &[TaintSink],
        state_vars: &HashSet<String>,
    ) -> Vec<TaintFlow> {
        let mut flows = Vec::new();

        // Build variable flow graph
        let flow_graph = Self::build_variable_flow_graph(body, state_vars);

        // Detect validations in the function
        let validations = Self::detect_validations(body);

        // For each source, check if it actually flows to any sink
        for source in sources {
            let source_var = match source {
                TaintSource::FunctionParameter { param_name, .. } => param_name.clone(),
                TaintSource::MsgSender => "msg.sender".to_string(),
                TaintSource::MsgValue => "msg.value".to_string(),
                TaintSource::MsgData => "msg.data".to_string(),
                _ => continue,
            };

            // Trace which variables this source flows to
            let influenced_vars = Self::trace_variable_influence(&source_var, &flow_graph, state_vars);

            // Check each sink to see if the source flows to it
            for sink in sinks {
                let sink_var = match sink {
                    TaintSink::StateModification { var_name, .. } => var_name.clone(),
                    TaintSink::ExternalCall { target_var, .. } => target_var.clone(),
                    TaintSink::ValueTransfer { target_expr } => target_expr.clone(),
                    TaintSink::DelegateCall { target_expr } => target_expr.clone(),
                    TaintSink::SelfDestruct { target_expr } => target_expr.clone(),
                    TaintSink::ArrayIndex { array_var } => array_var.clone(),
                };

                // Only create flow if source actually influences sink
                if influenced_vars.contains(&sink_var) {
                    // Check if there's validation on this flow
                    let is_validated = validations.iter().any(|v| v.contains(&source_var));

                    let severity = Self::assess_severity(source, sink);

                    // Build the path
                    let path = Self::build_flow_path(&source_var, &sink_var, &flow_graph);

                    flows.push(TaintFlow {
                        source: source.clone(),
                        sink: sink.clone(),
                        function_name: func.name.clone(),
                        path,
                        is_validated,
                        severity,
                    });
                }
            }
        }

        flows
    }

    /// Identify inputs that reach sensitive operations without validation
    /// Note: This is now deprecated in favor of validation detection in build_taint_flows()
    /// Kept for backwards compatibility with DataFlowAnalysis structure
    fn identify_unvalidated_inputs(
        _func: &FunctionDef,
        _body: &pt::FunctionDefinition,
        _sources: &[TaintSource],
        _sinks: &[TaintSink],
    ) -> Vec<UnvalidatedInput> {
        // Validation detection is now integrated into build_taint_flows()
        // which sets the is_validated field on TaintFlow objects
        // This function is kept empty to maintain API compatibility
        Vec::new()
    }

    /// Assess the severity of a taint flow based on source and sink types
    fn assess_severity(source: &TaintSource, sink: &TaintSink) -> TaintSeverity {
        match (source, sink) {
            // Critical: User-controlled delegatecall
            (TaintSource::FunctionParameter { .. }, TaintSink::DelegateCall { .. }) => {
                TaintSeverity::Critical
            }
            // Critical: User-controlled selfdestruct
            (TaintSource::FunctionParameter { .. }, TaintSink::SelfDestruct { .. }) => {
                TaintSeverity::Critical
            }
            // High: User-controlled external call
            (TaintSource::FunctionParameter { .. }, TaintSink::ExternalCall { .. }) => {
                TaintSeverity::High
            }
            // High: User-controlled value transfer
            (TaintSource::FunctionParameter { .. }, TaintSink::ValueTransfer { .. }) => {
                TaintSeverity::High
            }
            // Medium: User-controlled state modification
            (TaintSource::FunctionParameter { .. }, TaintSink::StateModification { .. }) => {
                TaintSeverity::Medium
            }
            // Medium: msg.sender affecting critical operations
            (TaintSource::MsgSender, TaintSink::DelegateCall { .. }) => {
                TaintSeverity::Medium
            }
            // Low: Other combinations
            _ => TaintSeverity::Low,
        }
    }

    /// Build a variable flow graph from function body
    /// Maps: variable -> set of variables it flows to
    fn build_variable_flow_graph(
        body: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
    ) -> HashMap<String, HashSet<String>> {
        let mut flow_graph: HashMap<String, HashSet<String>> = HashMap::new();

        if let Some(stmt) = &body.body {
            Self::scan_statement_for_flows(stmt, &mut flow_graph, state_vars);
        }

        flow_graph
    }

    /// Scan statement for variable assignments and flows
    fn scan_statement_for_flows(
        stmt: &pt::Statement,
        flow_graph: &mut HashMap<String, HashSet<String>>,
        state_vars: &HashSet<String>,
    ) {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_statement_for_flows(s, flow_graph, state_vars);
                }
            }
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_flows(expr, flow_graph, state_vars);
            }
            pt::Statement::If(_, _, if_branch, else_branch) => {
                Self::scan_statement_for_flows(if_branch, flow_graph, state_vars);
                if let Some(else_stmt) = else_branch {
                    Self::scan_statement_for_flows(else_stmt, flow_graph, state_vars);
                }
            }
            pt::Statement::While(_, _, body) => {
                Self::scan_statement_for_flows(body, flow_graph, state_vars);
            }
            pt::Statement::DoWhile(_, body, _) => {
                Self::scan_statement_for_flows(body, flow_graph, state_vars);
            }
            pt::Statement::For(_, _, _, _, Some(body)) => {
                Self::scan_statement_for_flows(body, flow_graph, state_vars);
            }
            pt::Statement::VariableDefinition(_, var_decl, Some(init)) => {
                // Track: init_vars -> declared_var
                let var_name = Self::extract_var_name_from_declaration(var_decl);
                if !var_name.is_empty() {
                    let source_vars = Self::extract_variables_from_expression(init);
                    for source in source_vars {
                        flow_graph
                            .entry(source)
                            .or_default()
                            .insert(var_name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    /// Scan expression for variable flows (assignments)
    fn scan_expression_for_flows(
        expr: &pt::Expression,
        flow_graph: &mut HashMap<String, HashSet<String>>,
        state_vars: &HashSet<String>,
    ) {
        match expr {
            pt::Expression::Assign(_, lhs, rhs) => {
                // Extract target variable from LHS
                let target_var = Self::extract_lvalue_var(lhs, state_vars);

                // Extract source variables from RHS
                let source_vars = Self::extract_variables_from_expression(rhs);

                // Add flows: source -> target
                for source in source_vars {
                    flow_graph
                        .entry(source)
                        .or_default()
                        .insert(target_var.clone());
                }
            }
            pt::Expression::AssignAdd(_, lhs, rhs)
            | pt::Expression::AssignSubtract(_, lhs, rhs)
            | pt::Expression::AssignMultiply(_, lhs, rhs)
            | pt::Expression::AssignDivide(_, lhs, rhs)
            | pt::Expression::AssignModulo(_, lhs, rhs)
            | pt::Expression::AssignOr(_, lhs, rhs)
            | pt::Expression::AssignAnd(_, lhs, rhs)
            | pt::Expression::AssignXor(_, lhs, rhs)
            | pt::Expression::AssignShiftLeft(_, lhs, rhs)
            | pt::Expression::AssignShiftRight(_, lhs, rhs) => {
                let target_var = Self::extract_lvalue_var(lhs, state_vars);
                let source_vars = Self::extract_variables_from_expression(rhs);

                // Compound assignment also flows from target to itself
                flow_graph
                    .entry(target_var.clone())
                    .or_default()
                    .insert(target_var.clone());

                for source in source_vars {
                    flow_graph
                        .entry(source)
                        .or_default()
                        .insert(target_var.clone());
                }
            }
            _ => {}
        }
    }

    /// Extract variable name from LHS of assignment
    fn extract_lvalue_var(expr: &pt::Expression, _state_vars: &HashSet<String>) -> String {
        match expr {
            pt::Expression::Variable(ident) => ident.name.clone(),
            pt::Expression::MemberAccess(_, base, _) => {
                // For x.field, extract x
                Self::extract_lvalue_var(base, _state_vars)
            }
            pt::Expression::ArraySubscript(_, base, _) => {
                // For arr[i], extract arr
                Self::extract_lvalue_var(base, _state_vars)
            }
            _ => String::new(),
        }
    }

    /// Extract all variable names from an expression
    fn extract_variables_from_expression(expr: &pt::Expression) -> Vec<String> {
        let mut vars = Vec::new();
        Self::collect_variables_recursive(expr, &mut vars);
        vars
    }

    /// Recursively collect all variable names from expression
    fn collect_variables_recursive(expr: &pt::Expression, vars: &mut Vec<String>) {
        match expr {
            pt::Expression::Variable(ident) => {
                vars.push(ident.name.clone());
            }
            pt::Expression::MemberAccess(_, base, member) => {
                // For state variables or struct access, use base name
                if let pt::Expression::Variable(ident) = base.as_ref() {
                    vars.push(ident.name.clone());
                } else {
                    Self::collect_variables_recursive(base, vars);
                }
                // Also track the full path for field access
                let base_name = Self::get_base_var_name(base);
                if !base_name.is_empty() {
                    vars.push(format!("{}.{}", base_name, member.name));
                }
            }
            pt::Expression::ArraySubscript(_, base, index) => {
                Self::collect_variables_recursive(base, vars);
                if let Some(idx) = index {
                    Self::collect_variables_recursive(idx, vars);
                }
            }
            pt::Expression::FunctionCall(_, func, args) => {
                Self::collect_variables_recursive(func, vars);
                for arg in args {
                    Self::collect_variables_recursive(arg, vars);
                }
            }
            // Binary operations
            pt::Expression::Add(_, l, r)
            | pt::Expression::Subtract(_, l, r)
            | pt::Expression::Multiply(_, l, r)
            | pt::Expression::Divide(_, l, r)
            | pt::Expression::Modulo(_, l, r)
            | pt::Expression::Power(_, l, r)
            | pt::Expression::BitwiseOr(_, l, r)
            | pt::Expression::BitwiseAnd(_, l, r)
            | pt::Expression::BitwiseXor(_, l, r)
            | pt::Expression::ShiftLeft(_, l, r)
            | pt::Expression::ShiftRight(_, l, r)
            | pt::Expression::Equal(_, l, r)
            | pt::Expression::NotEqual(_, l, r)
            | pt::Expression::Less(_, l, r)
            | pt::Expression::LessEqual(_, l, r)
            | pt::Expression::More(_, l, r)
            | pt::Expression::MoreEqual(_, l, r)
            | pt::Expression::And(_, l, r)
            | pt::Expression::Or(_, l, r) => {
                Self::collect_variables_recursive(l, vars);
                Self::collect_variables_recursive(r, vars);
            }
            // Unary operations
            pt::Expression::UnaryPlus(_, e)
            | pt::Expression::Negate(_, e)
            | pt::Expression::Not(_, e)
            | pt::Expression::BitwiseNot(_, e)
            | pt::Expression::PreIncrement(_, e)
            | pt::Expression::PreDecrement(_, e)
            | pt::Expression::PostIncrement(_, e)
            | pt::Expression::PostDecrement(_, e) => {
                Self::collect_variables_recursive(e, vars);
            }
            pt::Expression::Parenthesis(_, e) => {
                Self::collect_variables_recursive(e, vars);
            }
            pt::Expression::ConditionalOperator(_, cond, true_expr, false_expr) => {
                Self::collect_variables_recursive(cond, vars);
                Self::collect_variables_recursive(true_expr, vars);
                Self::collect_variables_recursive(false_expr, vars);
            }
            _ => {}
        }
    }

    /// Get base variable name from expression
    fn get_base_var_name(expr: &pt::Expression) -> String {
        match expr {
            pt::Expression::Variable(ident) => ident.name.clone(),
            pt::Expression::MemberAccess(_, base, _) => Self::get_base_var_name(base),
            pt::Expression::ArraySubscript(_, base, _) => Self::get_base_var_name(base),
            _ => String::new(),
        }
    }

    /// Extract variable name from variable declaration
    fn extract_var_name_from_declaration(decl: &pt::VariableDeclaration) -> String {
        decl.name.as_ref().map(|id| id.name.clone()).unwrap_or_default()
    }

    /// Trace which variables a source influences through the flow graph
    fn trace_variable_influence(
        source: &str,
        flow_graph: &HashMap<String, HashSet<String>>,
        state_vars: &HashSet<String>,
    ) -> Vec<String> {
        let mut influenced = HashSet::new();
        let mut to_visit = vec![source.to_string()];
        let mut visited = HashSet::new();

        // BFS to find all variables influenced by source
        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if let Some(targets) = flow_graph.get(&current) {
                for target in targets {
                    // Track state variables and continue traversal
                    if state_vars.contains(target) || state_vars.contains(&target.split('.').next().unwrap_or("").to_string()) {
                        influenced.insert(target.clone());
                    }
                    to_visit.push(target.clone());
                }
            }
        }

        influenced.into_iter().collect()
    }

    /// Build the flow path from source to sink
    fn build_flow_path(
        source: &str,
        sink: &str,
        flow_graph: &HashMap<String, HashSet<String>>,
    ) -> Vec<String> {
        let mut to_visit = vec![(source.to_string(), vec![source.to_string()])];
        let mut visited = HashSet::new();

        // BFS to find shortest path
        while let Some((current, current_path)) = to_visit.pop() {
            if current == sink {
                return current_path;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if let Some(targets) = flow_graph.get(&current) {
                for target in targets {
                    let mut new_path = current_path.clone();
                    new_path.push(target.clone());
                    to_visit.push((target.clone(), new_path));
                }
            }
        }

        // If no path found, return source and sink
        vec![source.to_string(), sink.to_string()]
    }

    /// Detect validations (require, revert, if checks) in function body
    fn detect_validations(body: &pt::FunctionDefinition) -> Vec<String> {
        let mut validations = Vec::new();

        if let Some(stmt) = &body.body {
            Self::scan_for_validations(stmt, &mut validations);
        }

        validations
    }

    /// Recursively scan for validation statements
    fn scan_for_validations(stmt: &pt::Statement, validations: &mut Vec<String>) {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::scan_for_validations(s, validations);
                }
            }
            pt::Statement::Expression(_, expr) => {
                Self::scan_expression_for_validations(expr, validations);
            }
            pt::Statement::If(_, condition, if_branch, else_branch) => {
                // If statement with revert is a validation
                if Self::contains_revert(if_branch) {
                    let validated_vars = Self::extract_variables_from_expression(condition);
                    validations.extend(validated_vars);
                }
                Self::scan_for_validations(if_branch, validations);
                if let Some(else_stmt) = else_branch {
                    Self::scan_for_validations(else_stmt, validations);
                }
            }
            pt::Statement::While(_, _, body) => {
                Self::scan_for_validations(body, validations);
            }
            pt::Statement::DoWhile(_, body, _) => {
                Self::scan_for_validations(body, validations);
            }
            pt::Statement::For(_, _, _, _, Some(body)) => {
                Self::scan_for_validations(body, validations);
            }
            _ => {}
        }
    }

    /// Scan expression for require() calls
    fn scan_expression_for_validations(expr: &pt::Expression, validations: &mut Vec<String>) {
        if let pt::Expression::FunctionCall(_, func, args) = expr {
            // Check if this is a require() call
            if let pt::Expression::Variable(ident) = func.as_ref() {
                if ident.name == "require" || ident.name == "assert" {
                    // Extract variables from the condition
                    if let Some(condition) = args.first() {
                        let validated_vars = Self::extract_variables_from_expression(condition);
                        validations.extend(validated_vars);
                    }
                }
            }
        }
    }

    /// Check if a statement contains a revert
    fn contains_revert(stmt: &pt::Statement) -> bool {
        match stmt {
            pt::Statement::Revert { .. } => true,
            pt::Statement::Block { statements, .. } => {
                statements.iter().any(Self::contains_revert)
            }
            pt::Statement::Expression(_, expr) => Self::expression_is_revert(expr),
            _ => false,
        }
    }

    /// Check if expression is a revert call
    fn expression_is_revert(expr: &pt::Expression) -> bool {
        match expr {
            pt::Expression::FunctionCall(_, func, _) => {
                if let pt::Expression::Variable(ident) = func.as_ref() {
                    ident.name == "revert"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Determine the type of influence based on how parameter affects state var
    fn determine_influence_type(
        param_name: &str,
        influenced_vars: &[String],
        body: &pt::FunctionDefinition,
        state_vars: &HashSet<String>,
    ) -> InfluenceType {
        // Scan the function body to find how the parameter influences state variables
        if let Some(stmt) = &body.body {
            let mut influence_context = InfluenceContext {
                param_name: param_name.to_string(),
                influenced_vars: influenced_vars.iter().cloned().collect(),
                state_vars: state_vars.clone(),
                has_arithmetic: false,
                has_conditional: false,
                has_index_access: false,
                has_direct: false,
            };

            Self::analyze_influence_in_statement(stmt, &mut influence_context);

            // Prioritize influence types: IndexAccess > Conditional > Arithmetic > Direct
            if influence_context.has_index_access {
                return InfluenceType::IndexAccess;
            }
            if influence_context.has_conditional {
                return InfluenceType::Conditional;
            }
            if influence_context.has_arithmetic {
                return InfluenceType::Arithmetic;
            }
        }

        InfluenceType::Direct
    }

    /// Analyze statements to determine influence type
    fn analyze_influence_in_statement(stmt: &pt::Statement, context: &mut InfluenceContext) {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                for s in statements {
                    Self::analyze_influence_in_statement(s, context);
                }
            }
            pt::Statement::Expression(_, expr) => {
                Self::analyze_influence_in_expression(expr, context, false);
            }
            pt::Statement::If(_, condition, if_branch, else_branch) => {
                // Check if parameter is in condition
                if Self::expression_contains_var(condition, &context.param_name) {
                    // Check if any influenced state vars are modified in branches
                    if Self::statement_modifies_influenced_vars(if_branch, context) {
                        context.has_conditional = true;
                    }
                    if let Some(else_stmt) = else_branch {
                        if Self::statement_modifies_influenced_vars(else_stmt, context) {
                            context.has_conditional = true;
                        }
                    }
                }
                Self::analyze_influence_in_statement(if_branch, context);
                if let Some(else_stmt) = else_branch {
                    Self::analyze_influence_in_statement(else_stmt, context);
                }
            }
            pt::Statement::While(_, _, body) | pt::Statement::DoWhile(_, body, _) => {
                Self::analyze_influence_in_statement(body, context);
            }
            pt::Statement::For(_, _, _, _, Some(body)) => {
                Self::analyze_influence_in_statement(body, context);
            }
            pt::Statement::VariableDefinition(_, _, Some(init)) => {
                Self::analyze_influence_in_expression(init, context, false);
            }
            _ => {}
        }
    }

    /// Analyze expressions to determine influence type
    fn analyze_influence_in_expression(expr: &pt::Expression, context: &mut InfluenceContext, in_assignment: bool) {
        match expr {
            pt::Expression::Assign(_, lhs, rhs) => {
                // Check if LHS is an influenced state variable
                let lhs_var = Self::get_base_var_name(lhs);
                if context.state_vars.contains(&lhs_var) && context.influenced_vars.contains(&lhs_var) {
                    // Analyze RHS to determine how parameter influences it
                    Self::analyze_influence_in_expression(rhs, context, true);

                    // Check if it's a direct assignment (param = stateVar or stateVar = param)
                    if let pt::Expression::Variable(ident) = rhs.as_ref() {
                        if ident.name == context.param_name {
                            context.has_direct = true;
                        }
                    }
                }
            }
            pt::Expression::AssignAdd(_, lhs, rhs)
            | pt::Expression::AssignSubtract(_, lhs, rhs)
            | pt::Expression::AssignMultiply(_, lhs, rhs)
            | pt::Expression::AssignDivide(_, lhs, rhs)
            | pt::Expression::AssignModulo(_, lhs, rhs) => {
                let lhs_var = Self::get_base_var_name(lhs);
                if context.state_vars.contains(&lhs_var) && context.influenced_vars.contains(&lhs_var)
                    && Self::expression_contains_var(rhs, &context.param_name) {
                        context.has_arithmetic = true;
                    }
            }
            pt::Expression::Add(_, l, r)
            | pt::Expression::Subtract(_, l, r)
            | pt::Expression::Multiply(_, l, r)
            | pt::Expression::Divide(_, l, r)
            | pt::Expression::Modulo(_, l, r)
            | pt::Expression::Power(_, l, r) => {
                if in_assignment && Self::expression_contains_var(expr, &context.param_name) {
                    context.has_arithmetic = true;
                }
                Self::analyze_influence_in_expression(l, context, in_assignment);
                Self::analyze_influence_in_expression(r, context, in_assignment);
            }
            pt::Expression::ArraySubscript(_, base, index) => {
                // Check if parameter is used as array/mapping index
                if let Some(idx) = index {
                    if Self::expression_contains_var(idx, &context.param_name) {
                        let base_var = Self::get_base_var_name(base);
                        if context.state_vars.contains(&base_var) {
                            context.has_index_access = true;
                        }
                    }
                }
                Self::analyze_influence_in_expression(base, context, in_assignment);
                if let Some(idx) = index {
                    Self::analyze_influence_in_expression(idx, context, in_assignment);
                }
            }
            pt::Expression::MemberAccess(_, base, _) => {
                Self::analyze_influence_in_expression(base, context, in_assignment);
            }
            pt::Expression::FunctionCall(_, func, args) => {
                Self::analyze_influence_in_expression(func, context, in_assignment);
                for arg in args {
                    Self::analyze_influence_in_expression(arg, context, in_assignment);
                }
            }
            _ => {}
        }
    }

    /// Check if expression contains a specific variable
    fn expression_contains_var(expr: &pt::Expression, var_name: &str) -> bool {
        match expr {
            pt::Expression::Variable(ident) => ident.name == var_name,
            pt::Expression::MemberAccess(_, base, _) => Self::expression_contains_var(base, var_name),
            pt::Expression::ArraySubscript(_, base, index) => {
                Self::expression_contains_var(base, var_name)
                    || index.as_ref().is_some_and(|idx| Self::expression_contains_var(idx, var_name))
            }
            pt::Expression::FunctionCall(_, func, args) => {
                Self::expression_contains_var(func, var_name)
                    || args.iter().any(|arg| Self::expression_contains_var(arg, var_name))
            }
            pt::Expression::Assign(_, l, r)
            | pt::Expression::Add(_, l, r)
            | pt::Expression::Subtract(_, l, r)
            | pt::Expression::Multiply(_, l, r)
            | pt::Expression::Divide(_, l, r)
            | pt::Expression::Equal(_, l, r)
            | pt::Expression::NotEqual(_, l, r)
            | pt::Expression::Less(_, l, r)
            | pt::Expression::More(_, l, r)
            | pt::Expression::And(_, l, r)
            | pt::Expression::Or(_, l, r) => {
                Self::expression_contains_var(l, var_name) || Self::expression_contains_var(r, var_name)
            }
            pt::Expression::Not(_, e)
            | pt::Expression::Negate(_, e)
            | pt::Expression::Parenthesis(_, e) => Self::expression_contains_var(e, var_name),
            _ => false,
        }
    }

    /// Check if statement modifies any influenced state variables
    fn statement_modifies_influenced_vars(stmt: &pt::Statement, context: &InfluenceContext) -> bool {
        match stmt {
            pt::Statement::Block { statements, .. } => {
                statements.iter().any(|s| Self::statement_modifies_influenced_vars(s, context))
            }
            pt::Statement::Expression(_, expr) => Self::expression_modifies_influenced_vars(expr, context),
            _ => false,
        }
    }

    /// Check if expression modifies any influenced state variables
    fn expression_modifies_influenced_vars(expr: &pt::Expression, context: &InfluenceContext) -> bool {
        match expr {
            pt::Expression::Assign(_, lhs, _)
            | pt::Expression::AssignAdd(_, lhs, _)
            | pt::Expression::AssignSubtract(_, lhs, _)
            | pt::Expression::AssignMultiply(_, lhs, _)
            | pt::Expression::AssignDivide(_, lhs, _) => {
                let lhs_var = Self::get_base_var_name(lhs);
                context.influenced_vars.contains(&lhs_var)
            }
            _ => false,
        }
    }
}
