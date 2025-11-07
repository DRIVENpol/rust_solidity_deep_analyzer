# MainnetReady - Solidity Enhanced Analyzer

**Created by Paul Socarde**

A comprehensive Rust-based static analysis tool for Solidity smart contracts. Analyzes state variables, tracks modifications across function call chains, and maps cross-contract relationships.

## Features

- **State Variable Tracking**: Identifies all state variables with their types, visibility, and modification chains
- **Inter-Procedural Analysis**: Tracks state modifications through storage reference parameters across function calls
- **Function Analysis**: Extracts function signatures, parameters, return types, visibility, and state mutability
- **Cross-Contract Relationships**: Maps external contract calls and tracks state modifications across contracts
- **Multiple Entry Point Detection**: Identifies state variables modifiable through multiple cross-contract paths
- **Interface Resolution**: Automatically resolves interface types (e.g., `IToken` → `Token`) to their implementations
- **Recursive Call Chain Analysis**: Follows function calls recursively to track all state modifications
- **Event & Error Tracking**: Detects custom events and errors with their usage locations
- **Modifier Detection**: Identifies modifiers and their usage across functions
- **Multiple Output Formats**:
  - Detailed console output
  - Individual markdown reports per contract
  - Cross-contract relations report with modification summary
  - JSON export
- **Flexible Exclusions**: `.analyzerignore` file with wildcard pattern support

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
   - Console shows detailed analysis
   - `./reports/` contains individual contract markdown files
   - `./reports/0_relations/relations.md` contains cross-contract relationships

## Project Setup

### Directory Structure

```
rust_state_variables/
├── .analyzerignore          # Exclusion patterns (optional)
├── contracts/               # Your Solidity contracts
│   ├── Token.sol
│   ├── Staking.sol
│   └── interfaces/
│       └── IToken.sol
├── reports/                 # Generated reports (auto-created)
│   ├── 0_relations/
│   │   └── relations.md
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
- Functions with parameters, returns, and modifiers
- Events with parameters and emission locations
- Custom errors with usage
- Structs and enums

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
```

### Relations Report

Located in `./reports/0_relations/relations.md`

**Contains:**
- All cross-contract external calls
- Full call chains with nested function calls
- State variables modified at each step
- Other functions that modify the same variables
- Summary of state variables with multiple entry points

**Example:**
```markdown
## Cross-Contract Call Chains

`Staking.claimReward()`
   └─> `rewardToken.mint()` [IToken → Token]
          └─> modifies: `totalSupply`, `balances`
          └─> also modified by: `_burn`, `_transfer`
          `└─> _mint` (internal)

## State Variables with Multiple Entry Points

`totalSupply` in contract `Token`
   3 entry point(s):
      ├─ `Staking.claimReward()` → `mint()`
      ├─ `Vesting.release()` → `mint()`
      ├─ `Treasury.distribute()` → `mint()`
```

## Feature Details

### State Variable Modification Tracking

Tracks how state variables are modified through:
- Direct modifications in functions
- Indirect modifications through internal function calls
- Storage reference parameters (e.g., `function modify(Type storage _var)`)
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

1. **No Security Analysis**: This tool does NOT:
   - Detect vulnerabilities (reentrancy, overflow, etc.)
   - Perform security audits
   - Check for best practices
   - Validate business logic

2. **No Runtime Analysis**:
   - Cannot track dynamic behavior
   - Does not simulate execution
   - Cannot determine actual values

3. **Limited Assembly Support**:
   - Does not analyze inline assembly blocks
   - Assembly state modifications may not be detected

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

[Add your license here]

## Support

For issues or questions:
1. Check this README
2. Review the Troubleshooting section
3. Check `.analyzerignore` patterns
4. Verify contract syntax is valid Solidity

---

**Note**: This tool performs static analysis only. Always perform proper security audits and testing for production smart contracts.
