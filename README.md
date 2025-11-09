# MainnetReady - Solidity Enhanced Analyzer

**Created by Paul Socarde**

A comprehensive Rust-based static analysis tool for Solidity smart contracts. Analyzes state variables, tracks modifications across function call chains, and maps cross-contract relationships.

## Features

### Core Analysis Capabilities

- **State Variable Tracking**: Identifies all state variables with their types, visibility, and modification chains
- **Inter-Procedural Analysis**: Tracks state modifications through storage reference parameters across function calls
- **Upgradeable Contract Support (ERC-7201)**: Detects and analyzes upgradeable storage patterns with namespaced storage slots
  - Identifies storage structs with `@custom:storage-location` annotations
  - Tracks modifications through storage accessor functions
  - Treats storage struct fields as virtual state variables in reports
- **Function Analysis**: Extracts function signatures, parameters, return types, visibility, and state mutability
- **Cross-Contract Relationships**: Maps external contract calls and tracks state modifications across contracts
- **Multiple Entry Point Detection**: Identifies state variables modifiable through multiple cross-contract paths
- **Interface Resolution**: Automatically resolves interface types (e.g., `IToken` → `Token`) to their implementations
- **Recursive Call Chain Analysis**: Follows function calls recursively to track all state modifications
- **Event & Error Tracking**: Detects custom events and errors with their usage locations
  - Tracks inherited/imported errors from parent contracts and interfaces
  - Distinguishes between locally defined and inherited errors
- **Modifier Detection**: Identifies modifiers and their usage across functions
- **Flexible Exclusions**: `.analyzerignore` file with wildcard pattern support

### Advanced Security Analysis 🔒

- **Taint Analysis**: Tracks untrusted input flows from sources to dangerous sinks
  - **Sources**: User inputs (msg.sender, msg.value, msg.data), function parameters, external call returns
  - **Sinks**: selfdestruct, delegatecall, state modifications, value transfers, array indexing
  - **Validation Detection**: Identifies require/assert checks on tainted data
- **Data Flow Analysis**: Maps how data flows through variables and function calls
- **Severity Ratings**: Categorizes findings as Critical, High, Medium, Low, or Info
- **Ignored Return Value Detection**: Flags unchecked external calls (e.g., token transfers)
- **Parameter Influence Tracking**: Shows how function parameters affect state variables

### Comprehensive Reporting & Visualization 📊

- **Multiple Report Types**:
  - Individual contract analysis (detailed markdown per contract)
  - Contract interaction mapping (who calls whom)
  - Function call graphs (internal and external call hierarchies)
  - State variable access reports (read/write patterns)
  - Cross-contract state dependencies (state access across boundaries)
- **Multiple Output Formats**:
  - **Markdown**: Human-readable reports with tables and summaries
  - **JSON**: Machine-readable data for CI/CD and custom tools
  - **DOT**: Graphviz graph files for visual diagrams
  - **Console**: Detailed or table-based terminal output
- **Graph Visualization**: Generate visual diagrams with Graphviz
  - Contract interaction graphs
  - Function call hierarchies
  - State variable dependency maps
  - Cross-contract relationship diagrams

## Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Comes with Rust installation

To install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Installation

### 1. Download the Project

**Option A: Clone the repository**
```bash
git clone <repository-url>
cd rust_state_variables
```

**Option B: Download ZIP**
- Download and extract the project
- Navigate to the project directory

### 2. Install Dependencies

Dependencies are automatically downloaded when you build:
```bash
cargo build --release
```

This installs:
- `solang-parser`: Solidity AST parser
- `clap`: Command-line argument parsing
- `walkdir`: Directory traversal
- `serde` / `serde_json`: Serialization
- `anyhow`: Error handling
- `prettytable-rs`: Table formatting
- `colored`: Terminal colors

### 3. Verify Installation

```bash
cargo run -- --help
```

You should see the help message with available commands.

## Quick Start

