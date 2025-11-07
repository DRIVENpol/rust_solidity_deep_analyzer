use anyhow::{Context, Result};
use solang_parser::{parse, pt};
use std::fs;
use std::path::Path;

use crate::models::*;
use crate::analyzer::StateModificationAnalyzer;

pub struct SolidityParser;

impl SolidityParser {
    /// Parse file and return both contracts and their ASTs for relationship analysis
    pub fn parse_file_with_ast(path: &Path) -> Result<(Vec<ContractInfo>, Vec<pt::ContractDefinition>)> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        let (ast, comments) = parse(&content, 0)
            .map_err(|e| anyhow::anyhow!("Parse error in {:?}: {:?}", path, e))?;

        let mut contracts = Vec::new();
        let mut contract_asts = Vec::new();

        for item in &ast.0 {
            if let pt::SourceUnitPart::ContractDefinition(contract) = item {
                let mut contract_info = Self::extract_contract_info(contract, path, &content, &comments)?;

                // Analyze state modifications and call chains
                StateModificationAnalyzer::analyze(&mut contract_info, contract);

                contracts.push(contract_info);
                contract_asts.push((**contract).clone());
            }
        }

        Ok((contracts, contract_asts))
    }

    fn extract_contract_info(
        contract: &pt::ContractDefinition,
        path: &Path,
        content: &str,
        comments: &[pt::Comment],
    ) -> Result<ContractInfo> {
        let mut info = ContractInfo {
            name: contract.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            file_path: path.to_string_lossy().to_string(),
            state_variables: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            events: Vec::new(),
            functions: Vec::new(),
            modifiers: Vec::new(),
            errors: Vec::new(),
            upgradeable_storage: None,
        };

        for part in &contract.parts {
            match part {
                pt::ContractPart::VariableDefinition(var) => {
                    info.state_variables.push(Self::extract_state_variable(var, content)?);
                }
                pt::ContractPart::StructDefinition(s) => {
                    // Extract storage location from comments before the struct
                    let storage_location = Self::extract_storage_location(&s.loc, comments, content);
                    info.structs.push(Self::extract_struct(s, content, storage_location)?);
                }
                pt::ContractPart::EnumDefinition(e) => {
                    info.enums.push(Self::extract_enum(e, content)?);
                }
                pt::ContractPart::EventDefinition(e) => {
                    info.events.push(Self::extract_event(e, content)?);
                }
                pt::ContractPart::ErrorDefinition(err) => {
                    info.errors.push(Self::extract_error(err, content)?);
                }
                pt::ContractPart::FunctionDefinition(f) => {
                    // Check if it's a modifier or a function
                    if matches!(f.ty, pt::FunctionTy::Modifier) {
                        info.modifiers.push(Self::extract_modifier(f, content)?);
                    } else {
                        info.functions.push(Self::extract_function(f, content)?);
                    }
                }
                _ => {}
            }
        }

        // Detect upgradeable storage pattern (ERC-7201)
        info.upgradeable_storage = Self::detect_upgradeable_storage(&info, contract, content);

        Ok(info)
    }

    fn extract_state_variable(var: &pt::VariableDefinition, content: &str) -> Result<StateVariable> {
        Ok(StateVariable {
            name: var.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            var_type: Self::type_to_string(&var.ty),
            visibility: Self::visibility_to_string(&var.attrs),
            is_constant: var.attrs.iter().any(|a| {
                matches!(a, pt::VariableAttribute::Constant(_))
            }),
            is_immutable: var.attrs.iter().any(|a| {
                matches!(a, pt::VariableAttribute::Immutable(_))
            }),
            line_number: Self::get_line_number(&var.loc, content),
            modification_chains: Vec::new(), // Will be filled by analyzer
        })
    }

    fn extract_struct(s: &pt::StructDefinition, content: &str, storage_location: Option<String>) -> Result<StructDef> {
        let members = s.fields.iter().map(|f| StructMember {
            name: f.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            member_type: Self::type_to_string(&f.ty),
        }).collect();

        Ok(StructDef {
            name: s.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            members,
            line_number: Self::get_line_number(&s.loc, content),
            storage_location,
        })
    }

    fn extract_enum(e: &pt::EnumDefinition, content: &str) -> Result<EnumDef> {
        let values = e.values.iter()
            .map(|v| v.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default())
            .collect();

        Ok(EnumDef {
            name: e.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            values,
            line_number: Self::get_line_number(&e.loc, content),
        })
    }

    fn extract_event(e: &pt::EventDefinition, content: &str) -> Result<EventDef> {
        let parameters = e.fields.iter().map(|p| EventParam {
            name: p.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            param_type: Self::type_to_string(&p.ty),
            indexed: p.indexed,
        }).collect();

        Ok(EventDef {
            name: e.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            parameters,
            line_number: Self::get_line_number(&e.loc, content),
            emitted_in: Vec::new(), // Will be filled by analyzer
        })
    }

    fn extract_error(err: &pt::ErrorDefinition, content: &str) -> Result<ErrorDef> {
        let parameters = err.fields.iter().map(|p| ErrorParam {
            name: p.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            param_type: Self::type_to_string(&p.ty),
        }).collect();

        Ok(ErrorDef {
            name: err.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            parameters,
            line_number: Self::get_line_number(&err.loc, content),
            used_in: Vec::new(), // Will be filled by analyzer
            is_inherited: false, // Locally defined errors are not inherited
        })
    }

    fn extract_function(f: &pt::FunctionDefinition, content: &str) -> Result<FunctionDef> {
        let params = f.params.iter()
            .map(|(_, p)| p.as_ref()
                .map(Self::param_to_string)
                .unwrap_or_default())
            .collect();

        let returns = f.returns.iter()
            .map(|(_, p)| p.as_ref()
                .map(Self::param_to_string)
                .unwrap_or_default())
            .collect();

        // Extract modifiers applied to this function
        let uses_modifiers = f.attributes.iter()
            .filter_map(|attr| {
                if let pt::FunctionAttribute::BaseOrModifier(_, base) = attr {
                    Some(base.name.identifiers.iter()
                        .map(|id| id.name.clone())
                        .collect::<Vec<_>>()
                        .join("."))
                } else {
                    None
                }
            })
            .collect();

        Ok(FunctionDef {
            name: f.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_else(|| "constructor".to_string()),
            visibility: Self::func_visibility_to_string(&f.attributes),
            state_mutability: Self::func_mutability_to_string(&f.attributes),
            parameters: params,
            returns,
            line_number: Self::get_line_number(&f.loc, content),
            modifies_states: Vec::new(),     // Will be filled by analyzer
            modifies_state_fields: Vec::new(), // Will be filled by analyzer
            calls_functions: Vec::new(),     // Will be filled by analyzer
            external_calls: Vec::new(),      // Will be filled by analyzer
            storage_params: Vec::new(),      // Will be filled by analyzer
            uses_modifiers,                  // Extracted here
            emits_events: Vec::new(),        // Will be filled by analyzer
            uses_errors: Vec::new(),         // Will be filled by analyzer
            has_unchecked: false,            // Will be filled by analyzer
        })
    }

    fn extract_modifier(m: &pt::FunctionDefinition, content: &str) -> Result<ModifierDef> {
        let params = m.params.iter()
            .map(|(_, p)| p.as_ref()
                .map(Self::param_to_string)
                .unwrap_or_default())
            .collect();

        Ok(ModifierDef {
            name: m.name.as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            parameters: params,
            line_number: Self::get_line_number(&m.loc, content),
            used_in: Vec::new(), // Will be filled by analyzing function modifiers
        })
    }

    // Helper function to get line number from Loc
    fn get_line_number(loc: &pt::Loc, content: &str) -> usize {
        match loc {
            pt::Loc::File(_, start, _) => {
                // Count newlines up to the start position
                content[..*start].chars().filter(|&c| c == '\n').count() + 1
            }
            _ => 0,
        }
    }

    // Enhanced type to string conversion
    fn type_to_string(ty: &pt::Expression) -> String {
        match ty {
            pt::Expression::Type(_, t) => Self::base_type_to_string(t),
            pt::Expression::Variable(ident) => ident.name.clone(),
            pt::Expression::ArraySubscript(_, base, None) => {
                format!("{}[]", Self::type_to_string(base))
            }
            pt::Expression::ArraySubscript(_, base, Some(len)) => {
                format!("{}[{}]", Self::type_to_string(base), Self::expr_to_string(len))
            }
            pt::Expression::MemberAccess(_, base, member) => {
                // Handle types like ContractName.StructName
                format!("{}.{}", Self::type_to_string(base), member.name)
            }
            _ => format!("{:?}", ty),
        }
    }

    fn base_type_to_string(ty: &pt::Type) -> String {
        match ty {
            pt::Type::Address => "address".to_string(),
            pt::Type::AddressPayable => "address payable".to_string(),
            pt::Type::Bool => "bool".to_string(),
            pt::Type::String => "string".to_string(),
            pt::Type::Bytes(n) => format!("bytes{}", n),
            pt::Type::Uint(n) => format!("uint{}", n),
            pt::Type::Int(n) => format!("int{}", n),
            pt::Type::Mapping { key, value, .. } => {
                format!("mapping({} => {})",
                    Self::type_to_string(key),
                    Self::type_to_string(value))
            }
            pt::Type::DynamicBytes => "bytes".to_string(),
            _ => format!("{:?}", ty),
        }
    }

    fn expr_to_string(expr: &pt::Expression) -> String {
        match expr {
            pt::Expression::NumberLiteral(_, val, _, _) => val.clone(),
            pt::Expression::Variable(ident) => ident.name.clone(),
            _ => "".to_string(),
        }
    }

    fn param_to_string(param: &pt::Parameter) -> String {
        let type_str = Self::type_to_string(&param.ty);
        let name = param.name.as_ref()
            .map(|n| n.name.clone())
            .unwrap_or_default();

        if name.is_empty() {
            type_str
        } else {
            format!("{} {}", type_str, name)
        }
    }

    fn visibility_to_string(attrs: &[pt::VariableAttribute]) -> String {
        for attr in attrs {
            if let pt::VariableAttribute::Visibility(v) = attr {
                return match v {
                    pt::Visibility::Public(_) => "public".to_string(),
                    pt::Visibility::Private(_) => "private".to_string(),
                    pt::Visibility::Internal(_) => "internal".to_string(),
                    pt::Visibility::External(_) => "external".to_string(),
                };
            }
        }
        "internal".to_string()
    }

    fn func_visibility_to_string(attrs: &[pt::FunctionAttribute]) -> String {
        for attr in attrs {
            if let pt::FunctionAttribute::Visibility(v) = attr {
                return match v {
                    pt::Visibility::Public(_) => "public".to_string(),
                    pt::Visibility::Private(_) => "private".to_string(),
                    pt::Visibility::Internal(_) => "internal".to_string(),
                    pt::Visibility::External(_) => "external".to_string(),
                };
            }
        }
        "public".to_string()
    }

    fn func_mutability_to_string(attrs: &[pt::FunctionAttribute]) -> String {
        for attr in attrs {
            if let pt::FunctionAttribute::Mutability(m) = attr {
                return match m {
                    pt::Mutability::Pure(_) => "pure".to_string(),
                    pt::Mutability::View(_) => "view".to_string(),
                    pt::Mutability::Payable(_) => "payable".to_string(),
                    pt::Mutability::Constant(_) => "view".to_string(), // Constant is an alias for view
                };
            }
        }
        "nonpayable".to_string()
    }

    /// Extract storage location from NatSpec comments
    /// Looks for: /// @custom:storage-location erc7201:namespace.name
    fn extract_storage_location(loc: &pt::Loc, comments: &[pt::Comment], _content: &str) -> Option<String> {
        // Get the position of the struct definition
        if let pt::Loc::File(_, start, _) = loc {
            // Look for comments immediately before this position
            for comment in comments {
                if let pt::Comment::DocLine(pt::Loc::File(_, _comment_start, comment_end), comment_text) = comment {
                    // Check if this comment is right before the struct (within ~200 chars)
                    if comment_end < start && start - comment_end < 200 {
                        // Look for @custom:storage-location pattern
                        if let Some(storage_loc) = Self::parse_storage_location_comment(comment_text) {
                            return Some(storage_loc);
                        }
                    }
                }
            }
        }
        None
    }

    /// Parse @custom:storage-location comment to extract namespace
    fn parse_storage_location_comment(comment: &str) -> Option<String> {
        // Pattern: @custom:storage-location erc7201:openzeppelin.storage.ERC20
        if let Some(pos) = comment.find("@custom:storage-location") {
            let after = &comment[pos + "@custom:storage-location".len()..];
            let trimmed = after.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    /// Detect ERC-7201 upgradeable storage pattern
    fn detect_upgradeable_storage(
        info: &ContractInfo,
        contract: &pt::ContractDefinition,
        _content: &str,
    ) -> Option<UpgradeableStorage> {
        use crate::models::UpgradeableStorage;

        // Look for a struct with storage-location annotation
        let storage_struct = info.structs.iter().find(|s| s.storage_location.is_some())?;

        // Extract namespace from storage location (e.g., "erc7201:openzeppelin.storage.ERC20")
        let storage_location = storage_struct.storage_location.as_ref()?;
        let namespace = if let Some(colon_pos) = storage_location.find(':') {
            storage_location[colon_pos + 1..].to_string()
        } else {
            storage_location.clone()
        };

        // Look for storage location constant (bytes32 constant XXXStorageLocation = ...)
        let mut storage_location_constant = String::new();
        let mut storage_slot = String::new();

        for part in &contract.parts {
            if let pt::ContractPart::VariableDefinition(var) = part {
                // Check if this is a bytes32 constant
                let is_bytes32 = matches!(
                    Self::type_to_string(&var.ty).as_str(),
                    "bytes32"
                );
                let is_constant = var.attrs.iter().any(|a| {
                    matches!(a, pt::VariableAttribute::Constant(_))
                });

                if is_bytes32 && is_constant {
                    if let Some(name) = &var.name {
                        let var_name = name.name.clone();
                        // Check if name matches pattern like "ERC20StorageLocation"
                        if var_name.contains("StorageLocation") || var_name.contains("Storage") {
                            storage_location_constant = var_name;
                            // Extract the slot value from the initializer
                            if let Some(init) = &var.initializer {
                                storage_slot = Self::extract_hex_value(init);
                            }
                            break;
                        }
                    }
                }
            }
        }

        // Look for accessor function (e.g., _getERC20Storage)
        let accessor_function = info.functions.iter()
            .find(|f| {
                f.name.contains("get") &&
                f.name.contains("Storage") &&
                f.visibility == "private"
            })
            .map(|f| f.name.clone())
            .unwrap_or_default();

        // Only return if we found the essential components
        if !storage_location_constant.is_empty() && !accessor_function.is_empty() {
            Some(UpgradeableStorage {
                namespace,
                storage_struct: storage_struct.name.clone(),
                storage_location_constant,
                storage_slot,
                accessor_function,
                struct_fields: storage_struct.members.clone(),
                line_number: storage_struct.line_number,
            })
        } else {
            None
        }
    }

    /// Extract hex value from expression (for storage slot constants)
    fn extract_hex_value(expr: &pt::Expression) -> String {
        match expr {
            pt::Expression::HexNumberLiteral(_, val, _) => val.clone(),
            pt::Expression::NumberLiteral(_, val, _, _) => val.clone(),
            _ => String::new(),
        }
    }
}
