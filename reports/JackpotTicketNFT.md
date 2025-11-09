════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `JackpotTicketNFT`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/JackpotTicketNFT.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 12 (9 public/external entry points)
   • State Variables: 3 (2 mutable)
   • Events: 0
   • Modifiers: 1
   • Custom Errors: 1

🔒 **Security Findings:**
   • Total: 4 finding(s) detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`userTickets`**
   **Type:** `mapping(address => mapping(uint256 => UserTickets))`
   **Visibility:** internal

   **Modified by:**
      ├─ `_beforeTokenTransfer` *(internal)*
      └─ `_afterTokenTransfer` *(internal)*

   **Read by:**
      ├─ `getUserTickets` *(external)*
      ├─ `_beforeTokenTransfer` *(internal)*
      └─ `_afterTokenTransfer` *(internal)*


**`tickets`**
   **Type:** `mapping(uint256 => TrackedTicket)`
   **Visibility:** public

   **Modified by:**
      └─ `mintTicket` *(external)*

   **Read by:**
      ├─ `getTicketInfo` *(external)*
      ├─ `_beforeTokenTransfer` *(internal)*
      ├─ `_afterTokenTransfer` *(internal)*
      └─ `_getExtendedTicketInfo` *(internal)* ← `getExtendedTicketInfo` *(external)*


**`jackpot`**
   **Type:** `IJackpot`
   **Visibility:** public, immutable

   **Modified by:**
      └─ `constructor` *(public)*

   **Read by:**
      └─ `_getExtendedTicketInfo` *(internal)* ← `getExtendedTicketInfo` *(external)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**MODIFIERS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`onlyJackpot()`**

   **Used in:**
      ├─ `mintTicket`
      └─ `burnTicket`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`UnauthorizedCaller`**

   **Used in:**
      └─ `onlyJackpot`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(IJackpot _jackpot)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 81


**`mintTicket(address _recipient, uint256 _ticketId, uint256 _drawingId, uint256 _packedTicket, bytes32 _referralScheme)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 112

   **Modifiers:**
      └─ `onlyJackpot`


**`burnTicket(uint256 _ticketId)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 128

   **Modifiers:**
      └─ `onlyJackpot`


**`getUserTickets(address _userAddress, uint256 _drawingId)`** → `ExtendedTrackedTicket[]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 136


**`getTicketInfo(uint256 _ticketId)`** → `TrackedTicket`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 146


**`getExtendedTicketInfo(uint256 _ticketId)`** → `ExtendedTrackedTicket`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 150


**`name()`** → `string`
   **Visibility:** public
   **State Mutability:** pure
   **Line:** 154


**`symbol()`** → `string`
   **Visibility:** public
   **State Mutability:** pure
   **Line:** 158


**`tokenURI(uint256)`** → `string`
   **Visibility:** public
   **State Mutability:** pure
   **Line:** 162


**`_beforeTokenTransfer(address _from, address, uint256 _tokenId)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 170


**`_afterTokenTransfer(address, address _to, uint256 _tokenId)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 187


**`_getExtendedTicketInfo(uint256 _ticketId)`** → `ExtendedTrackedTicket`
   **Visibility:** internal
   **State Mutability:** view
   **Line:** 198

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_jackpot`:
   Influences:
      • `jackpot`

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

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**IGNORED RETURN VALUES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ **Warning:** The following function calls have return values that are not checked.
Ignoring return values can lead to silent failures and security vulnerabilities.

### ⚠️ LOW Severity

1. **In function:** `mintTicket`
   - **Ignored call:** `_mint()`

2. **In function:** `burnTicket`
   - **Ignored call:** `_burn()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
