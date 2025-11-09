════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `JackpotBridgeManager`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/JackpotBridgeManager.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 18 (15 public/external entry points)
   • State Variables: 8 (2 mutable)
   • Events: 3
   • Modifiers: 0
   • Custom Errors: 7

🔒 **Security Findings:**
   • 🔴 3 HIGH/CRITICAL severity issue(s)
   • Total: 16 finding(s) detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`CLAIM_WINNINGS_TYPEHASH`**
   **Type:** `bytes32`
   **Visibility:** public, constant
   **Read by:**
      ├─ `createClaimWinningsEIP712Hash` *(public)*
      └─ `CLAIM_WINNINGS_TYPEHASH` *(external)*


**`CLAIM_TICKET_TYPEHASH`**
   **Type:** `bytes32`
   **Visibility:** public, constant
   **Read by:**
      ├─ `createClaimTicketEIP712Hash` *(public)*
      └─ `CLAIM_TICKET_TYPEHASH` *(external)*


**`RELAY_TYPEHASH`**
   **Type:** `bytes32`
   **Visibility:** public, constant
   **Read by:**
      ├─ `createClaimWinningsEIP712Hash` *(public)*
      └─ `RELAY_TYPEHASH` *(external)*


**`userTickets`**
   **Type:** `mapping(address => mapping(uint256 => UserTickets))`
   **Visibility:** public

   **Modified by:**
      └─ `buyTickets` *(external)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `getUserTickets` *(external)*
      └─ `userTickets` *(external)*


**`ticketOwner`**
   **Type:** `mapping(uint256 => address)`
   **Visibility:** public

   **Modified by:**
      ├─ `buyTickets` *(external)*
      └─ `_updateTicketOwnership` *(private)* ← `claimTickets` *(external)*

   **Read by:**
      ├─ `getUserTickets` *(external)*
      ├─ `_validateTicketOwnership` *(private)* ← `claimWinnings` *(external)* ← `claimTickets` *(external)*
      └─ `ticketOwner` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `claimWinnings` *(external)*
      └─ `jackpot` *(external)*


**`jackpotTicketNFT`**
   **Type:** `IJackpotTicketNFT`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      └─ `jackpotTicketNFT` *(external)*


**`usdc`**
   **Type:** `IERC20`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `claimWinnings` *(external)*
      ├─ `_bridgeFunds` *(private)* ← `claimWinnings` *(external)*
      └─ `usdc` *(external)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**EVENTS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`TicketsBought`**
   **Parameters:** `address` _recipient *(indexed)*, `uint256` _drawingId *(indexed)*, `uint256[]` _ticketIds

   **Emitted in:**
      └─ `buyTickets`


**`WinningsClaimed`**
   **Parameters:** `address` _winner *(indexed)*, `address` _bridgeProvider *(indexed)*, `uint256[]` _ticketIds, `uint256` _amount

   **Emitted in:**
      └─ `claimWinnings`


**`FundsBridged`**
   **Parameters:** `address` _to *(indexed)*, `uint256` _amount

   **Emitted in:**
      └─ `_bridgeFunds`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`BridgeFundsFailed`**

   **Used in:**
      └─ `_bridgeFunds`


**`NotAllFundsBridged`**

   **Used in:**
      └─ `_bridgeFunds`


**`InvalidClaimedAmount`**

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.NoTicketsToClaim`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.InvalidRecipient`** *(inherited)*

   **Used in:**
      └─ `claimTickets`


**`JackpotErrors.NoTicketsToClaim`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.ZeroAddress`** *(inherited)*

   **Used in:**
      ├─ `claimTickets`
      └─ `buyTickets`


<<<<<<< Updated upstream
=======
**`JackpotErrors.NotTicketOwner`** *(inherited)*

   **Used in:**
      └─ `_validateTicketOwnership`


>>>>>>> Stashed changes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(IJackpot _jackpot, IJackpotTicketNFT _jackpotTicketNFT, IERC20 _usdc, string _name, string _version)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 124

   **Modifiers:**
      ├─ `Ownable`
      └─ `EIP712`


**`buyTickets(IJackpot.Ticket[] _tickets, address _recipient, address[] _referrers, uint256[] _referralSplitBps, bytes32 _source)`** → `uint256[]`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 166

   **Modifiers:**
      └─ `nonReentrant`


**`claimWinnings(uint256[] _userTicketIds, RelayTxData _bridgeDetails, bytes _signature)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 225

   **Modifiers:**
      └─ `nonReentrant`