1. **Place your Solidity contracts** in the `./contracts` directory:
   ```
   ./contracts/
   ├── Token.sol
   ├── Staking.sol
   └── interfaces/
       └── IToken.sol
   ```

2. **Run the analyzer**:
   ```bash
   cargo run -- analyze
   ```

3. **View the results**:
   - Console shows detailed analysis with security findings
   - `./reports/` contains individual contract markdown files
   - `./reports/0_relations/` contains multiple analysis reports:
     - `contract_interactions.md` - Contract call mappings
     - `function_calls.md` - Function call graphs
     - `state_variables.md` - State variable access patterns
     - `cross_contract_state_dependencies.md` - Cross-contract dependencies
   - JSON and DOT graph files for each report type

## Project Setup

### Directory Structure

```
solidity-analyzer/
├── .analyzerignore          # Exclusion patterns (optional)
├── contracts/               # Your Solidity contracts
│   ├── Token.sol
│   ├── Staking.sol
│   └── interfaces/
│       └── IToken.sol
├── reports/                 # Generated reports (auto-created)
│   ├── 0_relations/
│   │   ├── contract_interactions.md / .json / .dot
│   │   ├── function_calls.md / .json / .dot
│   │   ├── state_variables.md / .json / .dot
│   │   └── cross_contract_state_dependencies.md / .json / .dot
│   ├── Token.md
│   └── Staking.md
└── src/                     # Analyzer source code
```

### Organizing Contracts

- Place all `.sol` files in `./contracts` or subdirectories
- The analyzer recursively scans all subdirectories
- Use `.analyzerignore` to exclude specific files/patterns

## Configuration: .analyzerignore

Create a `.analyzerignore` file in the project root to exclude files from analysis.

### Supported Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| Exact name | Specific file | `IERC20.sol` |
| Prefix wildcard | Files starting with | `Test*.sol` |
| Suffix wildcard | Files ending with | `*Mock.sol` |
| Directory | All files in directory | `lib/*`, `node_modules/*` |
| Interface prefix | All interfaces | `I*.sol` |
| Comments | Ignored lines | `# This is a comment` |

### Example .analyzerignore

```bash
# Exclude test files
*Test.sol
*.t.sol

# Exclude mocks
*Mock.sol

# Exclude external libraries
lib/*
node_modules/*

# Exclude specific interfaces
IERC20.sol
IERC721.sol

# Exclude all interfaces starting with I
I*.sol
```

### Creating .analyzerignore

```bash
# Create the file
touch .analyzerignore

# Add patterns
echo "*Test.sol" >> .analyzerignore
echo "*Mock.sol" >> .analyzerignore
```

## Usage & Commands

### Default Command (Recommended)

```bash
cargo run -- analyze
```

**This automatically:**
- Uses detailed output format
- Saves markdown reports for each contract
- Generates cross-contract relations
- Outputs to `./reports/`

### Command Reference

#### Basic Analysis
```bash
# Analyze contracts in ./contracts (default)
cargo run -- analyze

# Analyze contracts in custom directory
cargo run -- analyze --path ./my-contracts
```

#### Output Formats

```bash
# Table format (quick overview)
cargo run -- analyze --format table

# Detailed format (full analysis)
cargo run -- analyze --format detailed

# JSON format (machine-readable)
cargo run -- analyze --format json
```

#### Disable Features

```bash
# Skip markdown generation
cargo run -- analyze --save-md false

# Skip relations analysis
cargo run -- analyze --relations false

# Skip both (console output only)
cargo run -- analyze --save-md false --relations false
```

#### Custom Output Paths

```bash
# Custom markdown output directory
cargo run -- analyze --md-output ./my-reports

# Custom relations output file
cargo run -- analyze --relations-output ./analysis/relations.md

# Both custom
cargo run -- analyze --md-output ./reports --relations-output ./reports/relations.md
```

#### Export to JSON

```bash
# Export analysis to JSON file
cargo run -- analyze --export ./output.json

# Combine with other options
cargo run -- analyze --format table --export ./data.json
```

