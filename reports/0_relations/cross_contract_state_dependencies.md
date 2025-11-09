# Cross-Contract State Variable Dependencies

**Shows how functions in one contract access/modify state variables in other contracts**

════════════════════════════════════════════════════════════════════════════════

════════════════════════════════════════════════════════════════════════════════
## 📦 Jackpot (Source Contract)
════════════════════════════════════════════════════════════════════════════════

### 🎯 Target: **JackpotLPManager**

#### 🔗 `Jackpot.lpDeposit()` → `JackpotLPManager.processDeposit()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`
      - `JackpotLPManager.lpPoolCap`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.initiateWithdraw()` → `JackpotLPManager.processInitiateWithdraw()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.finalizeWithdraw()` → `JackpotLPManager.processFinalizeWithdraw()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpInfo`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpInfo`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.emergencyWithdrawLP()` → `JackpotLPManager.emergencyWithdrawLP()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.drawingAccumulator`
      - `JackpotLPManager.PRECISE_UNIT`
      - `JackpotLPManager.lpInfo`
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.initializeLPDeposits()` → `JackpotLPManager.initializeLP()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.drawingAccumulator`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.PRECISE_UNIT`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.initializeLPDeposits()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.setNormalBallMax()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.setGovernancePoolCap()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.setLpEdgeTarget()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.setReserveRatio()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.setTicketPrice()` → `JackpotLPManager.setLPPoolCap()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpPoolCap`

   🔍 **Reads State Variables:**
      - `JackpotLPManager.lpDrawingState`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot._setNewDrawingState()` → `JackpotLPManager.initializeDrawingLP()`

   ✏️  **Modifies State Variables:**
      - `JackpotLPManager.lpDrawingState`


────────────────────────────────────────────────────────────────────────────────

### 🎯 Target: **JackpotTicketNFT**

#### 🔗 `Jackpot.claimWinnings()` → `JackpotTicketNFT.getTicketInfo()`

   🔍 **Reads State Variables:**
      - `JackpotTicketNFT.tickets`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.emergencyRefundTickets()` → `JackpotTicketNFT.getTicketInfo()`

   🔍 **Reads State Variables:**
      - `JackpotTicketNFT.tickets`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.getTicketTierIds()` → `JackpotTicketNFT.getTicketInfo()`

   🔍 **Reads State Variables:**
      - `JackpotTicketNFT.tickets`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot._validateAndStoreTickets()` → `JackpotTicketNFT.mintTicket()`

   ✏️  **Modifies State Variables:**
      - `JackpotTicketNFT.tickets`


────────────────────────────────────────────────────────────────────────────────

### 🎯 Target: **ScaledEntropyProvider**

#### 🔗 `Jackpot.runJackpot()` → `ScaledEntropyProvider.getFee()`

   🔍 **Reads State Variables:**
      - `ScaledEntropyProvider.entropyProvider`
      - `ScaledEntropyProvider.entropy`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `Jackpot.getEntropyCallbackFee()` → `ScaledEntropyProvider.getFee()`

   🔍 **Reads State Variables:**
      - `ScaledEntropyProvider.entropyProvider`
      - `ScaledEntropyProvider.entropy`



════════════════════════════════════════════════════════════════════════════════
## 📦 JackpotBridgeManager (Source Contract)
════════════════════════════════════════════════════════════════════════════════

### 🎯 Target: **Jackpot**

#### 🔗 `JackpotBridgeManager.buyTickets()` → `Jackpot.buyTickets()`

   ✏️  **Modifies State Variables:**
      - `Jackpot.drawingState`

   🔍 **Reads State Variables:**
      - `Jackpot.drawingState`
      - `Jackpot.currentDrawingId`
      - `Jackpot.usdc`

   ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

#### 🔗 `JackpotBridgeManager.claimWinnings()` → `Jackpot.claimWinnings()`

   🔍 **Reads State Variables:**
      - `Jackpot.usdc`
      - `Jackpot.jackpotNFT`
      - `Jackpot.drawingState`
      - `Jackpot.payoutCalculator`
      - `Jackpot.currentDrawingId`




---

*Generated by MainnetReady - Solidity Enhanced Analyzer*
