# Function Call Graph Report

**Shows all internal and external function calls within each contract**

════════════════════════════════════════════════════════════════════════════════
## Contract: JackpotLPManager
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

**From `getLPValueBreakdown()`** *(external)*:
   - → `jackpot.currentDrawingId()` *[Jackpot]*

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`constructor()`** *(public)***

`emergencyWithdrawLP()`** *(external)***
   ├─> `_consolidateWithdrawals()` *(internal)*
   └─> `_consolidateDeposits()` *(internal)*

`getDrawingAccumulator()`** *(external)***

`getLPDrawingState()`** *(external)***

`getLPValueBreakdown()`** *(external)***

`getLpInfo()`** *(external)***

`initializeDrawingLP()`** *(external)***

`initializeLP()`** *(external)***

`processDeposit()`** *(external)***
   └─> `_consolidateDeposits()` *(internal)*

`processDrawingSettlement()`** *(external)***

`processFinalizeWithdraw()`** *(external)***
   └─> `_consolidateWithdrawals()` *(internal)*

`processInitiateWithdraw()`** *(external)***
   ├─> `_consolidateWithdrawals()` *(internal)*
   └─> `_consolidateDeposits()` *(internal)*

`setLPPoolCap()`** *(external)***


════════════════════════════════════════════════════════════════════════════════
## Contract: JackpotBridgeManager
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

**From `claimWinnings()`** *(external)*:
   - → `usdc.balanceOf()`
   - → `jackpot.claimWinnings()` *[Jackpot]*
   - → `usdc.balanceOf()`

**From `_bridgeFunds()`** *(private)*:
   - → `usdc.approve()`
   - → `usdc.balanceOf()`
   - → `usdc.balanceOf()`

**From `buyTickets()`** *(external)*:
   - → `jackpot.ticketPrice()` *[Jackpot]*
   - → `jackpot.currentDrawingId()` *[Jackpot]*
   - → `usdc.safeTransferFrom()`
   - → `usdc.approve()`
   - → `jackpot.buyTickets()` *[Jackpot]*

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`buyTickets()`** *(external)***

`claimTickets()`** *(external)***
   ├─> `_updateTicketOwnership()` *(private)*
   └─> `_validateTicketOwnership()` *(private)*

`claimWinnings()`** *(external)***
   ├─> `_validateTicketOwnership()` *(private)*
   └─> `_bridgeFunds()` *(private)*

`constructor()`** *(public)***

`createClaimTicketEIP712Hash()`** *(public)***
   ├─> `_hashTypedDataV4()` *(unknown)*
   └─> `keccak256()` *(unknown)*

`createClaimWinningsEIP712Hash()`** *(public)***
   ├─> `keccak256()` *(unknown)*
   └─> `_hashTypedDataV4()` *(unknown)*

`getUserTickets()`** *(external)***


════════════════════════════════════════════════════════════════════════════════
## Contract: ScaledEntropyProvider
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

**From `getFee()`** *(public)*:
   - → `entropy.getFeeV2()`

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`constructor()`** *(public)***

`getEntropyContract()`** *(external)***

`getEntropyProvider()`** *(external)***

`getFee()`** *(public)***

`getPendingRequest()`** *(external)***

`requestAndCallbackScaledRandomness()`** *(external)***
   ├─> `_storePendingRequest()` *(internal)*
   └─> `_validateRequests()` *(internal)*

`setEntropyProvider()`** *(external)***

**⚠️  Orphaned Internal Functions** *(never called)*:
   - `entropyCallback` *(internal)*
   - `_getScaledRandomness` *(internal)*
   - `getEntropy` *(internal)*
   - `_drawWithReplacement` *(internal)*


════════════════════════════════════════════════════════════════════════════════
## Contract: JackpotTicketNFT
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

*No external calls detected*

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`burnTicket()`** *(external)***
   └─> `_burn()` *(unknown)*

`constructor()`** *(public)***

`getExtendedTicketInfo()`** *(external)***
   └─> `_getExtendedTicketInfo()` *(internal)*

`getTicketInfo()`** *(external)***

`getUserTickets()`** *(external)***

`mintTicket()`** *(external)***
   └─> `_mint()` *(unknown)*

`name()`** *(public)***

`symbol()`** *(public)***

`tokenURI()`** *(public)***

**⚠️  Orphaned Internal Functions** *(never called)*:
   - `_beforeTokenTransfer` *(internal)*
   - `_afterTokenTransfer` *(internal)*


════════════════════════════════════════════════════════════════════════════════
## Contract: GuaranteedMinimumPayoutCalculator
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

*No external calls detected*

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`calculateAndStoreDrawingUserWinnings()`** *(external)***

`constructor()`** *(public)***
   └─> `_setPremiumTierWeights()` *(internal)*

`getDrawingTierInfo()`** *(external)***

`getDrawingTierPayouts()`** *(external)***

`getMinPayoutTiers()`** *(external)***

`getPremiumTierWeights()`** *(external)***

`getTierPayout()`** *(external)***

`setDrawingTierInfo()`** *(external)***

`setMinPayoutTiers()`** *(external)***

`setMinimumPayout()`** *(external)***

`setPremiumTierMinAllocation()`** *(external)***

