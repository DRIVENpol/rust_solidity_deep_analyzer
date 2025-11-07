use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub members: Vec<StructMember>,
    pub line_number: usize,
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
    pub calls_functions: Vec<String>, // Other functions this function calls
    pub external_calls: Vec<ExternalCall>, // External contract calls this function makes
    pub storage_params: Vec<StorageParamInfo>, // Storage reference parameters
    pub uses_modifiers: Vec<String>,  // Modifiers applied to this function
    pub emits_events: Vec<String>,    // Events emitted by this function
    pub uses_errors: Vec<String>,     // Custom errors thrown by this function
    pub has_unchecked: bool,          // Whether function contains unchecked blocks
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
}

// Type of call in a call chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CallType {
    Internal,             // Internal function call within same contract
    External,             // External call to another contract (resolved)
    ExternalUnresolved,   // External call but target contract not in codebase
}

// Additional info for external calls in call chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCallInfo {
    pub target_variable: String,
    pub target_type: String,
    pub target_contract: Option<String>,
    pub state_mutability: String,
}

// Represents a single step in a call chain with its modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallChainStep {
    pub function_name: String,
    pub call_type: CallType,
    pub modified_variables: Vec<String>,
    pub other_modifiers: Vec<String>,  // Other functions that modify the same variables
    pub target_info: Option<ExternalCallInfo>, // For external calls
}

// Represents the full relationship between contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRelation {
    pub external_call: ExternalCall,
    pub modified_variables: Vec<String>,     // State vars modified in target function
    pub other_modifiers: Vec<String>,        // Other functions that modify same vars
    pub full_call_chain: Vec<CallChainStep>, // Complete chain with modifications
}
