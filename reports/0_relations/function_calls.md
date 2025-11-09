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

`drawingAccumulator()`** *(external)***

`emergencyWithdrawLP()`** *(external)***
   ├─> `_consolidateWithdrawals()` *(internal)*
   └─> `_consolidateDeposits()` *(internal)*

`getDrawingAccumulator()`** *(external)***

`getLPDrawingState()`** *(external)***

`getLPValueBreakdown()`** *(external)***

`getLpInfo()`** *(external)***

`initializeDrawingLP()`** *(external)***

`initializeLP()`** *(external)***

`jackpot()`** *(external)***

`lpInfo()`** *(external)***

`lpPoolCap()`** *(external)***

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

**From `claimWinnings()`** *(external)*:
   - → `usdc.balanceOf()`
   - → `jackpot.claimWinnings()` *[Jackpot]*
   - → `usdc.balanceOf()`

**From `buyTickets()`** *(external)*:
   - → `jackpot.ticketPrice()` *[Jackpot]*
   - → `jackpot.currentDrawingId()` *[Jackpot]*
   - → `usdc.safeTransferFrom()`
   - → `usdc.approve()`
   - → `jackpot.buyTickets()` *[Jackpot]*

<<<<<<< Updated upstream
=======
**From `_bridgeFunds()`** *(private)*:
   - → `usdc.approve()`
   - → `usdc.balanceOf()`
   - → `usdc.balanceOf()`

>>>>>>> Stashed changes
### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`CLAIM_TICKET_TYPEHASH()`** *(external)***

`CLAIM_WINNINGS_TYPEHASH()`** *(external)***

`RELAY_TYPEHASH()`** *(external)***

`buyTickets()`** *(external)***

`claimTickets()`** *(external)***
   ├─> `_updateTicketOwnership()` *(private)*
   └─> `_validateTicketOwnership()` *(private)*

`claimWinnings()`** *(external)***
   ├─> `_bridgeFunds()` *(private)*
   └─> `_validateTicketOwnership()` *(private)*

`constructor()`** *(public)***

`createClaimTicketEIP712Hash()`** *(public)***
   ├─> `keccak256()` *(unknown)*
   └─> `_hashTypedDataV4()` *(unknown)*

`createClaimWinningsEIP712Hash()`** *(public)***
   ├─> `keccak256()` *(unknown)*
   └─> `_hashTypedDataV4()` *(unknown)*

`getUserTickets()`** *(external)***

`jackpot()`** *(external)***

`jackpotTicketNFT()`** *(external)***

`ticketOwner()`** *(external)***

`usdc()`** *(external)***

`userTickets()`** *(external)***


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
   ├─> `_validateRequests()` *(internal)*
   └─> `_storePendingRequest()` *(internal)*

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

`jackpot()`** *(external)***

`mintTicket()`** *(external)***
   └─> `_mint()` *(unknown)*

`name()`** *(public)***

`symbol()`** *(public)***

`tickets()`** *(external)***

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

`PRECISE_UNIT()`** *(external)***

`calculateAndStoreDrawingUserWinnings()`** *(external)***

`constructor()`** *(public)***
   └─> `_setPremiumTierWeights()` *(internal)*

`drawingTierInfo()`** *(external)***

`getDrawingTierInfo()`** *(external)***

`getDrawingTierPayouts()`** *(external)***

`getMinPayoutTiers()`** *(external)***

`getPremiumTierWeights()`** *(external)***

`getTierPayout()`** *(external)***

`jackpot()`** *(external)***

`minPayoutTiers()`** *(external)***

`minimumPayout()`** *(external)***

`premiumTierMinAllocation()`** *(external)***

`premiumTierWeights()`** *(external)***

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

<<<<<<< Updated upstream
**From `getEntropyCallbackFee()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `_setNewDrawingState()`** *(internal)*:
   - → `jackpotLPManager.initializeDrawingLP()` *[JackpotLPManager]*
   - → `payoutCalculator.setDrawingTierInfo()`

**From `_validateAndStoreTickets()`** *(internal)*:
   - → `jackpotNFT.mintTicket()` *[JackpotTicketNFT]*
=======
**From `finalizeWithdraw()`** *(external)*:
   - → `jackpotLPManager.processFinalizeWithdraw()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `lpDeposit()`** *(external)*:
   - → `usdc.safeTransferFrom()`
   - → `jackpotLPManager.processDeposit()` *[JackpotLPManager]*

**From `runJackpot()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `emergencyWithdrawLP()`** *(external)*:
   - → `jackpotLPManager.emergencyWithdrawLP()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `claimReferralFees()`** *(external)*:
   - → `usdc.safeTransfer()`

**From `_validateAndStoreTickets()`** *(internal)*:
   - → `jackpotNFT.mintTicket()` *[JackpotTicketNFT]*

**From `emergencyRefundTickets()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
   - → `jackpotNFT.burnTicket()` *[JackpotTicketNFT]*
   - → `usdc.safeTransfer()`

**From `setLpEdgeTarget()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `setReserveRatio()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `setNormalBallMax()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*
>>>>>>> Stashed changes