`setPremiumTierWeights()`** *(external)***
   └─> `_setPremiumTierWeights()` *(internal)*

**⚠️  Orphaned Internal Functions** *(never called)*:
   - `_calculateAndStoreTierPayouts` *(internal)*
   - `_calculateTierTotalWinningCombos` *(internal)*


════════════════════════════════════════════════════════════════════════════════
## Contract: Jackpot
════════════════════════════════════════════════════════════════════════════════

### 📞 External Contract Calls

**From `initiateWithdraw()`** *(external)*:
   - → `jackpotLPManager.processInitiateWithdraw()` *[JackpotLPManager]*

**From `setReserveRatio()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `finalizeWithdraw()`** *(external)*:
   - → `jackpotLPManager.processFinalizeWithdraw()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `emergencyRefundTickets()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
   - → `jackpotNFT.burnTicket()` *[JackpotTicketNFT]*
   - → `usdc.safeTransfer()`

**From `_validateAndStoreTickets()`** *(internal)*:
   - → `jackpotNFT.mintTicket()` *[JackpotTicketNFT]*

**From `_transferProtocolFee()`** *(internal)*:
   - → `usdc.safeTransfer()`

**From `lpDeposit()`** *(external)*:
   - → `usdc.safeTransferFrom()`
   - → `jackpotLPManager.processDeposit()` *[JackpotLPManager]*

**From `runJackpot()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `buyTickets()`** *(external)*:
   - → `usdc.safeTransferFrom()`

**From `setNormalBallMax()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `setGovernancePoolCap()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `setTicketPrice()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `emergencyWithdrawLP()`** *(external)*:
   - → `jackpotLPManager.emergencyWithdrawLP()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `getEntropyCallbackFee()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `initializeLPDeposits()`** *(external)*:
   - → `jackpotLPManager.initializeLP()` *[JackpotLPManager]*
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `_setNewDrawingState()`** *(internal)*:
   - → `jackpotLPManager.initializeDrawingLP()` *[JackpotLPManager]*
   - → `payoutCalculator.setDrawingTierInfo()`

**From `getTicketTierIds()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*

**From `setLpEdgeTarget()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `claimWinnings()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
   - → `jackpotNFT.burnTicket()` *[JackpotTicketNFT]*
   - → `payoutCalculator.getTierPayout()`
   - → `usdc.safeTransfer()`

**From `claimReferralFees()`** *(external)*:
   - → `usdc.safeTransfer()`

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`buyTickets()`** *(external)***
   └─> `_validateBuyTicketInputs()` *(internal)*

`checkIfTicketsBought()`** *(external)***

`claimReferralFees()`** *(external)***

`claimWinnings()`** *(external)***

`constructor()`** *(public)***

`disableEmergencyMode()`** *(external)***

`disableTicketPurchases()`** *(external)***

`emergencyRefundTickets()`** *(external)***

`emergencyWithdrawLP()`** *(external)***

`enableEmergencyMode()`** *(external)***

`enableTicketPurchases()`** *(external)***

`finalizeWithdraw()`** *(external)***

`getDrawingState()`** *(external)***

`getEntropyCallbackFee()`** *(external)***

`getReferralScheme()`** *(external)***

`getSubsetCount()`** *(external)***

`getTicketTierIds()`** *(external)***

`getUnpackedTicket()`** *(external)***

`initialize()`** *(external)***

`initializeJackpot()`** *(external)***
   └─> `_setNewDrawingState()` *(internal)*

`initializeLPDeposits()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`initiateWithdraw()`** *(external)***

`lockJackpot()`** *(external)***
   └─> `_lockJackpot()` *(internal)*

`lpDeposit()`** *(external)***

`runJackpot()`** *(external)***
   └─> `_lockJackpot()` *(internal)*

`scaledEntropyCallback()`** *(external)***
   └─> `_setNewDrawingState()` *(internal)*

`setBonusballMin()`** *(external)***

`setDrawingDurationInSeconds()`** *(external)***

`setEntropy()`** *(external)***

`setEntropyBaseGasLimit()`** *(external)***

`setEntropyVariableGasLimit()`** *(external)***

`setGovernancePoolCap()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`setLpEdgeTarget()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`setMaxReferrers()`** *(external)***

`setNormalBallMax()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`setPayoutCalculator()`** *(external)***

`setProtocolFee()`** *(external)***

`setProtocolFeeAddress()`** *(external)***

`setProtocolFeeThreshold()`** *(external)***

`setReferralFee()`** *(external)***

`setReferralWinShare()`** *(external)***

`setReserveRatio()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`setTicketPrice()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`unlockJackpot()`** *(external)***
   └─> `_unlockJackpot()` *(internal)*

**⚠️  Orphaned Internal Functions** *(never called)*:
   - `_validateAndTrackReferrals` *(internal)*
   - `_validateAndStoreTickets` *(internal)*
   - `_calculateDrawingUserWinnings` *(internal)*
   - `_calculateTicketTierId` *(internal)*
   - `_payReferrersWinnings` *(internal)*
   - `_transferProtocolFee` *(internal)*
   - `_calculateEntropyGasLimit` *(internal)*



---

*Generated by MainnetReady - Solidity Enhanced Analyzer*
