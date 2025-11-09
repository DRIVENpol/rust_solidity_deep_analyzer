════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `JackpotLPManager`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/JackpotLPManager.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 15 (13 public/external entry points)
   • State Variables: 6 (5 mutable)
   • Events: 3
   • Modifiers: 1
   • Custom Errors: 6

🔒 **Security Findings:**
   • Total: 30 finding(s) detected

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
   **Read by:**
      ├─ `initializeLP` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `processDrawingSettlement` *(external)*
      ├─ `getLPValueBreakdown` *(external)*
      └─ `_consolidateWithdrawals` *(internal)* ← `processInitiateWithdraw` *(external)* ← `processFinalizeWithdraw` *(external)* ← `emergencyWithdrawLP` *(external)*


**`lpDrawingState`**
   **Type:** `mapping(uint256 => LPDrawingState)`
   **Visibility:** internal

   **Modified by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      └─ `initializeDrawingLP` *(external)*

   **Read by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `processDrawingSettlement` *(external)*
      ├─ `setLPPoolCap` *(external)*
      └─ `getLPDrawingState` *(external)*


**`lpInfo`**
   **Type:** `mapping(address => LP)`
   **Visibility:** public

   **Modified by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `processFinalizeWithdraw` *(external)*
      └─ `emergencyWithdrawLP` *(external)*

   **Read by:**
      ├─ `processDeposit` *(external)*
      ├─ `processInitiateWithdraw` *(external)*
      ├─ `processFinalizeWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `getLpInfo` *(external)*
      └─ `getLPValueBreakdown` *(external)*


**`drawingAccumulator`**
   **Type:** `mapping(uint256 => uint256)`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeLP` *(external)*
      └─ `processDrawingSettlement` *(external)*

   **Read by:**
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `getDrawingAccumulator` *(external)*
      ├─ `getLPValueBreakdown` *(external)*
      └─ `_consolidateDeposits` *(internal)* ← `processDeposit` *(external)* ← `processInitiateWithdraw` *(external)* ← `emergencyWithdrawLP` *(external)*


**`lpPoolCap`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      └─ `setLPPoolCap` *(external)*

   **Read by:**
      └─ `processDeposit` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      └─ `getLPValueBreakdown` *(external)*


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


**`JackpotErrors.NothingToWithdraw`** *(inherited)*

   **Used in:**
      └─ `processFinalizeWithdraw`


**`JackpotErrors.ExceedsPoolCap`** *(inherited)*

   **Used in:**
      └─ `processDeposit`


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

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_jackpot`:
   Influences:
      • `jackpot`

**`processDeposit`** - Parameter `_amount`:
   Influences:
      • `lpDrawingState`

**`processInitiateWithdraw`** - Parameter `_amountToWithdrawInShares`:
   Influences:
      • `lpDrawingState`

**`emergencyWithdrawLP`** - Parameter `_drawingId`:
   Influences:
      • `lpDrawingState`

**`emergencyWithdrawLP`** - Parameter `_user`:
   Influences:
      • `lpDrawingState`

**`processDrawingSettlement`** - Parameter `_drawingId`:
   Influences:
      • `drawingAccumulator`

**`processDrawingSettlement`** - Parameter `_lpEarnings`:
   Influences:
      • `drawingAccumulator`

**`processDrawingSettlement`** - Parameter `_userWinnings`:
   Influences:
      • `drawingAccumulator`

**`processDrawingSettlement`** - Parameter `_protocolFeeAmount`:
   Influences:
      • `drawingAccumulator`

**`setLPPoolCap`** - Parameter `_lpPoolCap`:
   Influences:
      • `lpPoolCap`

### Data Flow Security Findings

#### 🟡 MEDIUM Severity

1. **Function:** `constructor`
   - **Source:** Function parameter `_jackpot`
   - **Sink:** State modification: `jackpot`
   - **Status:** ✅ Validated

2. **Function:** `constructor`
   - **Source:** Function parameter `_jackpot`
   - **Sink:** State modification: `jackpot`
   - **Status:** ✅ Validated

3. **Function:** `processDeposit`
   - **Source:** Function parameter `_amount`
   - **Sink:** State modification: `lpDrawingState`
   - **Status:** ✅ Validated

4. **Function:** `processDeposit`
   - **Source:** Function parameter `_amount`
   - **Sink:** State modification: `lpDrawingState.pendingDeposits`
   - **Status:** ✅ Validated

5. **Function:** `processInitiateWithdraw`
   - **Source:** Function parameter `_amountToWithdrawInShares`
   - **Sink:** State modification: `lpDrawingState`
   - **Status:** ✅ Validated

6. **Function:** `processInitiateWithdraw`
   - **Source:** Function parameter `_amountToWithdrawInShares`
   - **Sink:** State modification: `lpDrawingState.pendingWithdrawals`
   - **Status:** ✅ Validated

7. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `lpDrawingState`
   - **Status:** ⚠️ No validation detected

8. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `lpDrawingState.pendingWithdrawals`
   - **Status:** ⚠️ No validation detected

9. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `lpDrawingState.pendingDeposits`
   - **Status:** ⚠️ No validation detected

10. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `lpDrawingState.lpPoolTotal`
   - **Status:** ⚠️ No validation detected

11. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_user`
   - **Sink:** State modification: `lpDrawingState`
   - **Status:** ⚠️ No validation detected

12. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_user`
   - **Sink:** State modification: `lpDrawingState.pendingWithdrawals`
   - **Status:** ⚠️ No validation detected

13. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_user`
   - **Sink:** State modification: `lpDrawingState.pendingDeposits`
   - **Status:** ⚠️ No validation detected

14. **Function:** `emergencyWithdrawLP`
   - **Source:** Function parameter `_user`
   - **Sink:** State modification: `lpDrawingState.lpPoolTotal`
   - **Status:** ⚠️ No validation detected

15. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

16. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_drawingId`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

17. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_lpEarnings`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

18. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_lpEarnings`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

19. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_userWinnings`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

20. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_userWinnings`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

21. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_protocolFeeAmount`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

22. **Function:** `processDrawingSettlement`
   - **Source:** Function parameter `_protocolFeeAmount`
   - **Sink:** State modification: `drawingAccumulator`
   - **Status:** ⚠️ No validation detected

23. **Function:** `setLPPoolCap`
   - **Source:** Function parameter `_lpPoolCap`
   - **Sink:** State modification: `lpPoolCap`
   - **Status:** ✅ Validated

24. **Function:** `setLPPoolCap`
   - **Source:** Function parameter `_lpPoolCap`
   - **Sink:** State modification: `lpPoolCap`
   - **Status:** ✅ Validated

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**IGNORED RETURN VALUES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ **Warning:** The following function calls have return values that are not checked.
Ignoring return values can lead to silent failures and security vulnerabilities.

### ⚠️ LOW Severity

1. **In function:** `processDeposit`
   - **Ignored call:** `_consolidateDeposits()`

2. **In function:** `processInitiateWithdraw`
   - **Ignored call:** `_consolidateDeposits()`

3. **In function:** `processInitiateWithdraw`
   - **Ignored call:** `_consolidateWithdrawals()`

4. **In function:** `processFinalizeWithdraw`
   - **Ignored call:** `_consolidateWithdrawals()`

5. **In function:** `emergencyWithdrawLP`
   - **Ignored call:** `_consolidateDeposits()`

6. **In function:** `emergencyWithdrawLP`
   - **Ignored call:** `_consolidateWithdrawals()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
