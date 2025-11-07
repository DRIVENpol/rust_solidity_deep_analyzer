# Cross-Contract Call Analysis

**NOTE:** *Call chains show all potential modification paths identified through static analysis. Some functions may only modify fields conditionally based on runtime values (e.g., `if (from == address(0))`).*

## **Cross-Contract Call Chains**

**`ERC20Upgradeable._balances`** 🔄
   └─> `_update` *(internal)* ← `_transfer` *(internal)* ← `transfer` *(public)* ← `transferFrom` *(public)* ← `_mint` *(internal)* ← `_burn` *(internal)*

**`ERC20Upgradeable._allowances`** 🔄
   ├─> `_approve` *(internal)* ← `approve` *(public)* ← `_spendAllowance` *(internal)* ← `transferFrom` *(public)*
   └─> `_approve` *(internal)* ← `approve` *(public)* ← `_spendAllowance` *(internal)* ← `transferFrom` *(public)*

**`ERC20Upgradeable._totalSupply`** 🔄
   └─> `_update` *(internal)* ← `_transfer` *(internal)* ← `transfer` *(public)* ← `transferFrom` *(public)* ← `_mint` *(internal)* ← `_burn` *(internal)*

**`ERC20Upgradeable._name`** 🔄
   └─> `__ERC20_init_unchained` *(internal)* ← `__ERC20_init` *(internal)*

**`ERC20Upgradeable._symbol`** 🔄
   └─> `__ERC20_init_unchained` *(internal)* ← `__ERC20_init` *(internal)*

**`JackpotBridgeManager.claimWinnings()`**
   └─> `jackpot.claimWinnings()` *[IJackpot → Jackpot]*
          `└─> jackpotNFT.burnTicket()` [IJackpotTicketNFT → JackpotTicketNFT]
          `└─> usdc.safeTransfer()` [IERC20]

**`Jackpot.claimWinnings()`**
   └─> `jackpotNFT.burnTicket()` *[IJackpotTicketNFT → JackpotTicketNFT]*

**`Jackpot.lpDeposit()`**
   └─> `jackpotLPManager.processDeposit()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              ├─ `lpDrawingState.pendingDeposits`
              │   └─ *also modified by:* `emergencyWithdrawLP`
              ├─ `lpInfo.lastDeposit.amount`
              └─ `lpInfo.lastDeposit.drawingId`
          `└─> _consolidateDeposits` (internal)
                └─> **modifies:**
                    ├─ `lpInfo.lastDeposit`
                    │   └─ *also modified by:* `emergencyWithdrawLP`
                    └─ `lpInfo.consolidatedShares`
                        └─ *also modified by:* `processInitiateWithdraw`, `emergencyWithdrawLP`

**`Jackpot.initiateWithdraw()`**
   └─> `jackpotLPManager.processInitiateWithdraw()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              ├─ `lpInfo.pendingWithdrawal.drawingId`
              ├─ `lpDrawingState.pendingWithdrawals`
              │   └─ *also modified by:* `emergencyWithdrawLP`
              ├─ `lpInfo.consolidatedShares`
              │   └─ *also modified by:* `emergencyWithdrawLP`
              └─ `lpInfo.pendingWithdrawal.amountInShares`
          `└─> _consolidateWithdrawals` (internal)
                └─> **modifies:**
                    ├─ `lpInfo.claimableWithdrawals`
                    │   └─ *also modified by:* `processFinalizeWithdraw`, `emergencyWithdrawLP`
                    └─ `lpInfo.pendingWithdrawal`
                        └─ *also modified by:* `emergencyWithdrawLP`
          `└─> _consolidateDeposits` (internal)
                └─> **modifies:**
                    ├─ `lpInfo.lastDeposit`
                    │   └─ *also modified by:* `emergencyWithdrawLP`
                    └─ `lpInfo.consolidatedShares`
                        └─ *also modified by:* `processInitiateWithdraw`, `emergencyWithdrawLP`

**`Jackpot.emergencyRefundTickets()`**
   └─> `jackpotNFT.burnTicket()` *[IJackpotTicketNFT → JackpotTicketNFT]*

**`Jackpot.initializeLPDeposits()`**
   └─> `jackpotLPManager.initializeLP()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `drawingAccumulator`
                  └─ *also modified by:* `processDrawingSettlement`

**`Jackpot.initializeLPDeposits()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot.setNormalBallMax()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot.setGovernancePoolCap()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot.setLpEdgeTarget()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot.setReserveRatio()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot.setTicketPrice()`**
   └─> `jackpotLPManager.setLPPoolCap()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpPoolCap`

**`Jackpot._setNewDrawingState()`**
   └─> `jackpotLPManager.initializeDrawingLP()` *[IJackpotLPManager → JackpotLPManager]*
          └─> **modifies:**
              └─ `lpDrawingState`

**`Jackpot._validateAndStoreTickets()`**
   └─> `jackpotNFT.mintTicket()` *[IJackpotTicketNFT → JackpotTicketNFT]*
          └─> **modifies:**
              └─ `tickets`


## **Fields with Multiple Entry Points**

*The following fields can be modified through multiple call paths, which may indicate important access control patterns:*

**`lpPoolCap`** in contract **`JackpotLPManager`**
   *6 entry point(s):*
      ├─ `Jackpot.initializeLPDeposits()` → `setLPPoolCap()`
      ├─ `Jackpot.setNormalBallMax()` → `setLPPoolCap()`
      ├─ `Jackpot.setGovernancePoolCap()` → `setLPPoolCap()`
      ├─ `Jackpot.setLpEdgeTarget()` → `setLPPoolCap()`
      ├─ `Jackpot.setReserveRatio()` → `setLPPoolCap()`
      ├─ `Jackpot.setTicketPrice()` → `setLPPoolCap()`

**`lpInfo.lastDeposit`** in contract **`JackpotLPManager`**
   *2 entry point(s):*
      ├─ `Jackpot.lpDeposit()` → `processDeposit()`
      ├─ `Jackpot.initiateWithdraw()` → `processInitiateWithdraw()`

**`_totalSupply`** in contract **`ERC20Upgradeable`**
   *2 entry point(s):*
      ├─ `ERC20Upgradeable.transfer()` → `_update()`
      ├─ `ERC20Upgradeable.transferFrom()` → `_update()`

**`_allowances`** in contract **`ERC20Upgradeable`**
   *2 entry point(s):*
      ├─ `ERC20Upgradeable.transferFrom()` → `_approve()`
      ├─ `ERC20Upgradeable.approve()` → `_approve()`

**`_balances`** in contract **`ERC20Upgradeable`**
   *2 entry point(s):*
      ├─ `ERC20Upgradeable.transferFrom()` → `_update()`
      ├─ `ERC20Upgradeable.transfer()` → `_update()`

**`lpInfo.consolidatedShares`** in contract **`JackpotLPManager`**
   *2 entry point(s):*
      ├─ `Jackpot.lpDeposit()` → `processDeposit()`
      ├─ `Jackpot.initiateWithdraw()` → `processInitiateWithdraw()`


---

*Generated by MainnetReady - Solidity Enhanced Analyzer*
