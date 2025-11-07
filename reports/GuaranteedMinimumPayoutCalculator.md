════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `GuaranteedMinimumPayoutCalculator`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/GuaranteedMinimumPayoutCalculator.sol`


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

**`NORMAL_BALL_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant

**`TOTAL_TIER_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant

**`drawingTierInfo`**
   **Type:** `mapping(uint256 => DrawingTierInfo)`
   **Visibility:** public

   **Modified by:**
      └─ `setDrawingTierInfo` *(external)*


**`tierPayouts`**
   **Type:** `mapping(uint256 => mapping(uint256 => uint256))`
   **Visibility:** internal

   **Modified by:**
      └─ `_calculateAndStoreTierPayouts` *(internal)*


**`premiumTierWeights`**
   **Type:** `uint256[TOTAL_TIER_COUNT]`
   **Visibility:** public

   **Modified by:**
      └─ `_setPremiumTierWeights` *(internal)* ← `constructor` *(public)* ← `setPremiumTierWeights` *(external)*


**`minPayoutTiers`**
   **Type:** `bool[TOTAL_TIER_COUNT]`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setMinPayoutTiers` *(external)*


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

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