**`claimTickets(uint256[] _ticketIds, address _recipient, bytes _signature)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 268


**`createClaimWinningsEIP712Hash(uint256[] _userTicketIds, RelayTxData _bridgeDetails)`** → `bytes32`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 292


**`createClaimTicketEIP712Hash(uint256[] _ticketIds, address _recipient)`** → `bytes32`
   **Visibility:** public
   **State Mutability:** view
   **Line:** 306


**`getUserTickets(address _user, uint256 _drawingId)`** → `uint256[]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 322


**`_validateTicketOwnership(uint256[] _userTicketIds, address _signer)`**
   **Visibility:** private
   **State Mutability:** view
   **Line:** 338


**`_bridgeFunds(RelayTxData _bridgeDetails, uint256 _claimedAmount)`**
   **Visibility:** private
   **State Mutability:** nonpayable
   **Line:** 345


**`_updateTicketOwnership(uint256[] _ticketIds, address _recipient)`**
   **Visibility:** private
   **State Mutability:** nonpayable
   **Line:** 364


**`CLAIM_WINNINGS_TYPEHASH()`** → `bytes32`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 82


**`CLAIM_TICKET_TYPEHASH()`** → `bytes32`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 85


**`RELAY_TYPEHASH()`** → `bytes32`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 88


**`userTickets()`** → `mapping(address => mapping(uint256 => UserTickets))`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 94


**`ticketOwner()`** → `mapping(uint256 => address)`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 95


**`jackpot()`** → `IJackpot`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 97


**`jackpotTicketNFT()`** → `IJackpotTicketNFT`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 98


**`usdc()`** → `IERC20`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 99

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_jackpot`:
   Influences:
      • `jackpot`

**`constructor`** - Parameter `_jackpotTicketNFT`:
   Influences:
      • `jackpotTicketNFT`

**`constructor`** - Parameter `_usdc`:
   Influences:
      • `usdc`

**`buyTickets`** - Parameter `_recipient`:
   Influences:
      • `ticketOwner`

### Data Flow Security Findings

#### 🟡 MEDIUM Severity

1. **Function:** `constructor`
   - **Source:** Function parameter `_jackpot`
   - **Sink:** State modification: `jackpot`
   - **Status:** ⚠️ No validation detected

2. **Function:** `constructor`
   - **Source:** Function parameter `_jackpot`
   - **Sink:** State modification: `jackpot`
   - **Status:** ⚠️ No validation detected

3. **Function:** `constructor`
   - **Source:** Function parameter `_jackpotTicketNFT`
   - **Sink:** State modification: `jackpotTicketNFT`
   - **Status:** ⚠️ No validation detected

4. **Function:** `constructor`
   - **Source:** Function parameter `_jackpotTicketNFT`
   - **Sink:** State modification: `jackpotTicketNFT`
   - **Status:** ⚠️ No validation detected

5. **Function:** `constructor`
   - **Source:** Function parameter `_usdc`
   - **Sink:** State modification: `usdc`
   - **Status:** ⚠️ No validation detected

6. **Function:** `constructor`
   - **Source:** Function parameter `_usdc`
   - **Sink:** State modification: `usdc`
   - **Status:** ⚠️ No validation detected

7. **Function:** `buyTickets`
   - **Source:** Function parameter `_recipient`
   - **Sink:** State modification: `ticketOwner`
   - **Status:** ✅ Validated

8. **Function:** `buyTickets`
   - **Source:** Function parameter `_recipient`
   - **Sink:** State modification: `ticketOwner`
   - **Status:** ✅ Validated

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**IGNORED RETURN VALUES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ **Warning:** The following function calls have return values that are not checked.
Ignoring return values can lead to silent failures and security vulnerabilities.

### 🔴 HIGH Severity

1. **In function:** `buyTickets`
   - **Ignored call:** `usdc.safeTransferFrom()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransferFrom`

2. **In function:** `buyTickets`
   - **Ignored call:** `usdc.approve()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `approve`

3. **In function:** `_bridgeFunds`
   - **Ignored call:** `usdc.approve()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `approve`

### 🟡 MEDIUM Severity

1. **In function:** `claimWinnings`
   - **Ignored call:** `jackpot.claimWinnings()`

### ⚠️ LOW Severity

1. **In function:** `claimWinnings`
   - **Ignored call:** `_validateTicketOwnership()`

2. **In function:** `claimWinnings`
   - **Ignored call:** `_bridgeFunds()`

3. **In function:** `claimTickets`
   - **Ignored call:** `_validateTicketOwnership()`

4. **In function:** `claimTickets`
   - **Ignored call:** `_updateTicketOwnership()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
