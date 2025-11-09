════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `ScaledEntropyProvider`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/ScaledEntropyProvider.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 13 (7 public/external entry points)
   • State Variables: 3 (3 mutable)
   • Events: 2
   • Modifiers: 0
   • Custom Errors: 9

🔒 **Security Findings:**
   • Total: 14 finding(s) detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`entropy`**
   **Type:** `IEntropyV2`
   **Visibility:** private

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      ├─ `getFee` *(public)*
      ├─ `getEntropyContract` *(external)*
      └─ `getEntropy` *(internal)*


**`entropyProvider`**
   **Type:** `address`
   **Visibility:** private

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setEntropyProvider` *(external)*

   **Read by:**
      ├─ `requestAndCallbackScaledRandomness` *(external)*
      ├─ `getFee` *(public)*
      └─ `getEntropyProvider` *(external)*


**`pending`**
   **Type:** `mapping(uint64 => PendingRequest)`
   **Visibility:** private

   **Modified by:**
      ├─ `entropyCallback` *(internal)*
      └─ `_storePendingRequest` *(internal)* ← `requestAndCallbackScaledRandomness` *(external)*

   **Read by:**
      ├─ `getPendingRequest` *(external)*
      ├─ `entropyCallback` *(internal)*
      └─ `_storePendingRequest` *(internal)* ← `requestAndCallbackScaledRandomness` *(external)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**EVENTS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`ScaledRandomnessDelivered`**
   **Parameters:** `uint64` sequence *(indexed)*, `address` callback *(indexed)*, `uint256` samples

   **Emitted in:**
      └─ `entropyCallback`


**`EntropyFulfilled`**
   **Parameters:** `uint64` sequence *(indexed)*, `bytes32` randomNumber

   **Emitted in:**
      └─ `entropyCallback`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`InvalidCallback`**

   **Used in:** *None*


**`CallbackFailed`**
   **Parameters:** `bytes4` selector

   **Used in:**
      └─ `entropyCallback`


**`ZeroAddress`**

   **Used in:**
      ├─ `constructor`
      └─ `setEntropyProvider`


**`InvalidSelector`**

   **Used in:**
      └─ `requestAndCallbackScaledRandomness`


**`InvalidRequests`**

   **Used in:**
      └─ `_validateRequests`


**`InvalidRange`**

   **Used in:**
      └─ `_validateRequests`


**`InvalidSamples`**

   **Used in:**
      └─ `_validateRequests`


**`InsufficientFee`**

   **Used in:**
      └─ `requestAndCallbackScaledRandomness`


**`UnknownSequence`**

   **Used in:**
      └─ `entropyCallback`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(address _entropy, address _entropyProvider)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 95

   **Modifiers:**
      └─ `Ownable`


**`requestAndCallbackScaledRandomness(uint32 _gasLimit, SetRequest[] _requests, bytes4 _selector, bytes _context)`** → `uint64 sequence`
   **Visibility:** external
   **State Mutability:** payable
   **Line:** 133


**`getFee(uint32 _gasLimit)`** → `uint256`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 159


**`getEntropyContract()`** → `address`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 168


**`getEntropyProvider()`** → `address`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 177


**`getPendingRequest(uint64 sequence)`** → `PendingRequest`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 188


**`setEntropyProvider(address _entropyProvider)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 212

   **Modifiers:**
      └─ `onlyOwner`


**`entropyCallback(uint64 sequence, address, bytes32 randomNumber)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 245


**`_getScaledRandomness(bytes32 _randomNumber, SetRequest[] _setRequests)`** → `uint256[][] requestsOutputs`
   **Visibility:** internal
   **State Mutability:** pure
   **Line:** 259


**`getEntropy()`** → `address`
   **Visibility:** internal
   **State Mutability:** view
   **Line:** 288


**`_validateRequests(SetRequest[] _requests)`**
   **Visibility:** internal
   **State Mutability:** pure
   **Line:** 292


**`_storePendingRequest(uint64 sequence, bytes4 _selector, bytes _context, SetRequest[] _setRequests)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 300


**`_drawWithReplacement(uint256 _minRange, uint256 _maxRange, uint8 _samples, uint256 _randomNumber)`** → `uint256[]`
   **Visibility:** internal
   **State Mutability:** pure
   **Line:** 314

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_entropy`:
   Influences:
      • `entropy`

**`constructor`** - Parameter `_entropyProvider`:
   Influences:
      • `entropyProvider`

**`setEntropyProvider`** - Parameter `_entropyProvider`:
   Influences:
      • `entropyProvider`

**`_storePendingRequest`** - Parameter `_selector`:
   Influences:
      • `pending`

**`_storePendingRequest`** - Parameter `_context`:
   Influences:
      • `pending`

### Data Flow Security Findings

#### 🟡 MEDIUM Severity

1. **Function:** `constructor`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

2. **Function:** `constructor`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

3. **Function:** `constructor`
   - **Source:** Function parameter `_entropyProvider`
   - **Sink:** State modification: `entropyProvider`
   - **Status:** ✅ Validated

4. **Function:** `constructor`
   - **Source:** Function parameter `_entropyProvider`
   - **Sink:** State modification: `entropyProvider`
   - **Status:** ✅ Validated

5. **Function:** `setEntropyProvider`
   - **Source:** Function parameter `_entropyProvider`
   - **Sink:** State modification: `entropyProvider`
   - **Status:** ✅ Validated

6. **Function:** `setEntropyProvider`
   - **Source:** Function parameter `_entropyProvider`
   - **Sink:** State modification: `entropyProvider`
   - **Status:** ✅ Validated

#### ⚠️ LOW Severity

1. **Function:** `_storePendingRequest`
   - **Source:** msg.sender
   - **Sink:** State modification: `pending`
   - **Status:** ⚠️ No validation detected

2. **Function:** `_storePendingRequest`
   - **Source:** msg.sender
   - **Sink:** State modification: `pending.setRequests`
   - **Status:** ⚠️ No validation detected

3. **Function:** `_storePendingRequest`
   - **Source:** msg.sender
   - **Sink:** State modification: `pending.selector`
   - **Status:** ⚠️ No validation detected

4. **Function:** `_storePendingRequest`
   - **Source:** msg.sender
   - **Sink:** State modification: `pending.callback`
   - **Status:** ⚠️ No validation detected

5. **Function:** `_storePendingRequest`
   - **Source:** msg.sender
   - **Sink:** State modification: `pending.context`
   - **Status:** ⚠️ No validation detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**IGNORED RETURN VALUES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ **Warning:** The following function calls have return values that are not checked.
Ignoring return values can lead to silent failures and security vulnerabilities.

### ⚠️ LOW Severity

1. **In function:** `requestAndCallbackScaledRandomness`
   - **Ignored call:** `_validateRequests()`

2. **In function:** `requestAndCallbackScaledRandomness`
   - **Ignored call:** `_storePendingRequest()`

3. **In function:** `_storePendingRequest`
   - **Ignored call:** `pending.push()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
