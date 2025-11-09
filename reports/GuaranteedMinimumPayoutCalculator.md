════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `GuaranteedMinimumPayoutCalculator`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/GuaranteedMinimumPayoutCalculator.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 15 (12 public/external entry points)
   • State Variables: 10 (6 mutable)
   • Events: 0
   • Modifiers: 1
   • Custom Errors: 4

🔒 **Security Findings:**
   • Total: 16 finding(s) detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`PRECISE_UNIT`**
   **Type:** `uint256`
   **Visibility:** public, constant
   **Read by:**
      ├─ `constructor` *(public)*
      ├─ `setPremiumTierMinAllocation` *(external)*
      └─ `_setPremiumTierWeights` *(internal)* ← `constructor` *(public)* ← `setPremiumTierWeights` *(external)*


**`NORMAL_BALL_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant
   **Read by:**
      └─ `_calculateTierTotalWinningCombos` *(internal)*


**`TOTAL_TIER_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant
   **Read by:**
      ├─ `calculateAndStoreDrawingUserWinnings` *(external)*
      ├─ `getDrawingTierPayouts` *(external)*
      ├─ `_setPremiumTierWeights` *(internal)* ← `constructor` *(public)* ← `setPremiumTierWeights` *(external)*
      └─ `_calculateAndStoreTierPayouts` *(internal)*


**`drawingTierInfo`**
   **Type:** `mapping(uint256 => DrawingTierInfo)`
   **Visibility:** public

   **Modified by:**
      └─ `setDrawingTierInfo` *(external)*

   **Read by:**
      ├─ `calculateAndStoreDrawingUserWinnings` *(external)*
      ├─ `getDrawingTierInfo` *(external)*
      └─ `_calculateAndStoreTierPayouts` *(internal)*


**`tierPayouts`**
   **Type:** `mapping(uint256 => mapping(uint256 => uint256))`
   **Visibility:** internal

   **Modified by:**
      └─ `_calculateAndStoreTierPayouts` *(internal)*

   **Read by:**
      ├─ `getTierPayout` *(external)*
      └─ `getDrawingTierPayouts` *(external)*


**`premiumTierWeights`**
   **Type:** `uint256[TOTAL_TIER_COUNT]`
   **Visibility:** public

   **Modified by:**
      └─ `_setPremiumTierWeights` *(internal)* ← `constructor` *(public)* ← `setPremiumTierWeights` *(external)*

   **Read by:**
      └─ `getPremiumTierWeights` *(external)*


**`minPayoutTiers`**
   **Type:** `bool[TOTAL_TIER_COUNT]`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setMinPayoutTiers` *(external)*

   **Read by:**
      └─ `getMinPayoutTiers` *(external)*


**`minimumPayout`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setMinimumPayout` *(external)*


**`premiumTierMinAllocation`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setPremiumTierMinAllocation` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**MODIFIERS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`onlyJackpot()`**

   **Used in:**
      ├─ `calculateAndStoreDrawingUserWinnings`
      └─ `setDrawingTierInfo`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`UnauthorizedCaller`**

   **Used in:**
      └─ `onlyJackpot`


**`ZeroAddress`**

   **Used in:**
      └─ `constructor`


**`InvalidTierWeights`**

   **Used in:**
      └─ `_setPremiumTierWeights`


**`InvalidPremiumTierMinimumAllocation`**

   **Used in:**
      ├─ `setPremiumTierMinAllocation`
      └─ `constructor`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(IJackpot _jackpot, uint256 _minimumPayout, uint256 _premiumTierMinAllocation, bool[TOTAL_TIER_COUNT] _minPayoutTiers, uint256[TOTAL_TIER_COUNT] _premiumTierWeights)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 114

   **Modifiers:**
      └─ `Ownable`


**`calculateAndStoreDrawingUserWinnings(uint256 _drawingId, uint256 _prizePool, uint8 _normalMax, uint8 _bonusballMax, uint256[] _uniqueResult, uint256[] _dupResult)`** → `uint256 totalPayout`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 167

   **Modifiers:**
      └─ `onlyJackpot`


**`setDrawingTierInfo(uint256 _drawingId)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 236

   **Modifiers:**
      └─ `onlyJackpot`


**`setMinimumPayout(uint256 _minimumPayout)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 263

   **Modifiers:**
      └─ `onlyOwner`


**`setMinPayoutTiers(bool[TOTAL_TIER_COUNT] _minPayoutTiers)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 284

   **Modifiers:**
      └─ `onlyOwner`


**`setPremiumTierMinAllocation(uint256 _premiumTierMinAllocation)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 309

   **Modifiers:**
      └─ `onlyOwner`


**`setPremiumTierWeights(uint256[TOTAL_TIER_COUNT] _premiumTierWeights)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 333

   **Modifiers:**
      └─ `onlyOwner`


**`getTierPayout(uint256 _drawingId, uint256 _tierId)`** → `uint256`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 349


**`getDrawingTierPayouts(uint256 _drawingId)`** → `uint256[TOTAL_TIER_COUNT]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 360


**`getMinPayoutTiers()`** → `bool[TOTAL_TIER_COUNT]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 374