### All Available Flags

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--path` | `-p` | `./contracts` | Path to contracts directory |
| `--format` | `-f` | `detailed` | Output format: `table`, `detailed`, or `json` |
| `--export` | `-e` | None | Export to JSON file |
| `--save-md` | | `true` | Save markdown reports |
| `--md-output` | | `./reports` | Markdown output directory |
| `--relations` | | `true` | Generate relations report |
| `--relations-output` | | `./reports/0_relations/relations.md` | Relations output file |

## Output Files

### Individual Contract Reports

Located in `./reports/ContractName.md`

**Contains:**
- State variables with types and modification chains
- Upgradeable storage detection (ERC-7201 pattern with namespace and slot info)
- Functions with parameters, returns, and modifiers
- Events with parameters and emission locations
- Custom errors with usage (including inherited errors marked with "(inherited)")
- Structs and enums
- **NEW**: Security findings from taint analysis
- **NEW**: Ignored return value warnings
- **NEW**: Data flow analysis results

**Example:**
```markdown
CONTRACT: Token

STATE VARIABLES & MODIFICATIONS:
  • totalSupply
    Type: uint256, public
    Modified by:
      ├─ mint (external)
      └─ burn (external)

FUNCTIONS:
  • transfer(address to, uint256 amount) → bool
    Visibility: public
    Modifies: balances

🔒 SECURITY FINDINGS:

TAINT FLOWS (2 finding(s)):
  • HIGH: Untrusted parameter 'amount' flows to state modification 'balances'
    Validation: Checked with require
```

### Analysis Reports in `./reports/0_relations/`

#### 1. Contract Interactions Report

Files: `contract_interactions.md`, `.json`, `.dot`

**Contains:**
- Which contracts call which other contracts
- Function-level call mappings
- State mutability information
- Call frequencies and patterns

**Example:**
```markdown
## Contract Interaction: Staking → Token

Functions Called:
  • mint(address, uint256) - Called by: claimReward, distributeRewards
  • transfer(address, uint256) - Called by: emergencyWithdraw
```

#### 2. Function Call Graphs

Files: `function_calls.md`, `.json`, `.dot`

**Contains:**
- Internal function call hierarchies
- External contract calls
- Complete call chains
- Recursion detection

**Example:**
```markdown
Function: transfer
├─ _beforeTokenTransfer (internal)
├─ _transfer (internal)
│  ├─ _burn (internal)
│  └─ _mint (internal)
└─ _afterTokenTransfer (internal)
```

#### 3. State Variable Access Reports

Files: `state_variables.md`, `.json`, `.dot`

**Contains:**
- Read/write patterns for each state variable
- Which functions access which variables
- Access chains (direct and indirect)
- Modification frequency

**Example:**
```markdown
State Variable: balances

Modifications:
  • transfer ← _transfer ← _mint
  • burn ← _transfer ← _burn

Reads:
  • balanceOf (public view)
  • transfer (checks balance)
```

#### 4. Cross-Contract State Dependencies

Files: `cross_contract_state_dependencies.md`, `.json`, `.dot`

**Contains:**
- How external contracts modify state variables
- Cross-boundary state access patterns
- Multiple entry point analysis
- Dependency graphs

**Example:**
```markdown
State Variable: totalSupply in Token

External Modifiers:
  ├─ Staking.claimReward() → Token.mint()
  ├─ Vesting.release() → Token.mint()
  └─ Treasury.distribute() → Token.mint()
```

## Feature Details

### State Variable Modification Tracking

Tracks how state variables are modified through:
- Direct modifications in functions
- Indirect modifications through internal function calls
- Storage reference parameters (e.g., `function modify(Type storage _var)`)
- Upgradeable storage patterns (ERC-7201) with storage struct accessors
- Recursive call chain analysis
- Modifier effects

**Inter-Procedural Analysis Example:**
```solidity
function processDeposit(address user) external {
    UserInfo storage info = userInfo[user];
    _consolidateRewards(info);  // Passes storage reference
}

