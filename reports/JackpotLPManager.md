════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `JackpotLPManager`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/JackpotLPManager.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`PRECISE_UNIT`**
   **Type:** `uint256`
   **Visibility:** internal, constant

**`lpDrawingState`**
   **Type:** `mapping(uint256 => LPDrawingState)`
   **Visibility:** internal

   **Modified by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      └─ `initializeDrawingLP` *(external)*


**`lpInfo`**
   **Type:** `mapping(address => LP)`
   **Visibility:** public

   **Modified by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `processFinalizeWithdraw` *(external)*
      └─ `emergencyWithdrawLP` *(external)*


**`drawingAccumulator`**
   **Type:** `mapping(uint256 => uint256)`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeLP` *(external)*
      └─ `processDrawingSettlement` *(external)*


**`lpPoolCap`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      └─ `setLPPoolCap` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public

   **Modified by:**
      └─ `constructor` *(public)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**EVENTS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`LpDeposited`**
   **Parameters:** `address` lpAddress *(indexed)*, `uint256` currentDrawingId *(indexed)*, `uint256` amount, `uint256` totalPendingDeposits

   **Emitted in:**
      └─ `processDeposit`


**`LpWithdrawInitiated`**
   **Parameters:** `address` lpAddress *(indexed)*, `uint256` currentDrawingId *(indexed)*, `uint256` amount, `uint256` totalPendingWithdrawals

   **Emitted in:**
      └─ `processInitiateWithdraw`


**`LpWithdrawFinalized`**
   **Parameters:** `address` lpAddress *(indexed)*, `uint256` currentDrawingId *(indexed)*, `uint256` amount

   **Emitted in:**
      ├─ `processFinalizeWithdraw`
      └─ `emergencyWithdrawLP`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**MODIFIERS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`onlyJackpot()`**

   **Used in:**
      ├─ `initializeLP`
      ├─ `processDeposit`
      ├─ `processInitiateWithdraw`
      ├─ `processFinalizeWithdraw`
      ├─ `emergencyWithdrawLP`
      ├─ `processDrawingSettlement`
      ├─ `initializeDrawingLP`
      └─ `setLPPoolCap`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`UnauthorizedCaller`**

   **Used in:**
      └─ `onlyJackpot`


**`ZeroAddress`**

   **Used in:**
      └─ `constructor`


**`InvalidLPPoolCap`**

   **Used in:**
      └─ `setLPPoolCap`


**`JackpotErrors.ExceedsPoolCap`** *(inherited)*

   **Used in:**
      └─ `processDeposit`


**`JackpotErrors.NothingToWithdraw`** *(inherited)*

   **Used in:**
      └─ `processFinalizeWithdraw`


**`JackpotErrors.InsufficientShares`** *(inherited)*

   **Used in:**
      └─ `processInitiateWithdraw`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(IJackpot _jackpot)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 134

   **Modifiers:**
      └─ `Ownable`


**`initializeLP()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 156

   **Modifiers:**
      └─ `onlyJackpot`


**`processDeposit(uint256 _drawingId, address _lpAddress, uint256 _amount)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 180

   **Modifiers:**
      └─ `onlyJackpot`


**`processInitiateWithdraw(uint256 _drawingId, address _lpAddress, uint256 _amountToWithdrawInShares)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 219

   **Modifiers:**
      └─ `onlyJackpot`


**`processFinalizeWithdraw(uint256 _drawingId, address _lpAddress)`** → `uint256 withdrawableAmount`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 256

   **Modifiers:**
      └─ `onlyJackpot`


**`emergencyWithdrawLP(uint256 _drawingId, address _user)`** → `uint256 withdrawableAmount`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 291

   **Modifiers:**
      └─ `onlyJackpot`


**`processDrawingSettlement(uint256 _drawingId, uint256 _lpEarnings, uint256 _userWinnings, uint256 _protocolFeeAmount)`** → `uint256 newLPValue, uint256 newAccumulator`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 371

   **Modifiers:**
      └─ `onlyJackpot`


**`initializeDrawingLP(uint256 _drawingId, uint256 _initialLPValue)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 408

   **Modifiers:**
      └─ `onlyJackpot`


**`setLPPoolCap(uint256 _drawingId, uint256 _lpPoolCap)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 431

   **Modifiers:**
      └─ `onlyJackpot`


**`getDrawingAccumulator(uint256 _drawingId)`** → `uint256`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 447


**`getLpInfo(address _lpAddress)`** → `LP`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 457


**`getLPValueBreakdown(address _lpAddress)`** → `LPValueBreakdown breakdown`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 486


**`getLPDrawingState(uint256 _drawingId)`** → `LPDrawingState`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 514


**`_consolidateDeposits(LP _lp, uint256 _drawingId)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 522


**`_consolidateWithdrawals(LP _lp, uint256 _drawingId)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 531

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
