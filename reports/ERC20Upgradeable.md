════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `ERC20Upgradeable`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/ERC20Upgradable.sol`

🔄 **UPGRADEABLE STORAGE DETECTED (ERC-7201)**
   **Namespace:** `openzeppelin.storage.ERC20`
   **Storage Struct:** `ERC20Storage`
   **Storage Slot:** `0x52c63247e1f47db19d5ce0460030c497f067ca4cebf71ba98eeadabe20bace00`
   **Accessor Function:** `_getERC20Storage`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`ERC20StorageLocation`**
   **Type:** `bytes32`
   **Visibility:** private, constant

**`_balances`**
   **Type:** `mapping(address => uint256) (upgradeable storage)`
   **Visibility:** private

   **Modified by:**
      └─ `_update` *(internal)* ← `_transfer` *(internal)* ← `transfer` *(public)* ← `transferFrom` *(public)* ← `_mint` *(internal)* ← `_burn` *(internal)*


**`_allowances`**
   **Type:** `mapping(address => mapping(address => uint256)) (upgradeable storage)`
   **Visibility:** private

   **Modified by:**
      ├─ `_approve` *(internal)* ← `approve` *(public)* ← `_spendAllowance` *(internal)* ← `transferFrom` *(public)*
      └─ `_approve` *(internal)* ← `approve` *(public)* ← `_spendAllowance` *(internal)* ← `transferFrom` *(public)*


**`_totalSupply`**
   **Type:** `uint256 (upgradeable storage)`
   **Visibility:** private

   **Modified by:**
      └─ `_update` *(internal)* ← `_transfer` *(internal)* ← `transfer` *(public)* ← `transferFrom` *(public)* ← `_mint` *(internal)* ← `_burn` *(internal)*


**`_name`**
   **Type:** `string (upgradeable storage)`
   **Visibility:** private

   **Modified by:**
      └─ `__ERC20_init_unchained` *(internal)* ← `__ERC20_init` *(internal)*


**`_symbol`**
   **Type:** `string (upgradeable storage)`
   **Visibility:** private

   **Modified by:**
      └─ `__ERC20_init_unchained` *(internal)* ← `__ERC20_init` *(internal)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`ERC20InvalidReceiver`** *(inherited)*

   **Used in:**
      ├─ `_transfer`
      └─ `_mint`


**`ERC20InsufficientAllowance`** *(inherited)*

   **Used in:**
      └─ `_spendAllowance`


**`ERC20InvalidSpender`** *(inherited)*

   **Used in:**
      └─ `_approve`


**`ERC20InsufficientBalance`** *(inherited)*

   **Used in:**
      └─ `_update`


**`ERC20InvalidApprover`** *(inherited)*

   **Used in:**
      └─ `_approve`


**`ERC20InvalidSender`** *(inherited)*

   **Used in:**
      ├─ `_transfer`
      └─ `_burn`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`_getERC20Storage()`** → `ERC20Storage $`
   **Visibility:** private
   **State Mutability:** pure
   **Line:** 46


**`__ERC20_init(string name_, string symbol_)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 57

   **Modifiers:**
      └─ `onlyInitializing`


**`__ERC20_init_unchained(string name_, string symbol_)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 61

   **Modifiers:**
      └─ `onlyInitializing`


**`name()`** → `string`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 70


**`symbol()`** → `string`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 79


**`decimals()`** → `uint8`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 97


**`totalSupply()`** → `uint256`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 102


**`balanceOf(address account)`** → `uint256`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 108


**`transfer(address to, uint256 value)`** → `bool`
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 121


**`allowance(address owner, address spender)`** → `uint256`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 128


**`approve(address spender, uint256 value)`** → `bool`
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 143


**`transferFrom(address from, address to, uint256 value)`** → `bool`
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 165


**`_transfer(address from, address to, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 182


**`_update(address from, address to, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable *[unchecked]*
   **Line:** 199


**`_mint(address account, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 238


**`_burn(address account, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 253


**`_approve(address owner, address spender, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 275


**`_approve(address owner, address spender, uint256 value, bool emitEvent)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 297


**`_spendAllowance(address owner, address spender, uint256 value)`**
   **Visibility:** internal
   **State Mutability:** nonpayable *[unchecked]*
   **Line:** 319

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