function _consolidateRewards(UserInfo storage _info) internal {
    _info.rewards += calculateRewards();  // Modifies through parameter
}
```

The analyzer detects that `processDeposit` modifies `userInfo` even though the modification happens in `_consolidateRewards` through a storage parameter.

**Upgradeable Storage (ERC-7201) Example:**
```solidity
/// @custom:storage-location erc7201:openzeppelin.storage.ERC20
struct ERC20Storage {
    mapping(address => uint256) _balances;
    mapping(address => mapping(address => uint256)) _allowances;
    uint256 _totalSupply;
}

function _getERC20Storage() private pure returns (ERC20Storage storage $) {
    bytes32 slot = ERC20StorageLocation;
    assembly { $.slot := slot }
}

function transfer(address to, uint256 value) public {
    ERC20Storage storage $ = _getERC20Storage();
    $._balances[msg.sender] -= value;  // Tracked as modification of _balances
}
```

The analyzer detects upgradeable storage patterns and treats struct fields (`_balances`, `_allowances`, `_totalSupply`) as virtual state variables with full modification tracking.

### Cross-Contract Relationship Analysis

Detects and maps:
- External contract calls (`contract.function()`)
- Interface-based calls with automatic resolution
- State modifications in called functions
- Complete call chains across contracts

### Multiple Entry Point Detection

Identifies state variables that can be modified through multiple cross-contract call paths. This is crucial for:
- **Access control analysis**: Understanding all ways a critical state variable can be changed
- **Attack surface mapping**: Identifying variables with broad modification surfaces
- **Security auditing**: Spotting patterns where one state variable has many entry points

**Example:**
```markdown
`lpPoolCap` in contract `JackpotLPManager`
   6 entry point(s):
      ├─ `Jackpot.initializeLPDeposits()` → `setLPPoolCap()`
      ├─ `Jackpot.setNormalBallMax()` → `setLPPoolCap()`
      ├─ `Jackpot.setGovernancePoolCap()` → `setLPPoolCap()`
      └─ ... (3 more)
```

This shows that while `lpPoolCap` is only modified by one function (`setLPPoolCap`), it can be triggered from 6 different external entry points.

### Interface Resolution

Automatically resolves interface types to implementations:
- `IToken` → `Token`
- `IERC20` → `ERC20`
- Follows standard `I` prefix naming convention
- Falls back to direct type matching if no `I` prefix

### Recursive Call Chain Analysis

Follows function calls deeply:
```
transfer() → _processTransfer() → _transfer() → modifies balances
```

Tracks state modifications at each level.

### Upgradeable Contract Analysis (ERC-7201)

The analyzer fully supports OpenZeppelin's ERC-7201 namespaced storage pattern for upgradeable contracts:

**Detection:**
- Identifies `@custom:storage-location` NatSpec annotations on storage structs
- Detects storage accessor functions that return storage struct references
- Extracts namespace, storage slot (bytes32), and struct fields

**Analysis:**
- Tracks modifications through storage struct references (e.g., `$._balances[user] = amount`)
- Creates virtual state variables from struct fields
- Shows full modification chains for upgradeable storage fields
- Marks upgradeable storage fields with 🔄 emoji in reports

**Example Report Output:**
```
🔄 UPGRADEABLE STORAGE DETECTED (ERC-7201)
   Namespace: openzeppelin.storage.ERC20
   Storage Struct: ERC20Storage
   Storage Slot: 0x52c63247e1f47db19d5ce0460030c497f067ca4cebf71ba98eeadabe20bace00
   Accessor Function: _getERC20Storage

STATE VARIABLES:
_balances
   Type: mapping(address => uint256) (upgradeable storage)
   Modified by:
      └─ _update (internal) ← _transfer (internal) ← transfer (public)