**From `claimWinnings()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
   - → `jackpotNFT.burnTicket()` *[JackpotTicketNFT]*
   - → `payoutCalculator.getTierPayout()`
   - → `usdc.safeTransfer()`

<<<<<<< Updated upstream
**From `lpDeposit()`** *(external)*:
   - → `usdc.safeTransferFrom()`
   - → `jackpotLPManager.processDeposit()` *[JackpotLPManager]*
=======
**From `initiateWithdraw()`** *(external)*:
   - → `jackpotLPManager.processInitiateWithdraw()` *[JackpotLPManager]*

**From `_setNewDrawingState()`** *(internal)*:
   - → `jackpotLPManager.initializeDrawingLP()` *[JackpotLPManager]*
   - → `payoutCalculator.setDrawingTierInfo()`

**From `_transferProtocolFee()`** *(internal)*:
   - → `usdc.safeTransfer()`

**From `getTicketTierIds()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
>>>>>>> Stashed changes

**From `buyTickets()`** *(external)*:
   - → `usdc.safeTransferFrom()`

<<<<<<< Updated upstream
**From `setGovernancePoolCap()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

=======
>>>>>>> Stashed changes
**From `initializeLPDeposits()`** *(external)*:
   - → `jackpotLPManager.initializeLP()` *[JackpotLPManager]*
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `getEntropyCallbackFee()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `setTicketPrice()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

<<<<<<< Updated upstream
**From `runJackpot()`** *(external)*:
   - → `entropy.getFee()` *[ScaledEntropyProvider]*

**From `claimReferralFees()`** *(external)*:
   - → `usdc.safeTransfer()`

**From `_transferProtocolFee()`** *(internal)*:
   - → `usdc.safeTransfer()`

**From `setReserveRatio()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `emergencyWithdrawLP()`** *(external)*:
   - → `jackpotLPManager.emergencyWithdrawLP()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `getTicketTierIds()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*

**From `setLpEdgeTarget()`** *(external)*:
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

**From `initiateWithdraw()`** *(external)*:
   - → `jackpotLPManager.processInitiateWithdraw()` *[JackpotLPManager]*

**From `emergencyRefundTickets()`** *(external)*:
   - → `jackpotNFT.getTicketInfo()` *[JackpotTicketNFT]*
   - → `jackpotNFT.burnTicket()` *[JackpotTicketNFT]*
   - → `usdc.safeTransfer()`

**From `finalizeWithdraw()`** *(external)*:
   - → `jackpotLPManager.processFinalizeWithdraw()` *[JackpotLPManager]*
   - → `usdc.safeTransfer()`

**From `setTicketPrice()`** *(external)*:
=======
**From `setGovernancePoolCap()`** *(external)*:
>>>>>>> Stashed changes
   - → `jackpotLPManager.setLPPoolCap()` *[JackpotLPManager]*

### 🔗 Internal Call Graph

*Shows the hierarchy of internal function calls*

`allowTicketPurchases()`** *(external)***

`bonusballMin()`** *(external)***

`buyTickets()`** *(external)***
   └─> `_validateBuyTicketInputs()` *(internal)*

`checkIfTicketsBought()`** *(external)***

`claimReferralFees()`** *(external)***

`claimWinnings()`** *(external)***

`constructor()`** *(public)***

`currentDrawingId()`** *(external)***

`disableEmergencyMode()`** *(external)***

`disableTicketPurchases()`** *(external)***

`drawingDurationInSeconds()`** *(external)***

`emergencyMode()`** *(external)***

`emergencyRefundTickets()`** *(external)***

`emergencyWithdrawLP()`** *(external)***

`enableEmergencyMode()`** *(external)***

`enableTicketPurchases()`** *(external)***

`entropy()`** *(external)***

`entropyBaseGasLimit()`** *(external)***

`entropyVariableGasLimit()`** *(external)***

`finalizeWithdraw()`** *(external)***

`getDrawingState()`** *(external)***

`getEntropyCallbackFee()`** *(external)***

`getReferralScheme()`** *(external)***

`getSubsetCount()`** *(external)***

`getTicketTierIds()`** *(external)***

`getUnpackedTicket()`** *(external)***

`governancePoolCap()`** *(external)***

`initialize()`** *(external)***

`initializeJackpot()`** *(external)***
   └─> `_setNewDrawingState()` *(internal)*

`initializeLPDeposits()`** *(external)***
   └─> `_calculateLpPoolCap()` *(internal)*

`initialized()`** *(external)***

`initiateWithdraw()`** *(external)***

`jackpotLPManager()`** *(external)***

`jackpotNFT()`** *(external)***

`lockJackpot()`** *(external)***
   └─> `_lockJackpot()` *(internal)*

`lpDeposit()`** *(external)***

`lpEdgeTarget()`** *(external)***

`maxReferrers()`** *(external)***

`normalBallMax()`** *(external)***

`payoutCalculator()`** *(external)***

`protocolFee()`** *(external)***

`protocolFeeAddress()`** *(external)***

`protocolFeeThreshold()`** *(external)***

`referralFee()`** *(external)***

`referralFees()`** *(external)***

`referralWinShare()`** *(external)***

`reserveRatio()`** *(external)***

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

`ticketPrice()`** *(external)***

`unlockJackpot()`** *(external)***
   └─> `_unlockJackpot()` *(internal)*

`usdc()`** *(external)***

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