**`getPremiumTierWeights()`** → `uint256[TOTAL_TIER_COUNT]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 384


**`getDrawingTierInfo(uint256 _drawingId)`** → `DrawingTierInfo`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 395


**`_setPremiumTierWeights(uint256[TOTAL_TIER_COUNT] _premiumTierWeights)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 402


**`_calculateAndStoreTierPayouts(uint256 _drawingId, uint256 _remainingPrizePool, uint256 _minPayout, uint256[TOTAL_TIER_COUNT] _tierWinners, uint256[] _uniqueResult, uint256[] _dupResult)`** → `uint256 totalPayout`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 412


**`_calculateTierTotalWinningCombos(uint256 _matches, uint8 _normalMax, uint8 _bonusballMax, bool _bonusballMatch)`** → `uint256`
   **Visibility:** internal
   **State Mutability:** pure
   **Line:** 440

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_jackpot`:
   Influences:
      • `jackpot`

**`constructor`** - Parameter `_minimumPayout`:
   Influences:
      • `minimumPayout`

**`constructor`** - Parameter `_premiumTierMinAllocation`:
   Influences:
      • `premiumTierMinAllocation`

**`constructor`** - Parameter `_minPayoutTiers`:
   Influences:
      • `minPayoutTiers`

**`setMinimumPayout`** - Parameter `_minimumPayout`:
   Influences:
      • `minimumPayout`

**`setMinPayoutTiers`** - Parameter `_minPayoutTiers`:
   Influences:
      • `minPayoutTiers`

**`setPremiumTierMinAllocation`** - Parameter `_premiumTierMinAllocation`:
   Influences:
      • `premiumTierMinAllocation`

**`_setPremiumTierWeights`** - Parameter `_premiumTierWeights`:
   Influences:
      • `premiumTierWeights`

**`_calculateAndStoreTierPayouts`** - Parameter `_drawingId`:
   Influences:
      • `tierPayouts`

**`_calculateAndStoreTierPayouts`** - Parameter `_remainingPrizePool`:
   Influences:
      • `tierPayouts`

**`_calculateAndStoreTierPayouts`** - Parameter `_minPayout`:
   Influences:
      • `tierPayouts`

**`_calculateAndStoreTierPayouts`** - Parameter `_tierWinners`:
   Influences:
      • `tierPayouts`

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

3. **Function:** `constructor`
   - **Source:** Function parameter `_minimumPayout`
   - **Sink:** State modification: `minimumPayout`
   - **Status:** ⚠️ No validation detected

4. **Function:** `constructor`
   - **Source:** Function parameter `_minimumPayout`
   - **Sink:** State modification: `minimumPayout`
   - **Status:** ⚠️ No validation detected

5. **Function:** `constructor`
   - **Source:** Function parameter `_premiumTierMinAllocation`
   - **Sink:** State modification: `premiumTierMinAllocation`
   - **Status:** ✅ Validated

6. **Function:** `constructor`
   - **Source:** Function parameter `_premiumTierMinAllocation`
   - **Sink:** State modification: `premiumTierMinAllocation`
   - **Status:** ✅ Validated

7. **Function:** `constructor`
   - **Source:** Function parameter `_minPayoutTiers`
   - **Sink:** State modification: `minPayoutTiers`
   - **Status:** ⚠️ No validation detected

8. **Function:** `constructor`
   - **Source:** Function parameter `_minPayoutTiers`
   - **Sink:** State modification: `minPayoutTiers`
   - **Status:** ⚠️ No validation detected

9. **Function:** `setMinimumPayout`
   - **Source:** Function parameter `_minimumPayout`
   - **Sink:** State modification: `minimumPayout`
   - **Status:** ⚠️ No validation detected

10. **Function:** `setMinimumPayout`
   - **Source:** Function parameter `_minimumPayout`
   - **Sink:** State modification: `minimumPayout`
   - **Status:** ⚠️ No validation detected

11. **Function:** `setMinPayoutTiers`
   - **Source:** Function parameter `_minPayoutTiers`
   - **Sink:** State modification: `minPayoutTiers`
   - **Status:** ⚠️ No validation detected

12. **Function:** `setMinPayoutTiers`
   - **Source:** Function parameter `_minPayoutTiers`
   - **Sink:** State modification: `minPayoutTiers`
   - **Status:** ⚠️ No validation detected

13. **Function:** `setPremiumTierMinAllocation`
   - **Source:** Function parameter `_premiumTierMinAllocation`
   - **Sink:** State modification: `premiumTierMinAllocation`
   - **Status:** ✅ Validated

14. **Function:** `setPremiumTierMinAllocation`
   - **Source:** Function parameter `_premiumTierMinAllocation`
   - **Sink:** State modification: `premiumTierMinAllocation`
   - **Status:** ✅ Validated

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**IGNORED RETURN VALUES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ **Warning:** The following function calls have return values that are not checked.
Ignoring return values can lead to silent failures and security vulnerabilities.

### ⚠️ LOW Severity

1. **In function:** `constructor`
   - **Ignored call:** `_setPremiumTierWeights()`

2. **In function:** `setPremiumTierWeights`
   - **Ignored call:** `_setPremiumTierWeights()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