```

### Inherited Error Tracking

The analyzer tracks both locally defined and inherited/imported custom errors:

**Detection:**
- Scans all `revert` statements, not just locally defined errors
- Tracks errors imported from parent contracts and interfaces
- Identifies errors used but not defined in the contract

**Reporting:**
- Shows all errors with their usage locations
- Marks inherited errors with "(inherited)" indicator
- Preserves full qualified names (e.g., `JackpotErrors.ZeroAddress`)

**Example:**
```
CUSTOM ERRORS:

BridgeFundsFailed
   Used in:
      └─ _bridgeFunds

JackpotErrors.ZeroAddress (inherited)
   Used in:
      ├─ claimTickets
      └─ buyTickets
```

## Visualization with Graphviz

All analysis reports include `.dot` files that can be visualized using Graphviz.

### Install Graphviz

```bash
# macOS
brew install graphviz

# Ubuntu/Debian
sudo apt-get install graphviz

# Windows (using Chocolatey)
choco install graphviz
```

### Generate Visual Diagrams

```bash
# Run analysis to generate .dot files
cargo run -- analyze

# Convert DOT to PNG
dot -Tpng reports/0_relations/contract_interactions.dot -o interactions.png
dot -Tpng reports/0_relations/function_calls.dot -o call_graph.png
dot -Tpng reports/0_relations/state_variables.dot -o state_vars.png
dot -Tpng reports/0_relations/cross_contract_state_dependencies.dot -o dependencies.png

# Convert to SVG (scalable)
dot -Tsvg reports/0_relations/contract_interactions.dot -o interactions.svg

# Interactive viewing
xdot reports/0_relations/contract_interactions.dot
```

### Visualization Examples

**Contract Interactions Graph:**
- Nodes: Contracts
- Edges: Function calls
- Colors: Different contract types
- Labels: Called functions

**Function Call Graph:**
- Nodes: Functions
- Edges: Calls
- Styles: Internal (solid), External (dashed)
- Colors: By visibility (public, private, internal, external)

**State Variable Dependencies:**
- Nodes: State variables
- Edges: Modifications and reads
- Colors: By access type (read/write)
- Labels: Accessing functions

## Security Analysis Features

### Taint Analysis

The analyzer tracks data flow from untrusted sources to dangerous operations:

**Taint Sources (Untrusted):**
- `msg.sender` - Caller address
- `msg.value` - Ether amount
- `msg.data` - Call data
- Function parameters (external/public functions)
- External contract call return values
- Array accesses with tainted indices

**Taint Sinks (Dangerous Operations):**
- `selfdestruct` - Contract destruction
- `delegatecall` - Delegated execution
- State variable modifications
- Value transfers (`transfer`, `send`, `call{value:}`)
- Array index operations

**Validation Detection:**
- Identifies `require()` and `assert()` checks
- Tracks validated variables
- Reduces false positives for validated inputs

**Severity Levels:**
- **Critical**: Tainted data reaches `selfdestruct` or `delegatecall` without validation
- **High**: Unvalidated external inputs modify state or control flow
- **Medium**: Tainted data in array indices or external calls
- **Low**: Validated tainted data (informational)
- **Info**: General data flow tracking

**Example Output:**
```markdown
🔒 TAINT FLOWS (3 finding(s)):

• CRITICAL: msg.sender flows to delegatecall target
  Function: executeDelegate
  Path: msg.sender → target → delegatecall
  Validation: NONE

• HIGH: Parameter 'amount' flows to state modification
  Function: withdraw
  Path: amount → balances[msg.sender] -= amount
  Validation: Checked with require(amount <= balances[msg.sender])
  Severity reduced to LOW due to validation

• MEDIUM: Tainted array index access
  Function: getUser
  Path: msg.sender → users[msg.sender]
  Validation: No bounds check
```

### Ignored Return Values

Detects unchecked external call returns that could lead to vulnerabilities:

**Detection:**
- Identifies calls where return values are ignored
- Flags high-risk functions (transfer, transferFrom, approve, send)
- Categorizes by severity

**Example:**
```markdown
⚠️ IGNORED RETURN VALUES (2 finding(s)):

