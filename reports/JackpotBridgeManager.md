════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `JackpotBridgeManager`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/JackpotBridgeManager.sol`


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

**`CLAIM_TICKET_TYPEHASH`**
   **Type:** `bytes32`
   **Visibility:** public, constant

**`RELAY_TYPEHASH`**
   **Type:** `bytes32`
   **Visibility:** public, constant

**`userTickets`**
   **Type:** `mapping(address => mapping(uint256 => UserTickets))`
   **Visibility:** public

   **Modified by:**
      └─ `buyTickets` *(external)*


**`ticketOwner`**
   **Type:** `mapping(uint256 => address)`
   **Visibility:** public

   **Modified by:**
      ├─ `buyTickets` *(external)*
      └─ `_updateTicketOwnership` *(private)* ← `claimTickets` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*


**`jackpotTicketNFT`**
   **Type:** `IJackpotTicketNFT`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*


**`usdc`**
   **Type:** `IERC20`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*


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


**`JackpotErrors.ZeroAddress`** *(inherited)*

   **Used in:**
      ├─ `claimTickets`
      └─ `buyTickets`


**`JackpotErrors.NotTicketOwner`** *(inherited)*

   **Used in:**
      └─ `_validateTicketOwnership`


**`JackpotErrors.InvalidRecipient`** *(inherited)*

   **Used in:**
      └─ `claimTickets`


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

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
