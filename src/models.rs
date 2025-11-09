use serde::{Deserialize, Serialize};
use crate::dataflow::DataFlowAnalysis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub file_path: String,
    pub state_variables: Vec<StateVariable>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
    pub events: Vec<EventDef>,
    pub functions: Vec<FunctionDef>,
    pub modifiers: Vec<ModifierDef>,
    pub errors: Vec<ErrorDef>,
    pub upgradeable_storage: Option<UpgradeableStorage>, // ERC-7201 pattern info
    pub dataflow_analysis: Option<DataFlowAnalysis>, // Data flow and taint analysis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVariable {
    pub name: String,
    pub var_type: String,
    pub visibility: String,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub line_number: usize,
    pub modification_chains: Vec<ModificationChain>,
    pub read_chains: Vec<ModificationChain>, // Functions that read this variable (reusing ModificationChain structure)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub members: Vec<StructMember>,
    pub line_number: usize,
    pub storage_location: Option<String>, // For upgradeable contracts: e.g., "erc7201:openzeppelin.storage.ERC20"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructMember {
    pub name: String,
    pub member_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub values: Vec<String>,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDef {
    pub name: String,
    pub parameters: Vec<EventParam>,
    pub line_number: usize,
    pub emitted_in: Vec<String>, // Functions that emit this event
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParam {
    pub name: String,
    pub param_type: String,
    pub indexed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub visibility: String,
    pub state_mutability: String,
    pub parameters: Vec<String>,
    pub returns: Vec<String>,
    pub line_number: usize,
    pub modifies_states: Vec<String>, // State variables this function directly modifies
    pub modifies_state_fields: Vec<String>, // Granular field-level modifications (e.g., "lpInfo.consolidatedShares")
    pub reads_states: Vec<String>, // State variables this function reads from (non-modifying access)
    pub calls_functions: Vec<String>, // Other functions this function calls
    pub external_calls: Vec<ExternalCall>, // External contract calls this function makes
    pub storage_params: Vec<StorageParamInfo>, // Storage reference parameters
    pub uses_modifiers: Vec<String>,  // Modifiers applied to this function
    pub modifier_order: Vec<String>,  // Modifiers in execution order
    pub emits_events: Vec<String>,    // Events emitted by this function
    pub uses_errors: Vec<String>,     // Custom errors thrown by this function
    pub has_unchecked: bool,          // Whether function contains unchecked blocks
    pub return_value_usage: Vec<ReturnValueUsage>, // How return values from calls are used
    pub ignored_returns: Vec<IgnoredReturn>, // Function calls whose return values are ignored
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageParamInfo {
    pub param_index: usize,    // Position in parameter list (0-based)
    pub param_name: String,    // Parameter name (e.g., "_lp")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierDef {
    pub name: String,
    pub parameters: Vec<String>,
    pub line_number: usize,
    pub used_in: Vec<String>, // Functions that use this modifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDef {
    pub name: String,
    pub parameters: Vec<ErrorParam>,
    pub line_number: usize,
    pub used_in: Vec<String>, // Functions that throw this error
    pub is_inherited: bool,    // True if error is used but not defined locally
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorParam {
    pub name: String,
    pub param_type: String,
}

// Represents a modification chain: StateVar -> DirectFunc -> CallerFunc -> ExternalFunc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationChain {
    pub direct_modifier: String,        // Function that directly modifies the state
    pub direct_modifier_visibility: String,
    pub call_chain: Vec<FunctionCall>,  // Chain of callers leading to this modification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub function_name: String,
    pub visibility: String,
}

// Represents an external call from one contract to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCall {
    pub source_contract: String,
    pub source_function: String,
    pub target_variable: String,      // Variable name (e.g., "stakingToken")
    pub target_type: String,          // Type (e.g., "IERC20")
    pub target_function: String,      // Function called (e.g., "transferFrom")
    pub target_contract: Option<String>, // Matched contract name if found
    pub state_mutability: String,     // "view", "pure", "nonpayable", "payable", or "unknown"
    pub line_number: usize,
    pub target_modifies_states: Vec<String>, // State variables modified in target function
    pub target_reads_states: Vec<String>,    // State variables read in target function
}

// Represents an upgradeable storage pattern (ERC-7201)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeableStorage {
    pub namespace: String,                    // e.g., "openzeppelin.storage.ERC20"
    pub storage_struct: String,               // Name of the storage struct (e.g., "ERC20Storage")
    pub storage_location_constant: String,    // Name of the constant (e.g., "ERC20StorageLocation")
    pub storage_slot: String,                 // The actual slot value (bytes32 hex)
    pub accessor_function: String,            // Name of the getter function (e.g., "_getERC20Storage")
    pub struct_fields: Vec<StructMember>,     // Fields in the storage struct
    pub line_number: usize,                   // Line where the struct is defined
}

// Represents how a return value from a function call is used
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnValueUsage {
    pub called_function: String,        // Function whose return value is used
    pub call_type: ReturnCallType,      // Internal or external
    pub usage_type: ReturnUsageType,    // How the return value is used
    pub assigned_to: Option<String>,    // Variable name if assigned
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnCallType {
    Internal,    // Call to function in same contract
    External,    // Call to external contract
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnUsageType {
    Assigned,          // Return value assigned to variable
    UsedInExpression,  // Used in arithmetic, comparison, etc.
    UsedInCondition,   // Used in if/require/while condition
    Returned,          // Directly returned from current function
    PassedAsArgument,  // Passed to another function
    Ignored,           // Return value not used (WARNING!)
}

// Represents a function call whose return value is ignored (security issue)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoredReturn {
    pub called_function: String,     // Function whose return is ignored
    pub call_type: ReturnCallType,   // Internal or external
    pub is_external_call: bool,      // True if external contract call
    pub target_contract: Option<String>, // For external calls
    pub severity: IgnoredReturnSeverity,
    pub line_number: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IgnoredReturnSeverity {
    Info,     // Low priority (e.g., internal view function)
    Low,      // Should check but not critical
    Medium,   // Important to check (external call)
    High,     // Critical - likely a bug (transfer, approve, etc.)
}

impl IgnoredReturnSeverity {
    pub fn emoji(&self) -> &'static str {
        match self {
            IgnoredReturnSeverity::Info => "ℹ️",
            IgnoredReturnSeverity::Low => "⚠️",
            IgnoredReturnSeverity::Medium => "🟡",
            IgnoredReturnSeverity::High => "🔴",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            IgnoredReturnSeverity::Info => "INFO",
            IgnoredReturnSeverity::Low => "LOW",
            IgnoredReturnSeverity::Medium => "MEDIUM",
            IgnoredReturnSeverity::High => "HIGH",
        }
    }
}