• HIGH: token.transfer() return value ignored
  Function: distribute
  Risk: Silent failure could lead to accounting errors

• MEDIUM: externalContract.call() return value ignored
  Function: executeCall
  Risk: Failed call not handled
```

### JSON Export for Automation

All analysis data is available in JSON format for CI/CD integration:

```bash
# Generate JSON exports
cargo run -- analyze

# Query with jq
cat reports/0_relations/contract_interactions.json | jq '.interactions[] | select(.target_contract == "Token")'

# Check for critical findings
cat reports/TokenContract.md # Contains JSON-exportable data

# Custom security checks
python scripts/check_security.py reports/0_relations/*.json
```

**Use Cases:**
- Automated security checks in CI/CD
- Custom analysis scripts
- Integration with other tools
- Trend analysis over time
- Diff analysis between versions

## Limitations

### Scope Limitations

1. **Codebase-Only Analysis**: Only analyzes contracts present in the specified directory. Cannot analyze:
   - External library contracts not in the codebase
   - Contracts from npm packages (unless copied to contracts folder)
   - Blockchain-deployed contracts

2. **Interface Resolution**:
   - Works best with `I` prefix naming convention (e.g., `IToken` → `Token`)
   - May not resolve interfaces with non-standard naming
   - Requires implementation contract to be in the codebase

3. **External Calls**:
   - Only tracks calls where both source and target are in the analyzed codebase
   - Calls to unknown contracts are listed but not analyzed in depth

### Analysis Limitations

1. **Security Analysis Scope**: While this tool provides taint analysis and data flow tracking, it does NOT:
   - Guarantee detection of all vulnerabilities
   - Detect complex reentrancy patterns
   - Perform complete security audits
   - Validate business logic correctness
   - Check for all best practices
   - **Note**: The taint analysis and ignored return value detection provide valuable security insights, but should complement (not replace) professional security audits

2. **No Runtime Analysis**:
   - Cannot track dynamic behavior
   - Does not simulate execution
   - Cannot determine actual values
   - Shows all **potential** modification paths - functions may only modify fields conditionally based on runtime values

3. **Limited Assembly Support**:
   - Does not analyze inline assembly blocks in detail
   - Assembly state modifications may not be fully detected
   - ERC-7201 storage slot assembly assignments are detected at a high level

4. **Solidity Version**:
   - Optimized for Solidity 0.8.x
   - May have issues with older versions
   - Based on `solang-parser 0.3`

### Technical Limitations

- Does not track cross-chain calls or bridge interactions
- Cannot resolve dynamic contract addresses
- Does not follow delegatecall chains
- Limited support for proxy patterns

## Examples

### Example 1: Basic Project Analysis

```bash
# Place contracts
mkdir -p contracts
cp MyToken.sol contracts/
cp MyStaking.sol contracts/

# Run analyzer
cargo run -- analyze

# View results
cat reports/MyToken.md
cat reports/0_relations/relations.md
```

### Example 2: Excluding Test Files

```bash
# Create .analyzerignore
cat > .analyzerignore << EOF
*Test.sol
*Mock.sol
mocks/*
EOF

# Run analysis (test files excluded)
cargo run -- analyze
```

### Example 3: Quick Table Overview

```bash
# See high-level stats without detailed output
cargo run -- analyze --format table --save-md false --relations false
```

### Example 4: Export for CI/CD

```bash
# Generate JSON for automated processing
cargo run -- analyze --format json --export ./analysis.json

# Use in scripts
cat analysis.json | jq '.[] | select(.name == "Token")'
```

### Example 5: Analyzing Complex Projects

```bash
# Analyze large project with custom exclusions
echo "lib/*" >> .analyzerignore
echo "test/*" >> .analyzerignore
echo "*Mock.sol" >> .analyzerignore

# Run with custom output
cargo run -- analyze \
  --path ./src/contracts \
  --md-output ./docs/analysis \
  --relations-output ./docs/relations.md
```

### Example 6: Generate Visual Diagrams

```bash
# Run analysis
cargo run -- analyze

# Generate PNG diagrams from all reports
dot -Tpng reports/0_relations/contract_interactions.dot -o contract_map.png
dot -Tpng reports/0_relations/function_calls.dot -o call_hierarchy.png
dot -Tpng reports/0_relations/state_variables.dot -o state_access.png

# View diagrams
open contract_map.png call_hierarchy.png state_access.png
```

### Example 7: Security-Focused Analysis

```bash
# Run full analysis
cargo run -- analyze

# Check for critical security findings
grep -r "CRITICAL:" reports/*.md
grep -r "HIGH:" reports/*.md

# Review taint analysis
grep -A 5 "TAINT FLOWS" reports/*.md

# Check ignored return values
grep -A 3 "IGNORED RETURN" reports/*.md

# Export to JSON for automated security checks
cat reports/0_relations/*.json | jq '.[] | select(.severity == "Critical" or .severity == "High")'
```

## Interpreting Relations Output

### Reading Call Chains

```markdown
`ContractA.functionX()`
   └─> `contractB.functionY()` [IContractB → ContractB]
          └─> modifies: `stateVar1`, `stateVar2`
          └─> also modified by: `otherFunc1`, `otherFunc2`
          `└─> _internalFunc` (internal)
```

**Interpretation:**
- `ContractA.functionX()` calls external contract
- Calls `functionY()` on `contractB` (type `IContractB`, resolves to `ContractB`)
- `functionY` modifies `stateVar1` and `stateVar2` (includes modifications via storage parameters)
- These same variables are also modified by `otherFunc1` and `otherFunc2`
- `functionY` calls internal `_internalFunc`

### Understanding Modifications

- **modifies**: State variables changed in this function (including through storage parameters)
- **also modified by**: Other functions that modify the same state variables
- Helps identify potential conflicts or dependencies

### Reading the Entry Points Summary

```markdown
`stateVar` in contract `ContractB`
   3 entry point(s):
      ├─ `ContractA.func1()` → `setter()`
      ├─ `ContractA.func2()` → `setter()`
      ├─ `ContractC.func3()` → `update()`
```

**Interpretation:**
- `stateVar` can be modified from 3 different cross-contract entry points
- Two entry points (`func1`, `func2`) both call the same modifier function (`setter`)
- One entry point (`func3`) calls a different modifier function (`update`)
- This helps identify state variables with broad modification surfaces

## Troubleshooting

### Build Errors

**Error: `cargo: command not found`**
```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Error: `failed to compile solidity-analyzer`**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### Analysis Issues

**No contracts found**
```bash
# Check your path
ls ./contracts

# Use custom path
cargo run -- analyze --path /path/to/contracts
```

**Contracts ignored unexpectedly**
```bash
# Check .analyzerignore patterns
cat .analyzerignore

# Test without ignore file
mv .analyzerignore .analyzerignore.bak
cargo run -- analyze
```

**Missing cross-contract relationships**
- Ensure both source and target contracts are in the analyzed directory
- Check that interface naming follows `I` prefix convention
- Verify both contracts are not excluded in `.analyzerignore`

### Performance

**Slow analysis on large projects**
```bash
# Use table format for faster overview
cargo run -- analyze --format table --save-md false

# Exclude unnecessary files
echo "test/*" >> .analyzerignore
echo "lib/*" >> .analyzerignore
```

## Contributing

MainnetReady - Solidity Enhanced Analyzer is created by Paul Socarde. This is an analysis tool for Solidity smart contracts. To contribute:
1. Test with various Solidity versions
2. Report bugs with sample contracts
3. Suggest feature improvements

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For issues or questions:
1. Check this README
2. Review the Troubleshooting section
3. Check `.analyzerignore` patterns
4. Verify contract syntax is valid Solidity

---

**Note**: This tool performs static analysis only. Always perform proper security audits and testing for production smart contracts.
