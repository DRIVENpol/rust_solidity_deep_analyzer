# State Variable Access Report

**Shows which functions read and write each state variable, including call chains**

## Contract: JackpotLPManager

### State Variable: `PRECISE_UNIT`
**Type:** uint256
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (5 direct, 7 total via call chains):
   *Direct:*
      - `initializeLP`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `processDrawingSettlement`  *(external)*
      - `getLPValueBreakdown`  *(external)*
      - `_consolidateWithdrawals`  *(internal)*
   *Via call chains:*
      - `processInitiateWithdraw` *(external)* → _consolidateWithdrawals
      - `processFinalizeWithdraw` *(external)* → _consolidateWithdrawals

### State Variable: `lpDrawingState`
**Type:** mapping(uint256 => LPDrawingState)
**Visibility:** internal

**WRITERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `processDeposit`  *(external)*
      - `processInitiateWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `initializeDrawingLP`  *(external)*

**READERS** (6 direct, 6 total via call chains):
   *Direct:*
      - `processDeposit`  *(external)*
      - `processInitiateWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `processDrawingSettlement`  *(external)*
      - `setLPPoolCap`  *(external)*
      - `getLPDrawingState`  *(external)*

### State Variable: `lpInfo`
**Type:** mapping(address => LP)
**Visibility:** public

**WRITERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `processDeposit`  *(external)*
      - `processInitiateWithdraw`  *(external)*
      - `processFinalizeWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*

**READERS** (7 direct, 7 total via call chains):
   *Direct:*
      - `processDeposit`  *(external)*
      - `processInitiateWithdraw`  *(external)*
      - `processFinalizeWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `getLpInfo`  *(external)*
      - `getLPValueBreakdown`  *(external)*
      - `lpInfo`  *(external)*

### State Variable: `drawingAccumulator`
**Type:** mapping(uint256 => uint256)
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `initializeLP`  *(external)*
      - `processDrawingSettlement`  *(external)*

**READERS** (5 direct, 7 total via call chains):
   *Direct:*
      - `emergencyWithdrawLP`  *(external)*
      - `getDrawingAccumulator`  *(external)*
      - `getLPValueBreakdown`  *(external)*
      - `_consolidateDeposits`  *(internal)*
      - `drawingAccumulator`  *(external)*
   *Via call chains:*
      - `processDeposit` *(external)* → _consolidateDeposits
      - `processInitiateWithdraw` *(external)* → _consolidateDeposits

### State Variable: `lpPoolCap`
**Type:** uint256
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `setLPPoolCap`  *(external)*

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `processDeposit`  *(external)*
      - `lpPoolCap`  *(external)*

### State Variable: `jackpot`
**Type:** IJackpot
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `getLPValueBreakdown`  *(external)*
      - `jackpot`  *(external)*

## Contract: JackpotBridgeManager

### State Variable: `CLAIM_WINNINGS_TYPEHASH`
**Type:** bytes32
**Visibility:** public
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `createClaimWinningsEIP712Hash`  *(public)*
      - `CLAIM_WINNINGS_TYPEHASH`  *(external)*

### State Variable: `CLAIM_TICKET_TYPEHASH`
**Type:** bytes32
**Visibility:** public
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `createClaimTicketEIP712Hash`  *(public)*
      - `CLAIM_TICKET_TYPEHASH`  *(external)*

### State Variable: `RELAY_TYPEHASH`
**Type:** bytes32
**Visibility:** public
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `createClaimWinningsEIP712Hash`  *(public)*
      - `RELAY_TYPEHASH`  *(external)*

### State Variable: `userTickets`
**Type:** mapping(address => mapping(uint256 => UserTickets))
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `getUserTickets`  *(external)*
      - `userTickets`  *(external)*

### State Variable: `ticketOwner`
**Type:** mapping(uint256 => address)
**Visibility:** public

**WRITERS** (2 direct, 3 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `_updateTicketOwnership`  *(private)*
   *Via call chains:*
      - `claimTickets` *(external)* → _updateTicketOwnership

**READERS** (3 direct, 5 total via call chains):
   *Direct:*
      - `getUserTickets`  *(external)*
      - `_validateTicketOwnership`  *(private)*
      - `ticketOwner`  *(external)*
   *Via call chains:*
      - `claimWinnings` *(external)* → _validateTicketOwnership
      - `claimTickets` *(external)* → _validateTicketOwnership

### State Variable: `jackpot`
**Type:** IJackpot
**Visibility:** public
**Immutable:** yes

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `claimWinnings`  *(external)*
      - `jackpot`  *(external)*

### State Variable: `jackpotTicketNFT`
**Type:** IJackpotTicketNFT
**Visibility:** public
**Immutable:** yes

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `jackpotTicketNFT`  *(external)*

### State Variable: `usdc`
**Type:** IERC20
**Visibility:** public
**Immutable:** yes

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `claimWinnings`  *(external)*
      - `_bridgeFunds`  *(private)*
      - `usdc`  *(external)*

## Contract: ScaledEntropyProvider

### State Variable: `entropy`
**Type:** IEntropyV2
**Visibility:** private

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `getFee`  *(public)*
      - `getEntropyContract`  *(external)*
      - `getEntropy`  *(internal)*

### State Variable: `entropyProvider`
**Type:** address
**Visibility:** private

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setEntropyProvider`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `requestAndCallbackScaledRandomness`  *(external)*
      - `getFee`  *(public)*
      - `getEntropyProvider`  *(external)*

### State Variable: `pending`
**Type:** mapping(uint64 => PendingRequest)
**Visibility:** private

**WRITERS** (2 direct, 3 total via call chains):
   *Direct:*
      - `entropyCallback`  *(internal)*
      - `_storePendingRequest`  *(internal)*
   *Via call chains:*
      - `requestAndCallbackScaledRandomness` *(external)* → _storePendingRequest

**READERS** (3 direct, 4 total via call chains):
   *Direct:*
      - `getPendingRequest`  *(external)*
      - `entropyCallback`  *(internal)*
      - `_storePendingRequest`  *(internal)*
   *Via call chains:*
      - `requestAndCallbackScaledRandomness` *(external)* → _storePendingRequest

## Contract: JackpotTicketNFT

### State Variable: `userTickets`
**Type:** mapping(address => mapping(uint256 => UserTickets))
**Visibility:** internal

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `_beforeTokenTransfer`  *(internal)*
      - `_afterTokenTransfer`  *(internal)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `getUserTickets`  *(external)*
      - `_beforeTokenTransfer`  *(internal)*
      - `_afterTokenTransfer`  *(internal)*

### State Variable: `tickets`
**Type:** mapping(uint256 => TrackedTicket)
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `mintTicket`  *(external)*

**READERS** (5 direct, 6 total via call chains):
   *Direct:*
      - `getTicketInfo`  *(external)*
      - `_beforeTokenTransfer`  *(internal)*
      - `_afterTokenTransfer`  *(internal)*
      - `_getExtendedTicketInfo`  *(internal)*
      - `tickets`  *(external)*
   *Via call chains:*
      - `getExtendedTicketInfo` *(external)* → _getExtendedTicketInfo

### State Variable: `jackpot`
**Type:** IJackpot
**Visibility:** public
**Immutable:** yes

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (2 direct, 3 total via call chains):
   *Direct:*
      - `_getExtendedTicketInfo`  *(internal)*
      - `jackpot`  *(external)*
   *Via call chains:*
      - `getExtendedTicketInfo` *(external)* → _getExtendedTicketInfo

## Contract: GuaranteedMinimumPayoutCalculator

### State Variable: `PRECISE_UNIT`
**Type:** uint256
**Visibility:** public
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (4 direct, 5 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setPremiumTierMinAllocation`  *(external)*
      - `_setPremiumTierWeights`  *(internal)*
      - `PRECISE_UNIT`  *(external)*
   *Via call chains:*
      - `setPremiumTierWeights` *(external)* → _setPremiumTierWeights

### State Variable: `NORMAL_BALL_COUNT`
**Type:** uint8
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `_calculateTierTotalWinningCombos`  *(internal)*

### State Variable: `TOTAL_TIER_COUNT`
**Type:** uint8
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (4 direct, 6 total via call chains):
   *Direct:*
      - `calculateAndStoreDrawingUserWinnings`  *(external)*
      - `getDrawingTierPayouts`  *(external)*
      - `_setPremiumTierWeights`  *(internal)*
      - `_calculateAndStoreTierPayouts`  *(internal)*
   *Via call chains:*
      - `constructor` *(public)* → _setPremiumTierWeights
      - `setPremiumTierWeights` *(external)* → _setPremiumTierWeights

### State Variable: `drawingTierInfo`
**Type:** mapping(uint256 => DrawingTierInfo)
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `setDrawingTierInfo`  *(external)*

**READERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `calculateAndStoreDrawingUserWinnings`  *(external)*
      - `getDrawingTierInfo`  *(external)*
      - `_calculateAndStoreTierPayouts`  *(internal)*
      - `drawingTierInfo`  *(external)*

### State Variable: `tierPayouts`
**Type:** mapping(uint256 => mapping(uint256 => uint256))
**Visibility:** internal

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `_calculateAndStoreTierPayouts`  *(internal)*

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `getTierPayout`  *(external)*
      - `getDrawingTierPayouts`  *(external)*

### State Variable: `premiumTierWeights`
**Type:** uint256[TOTAL_TIER_COUNT]
**Visibility:** public

**WRITERS** (1 direct, 3 total via call chains):
   *Direct:*
      - `_setPremiumTierWeights`  *(internal)*
   *Via call chains:*
      - `constructor` *(public)* → _setPremiumTierWeights
      - `setPremiumTierWeights` *(external)* → _setPremiumTierWeights

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `getPremiumTierWeights`  *(external)*
      - `premiumTierWeights`  *(external)*

### State Variable: `minPayoutTiers`
**Type:** bool[TOTAL_TIER_COUNT]
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setMinPayoutTiers`  *(external)*

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `getMinPayoutTiers`  *(external)*
      - `minPayoutTiers`  *(external)*

### State Variable: `minimumPayout`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setMinimumPayout`  *(external)*

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `minimumPayout`  *(external)*

### State Variable: `premiumTierMinAllocation`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setPremiumTierMinAllocation`  *(external)*

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `premiumTierMinAllocation`  *(external)*

### State Variable: `jackpot`
**Type:** IJackpot
**Visibility:** public
**Immutable:** yes

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `constructor`  *(public)*

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `jackpot`  *(external)*

## Contract: Jackpot

### State Variable: `PRECISE_UNIT`
**Type:** uint256
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (10 direct, 16 total via call chains):
   *Direct:*
      - `emergencyRefundTickets`  *(external)*
      - `setLpEdgeTarget`  *(external)*
      - `setReserveRatio`  *(external)*
      - `setReferralFee`  *(external)*
      - `setReferralWinShare`  *(external)*
      - `_calculateLpPoolCap`  *(internal)*
      - `_setNewDrawingState`  *(internal)*
      - `_validateAndTrackReferrals`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*
      - `_transferProtocolFee`  *(internal)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeLPDeposits` *(external)* → _calculateLpPoolCap
      - `initializeJackpot` *(external)* → _setNewDrawingState
      - `setNormalBallMax` *(external)* → _calculateLpPoolCap
      - `setGovernancePoolCap` *(external)* → _calculateLpPoolCap
      - `setTicketPrice` *(external)* → _calculateLpPoolCap

### State Variable: `NORMAL_BALL_COUNT`
**Type:** uint8
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (3 direct, 11 total via call chains):
   *Direct:*
      - `_calculateLpPoolCap`  *(internal)*
      - `_setNewDrawingState`  *(internal)*
      - `_validateAndStoreTickets`  *(internal)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeLPDeposits` *(external)* → _calculateLpPoolCap
      - `initializeJackpot` *(external)* → _setNewDrawingState
      - `setNormalBallMax` *(external)* → _calculateLpPoolCap
      - `setGovernancePoolCap` *(external)* → _calculateLpPoolCap
      - `setLpEdgeTarget` *(external)* → _calculateLpPoolCap
      - `setReserveRatio` *(external)* → _calculateLpPoolCap
      - `setTicketPrice` *(external)* → _calculateLpPoolCap

### State Variable: `MAX_BIT_VECTOR_SIZE`
**Type:** uint8
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (0 direct, 0 total via call chains):
   - None (write-only or unused)

### State Variable: `MAX_PROTOCOL_FEE`
**Type:** uint256
**Visibility:** internal
**Constant:** yes

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `setProtocolFee`  *(external)*

### State Variable: `drawingEntries`
**Type:** mapping(uint256 => TicketComboTracker.Tracker)
**Visibility:** internal

**WRITERS** (0 direct, 0 total via call chains):
   - None (read-only or constant)

**READERS** (5 direct, 7 total via call chains):
   *Direct:*
      - `checkIfTicketsBought`  *(external)*
      - `getSubsetCount`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `_validateAndStoreTickets`  *(internal)*
      - `_calculateDrawingUserWinnings`  *(internal)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `drawingState`
**Type:** mapping(uint256 => DrawingState)
**Visibility:** internal

**WRITERS** (6 direct, 10 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `scaledEntropyCallback`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*
      - `_lockJackpot`  *(internal)*
      - `_unlockJackpot`  *(internal)*
   *Via call chains:*
      - `runJackpot` *(external)* → _lockJackpot
      - `initializeJackpot` *(external)* → _setNewDrawingState
      - `lockJackpot` *(external)* → _lockJackpot
      - `unlockJackpot` *(external)* → _unlockJackpot

**READERS** (17 direct, 18 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `claimWinnings`  *(external)*
      - `lpDeposit`  *(external)*
      - `initiateWithdraw`  *(external)*
      - `emergencyRefundTickets`  *(external)*
      - `runJackpot`  *(external)*
      - `scaledEntropyCallback`  *(external)*
      - `lockJackpot`  *(external)*
      - `unlockJackpot`  *(external)*
      - `getDrawingState`  *(external)*
      - `getSubsetCount`  *(external)*
      - `getUnpackedTicket`  *(external)*
      - `getTicketTierIds`  *(external)*
      - `getEntropyCallbackFee`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `_validateBuyTicketInputs`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*
   *Via call chains:*
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `referralFees`
**Type:** mapping(address => uint256)
**Visibility:** public

**WRITERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `claimReferralFees`  *(external)*
      - `_validateAndTrackReferrals`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*

**READERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `claimReferralFees`  *(external)*
      - `_validateAndTrackReferrals`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*
      - `referralFees`  *(external)*

### State Variable: `referralSchemes`
**Type:** mapping(bytes32 => ReferralScheme)
**Visibility:** internal

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `_validateAndTrackReferrals`  *(internal)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `getReferralScheme`  *(external)*
      - `_validateAndTrackReferrals`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*

### State Variable: `currentDrawingId`
**Type:** uint256
**Visibility:** public

**WRITERS** (1 direct, 3 total via call chains):
   *Direct:*
      - `_setNewDrawingState`  *(internal)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

**READERS** (40 direct, 42 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `claimWinnings`  *(external)*
      - `lpDeposit`  *(external)*
      - `initiateWithdraw`  *(external)*
      - `finalizeWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `emergencyRefundTickets`  *(external)*
      - `runJackpot`  *(external)*
      - `scaledEntropyCallback`  *(external)*
      - `initializeLPDeposits`  *(external)*
      - `initializeJackpot`  *(external)*
      - `setNormalBallMax`  *(external)*
      - `setProtocolFeeThreshold`  *(external)*
      - `setProtocolFee`  *(external)*
      - `setGovernancePoolCap`  *(external)*
      - `setDrawingDurationInSeconds`  *(external)*
      - `setBonusballMin`  *(external)*
      - `setLpEdgeTarget`  *(external)*
      - `setReserveRatio`  *(external)*
      - `setReferralFee`  *(external)*
      - `setReferralWinShare`  *(external)*
      - `setProtocolFeeAddress`  *(external)*
      - `setTicketPrice`  *(external)*
      - `setMaxReferrers`  *(external)*
      - `setPayoutCalculator`  *(external)*
      - `setEntropy`  *(external)*
      - `setEntropyBaseGasLimit`  *(external)*
      - `setEntropyVariableGasLimit`  *(external)*
      - `enableEmergencyMode`  *(external)*
      - `disableEmergencyMode`  *(external)*
      - `enableTicketPurchases`  *(external)*
      - `disableTicketPurchases`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `_validateAndStoreTickets`  *(internal)*
      - `_calculateDrawingUserWinnings`  *(internal)*
      - `_payReferrersWinnings`  *(internal)*
      - `_transferProtocolFee`  *(internal)*
      - `_lockJackpot`  *(internal)*
      - `_unlockJackpot`  *(internal)*
      - `currentDrawingId`  *(external)*
   *Via call chains:*
      - `lockJackpot` *(external)* → _lockJackpot
      - `unlockJackpot` *(external)* → _unlockJackpot

### State Variable: `ticketPrice`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setTicketPrice`  *(external)*

**READERS** (4 direct, 11 total via call chains):
   *Direct:*
      - `setTicketPrice`  *(external)*
      - `_calculateLpPoolCap`  *(internal)*
      - `_setNewDrawingState`  *(internal)*
      - `ticketPrice`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeLPDeposits` *(external)* → _calculateLpPoolCap
      - `initializeJackpot` *(external)* → _setNewDrawingState
      - `setNormalBallMax` *(external)* → _calculateLpPoolCap
      - `setGovernancePoolCap` *(external)* → _calculateLpPoolCap
      - `setLpEdgeTarget` *(external)* → _calculateLpPoolCap
      - `setReserveRatio` *(external)* → _calculateLpPoolCap

### State Variable: `normalBallMax`
**Type:** uint8
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setNormalBallMax`  *(external)*

**READERS** (8 direct, 10 total via call chains):
   *Direct:*
      - `initializeLPDeposits`  *(external)*
      - `setNormalBallMax`  *(external)*
      - `setGovernancePoolCap`  *(external)*
      - `setLpEdgeTarget`  *(external)*
      - `setReserveRatio`  *(external)*
      - `setTicketPrice`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `normalBallMax`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `bonusballMin`
**Type:** uint8
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setBonusballMin`  *(external)*

**READERS** (3 direct, 5 total via call chains):
   *Direct:*
      - `setBonusballMin`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `bonusballMin`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `drawingDurationInSeconds`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setDrawingDurationInSeconds`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `scaledEntropyCallback`  *(external)*
      - `setDrawingDurationInSeconds`  *(external)*
      - `drawingDurationInSeconds`  *(external)*

### State Variable: `reserveRatio`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setReserveRatio`  *(external)*

**READERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `setReserveRatio`  *(external)*
      - `reserveRatio`  *(external)*

### State Variable: `lpEdgeTarget`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setLpEdgeTarget`  *(external)*

**READERS** (3 direct, 5 total via call chains):
   *Direct:*
      - `setLpEdgeTarget`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `lpEdgeTarget`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `governancePoolCap`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `initializeLPDeposits`  *(external)*
      - `setGovernancePoolCap`  *(external)*

**READERS** (3 direct, 8 total via call chains):
   *Direct:*
      - `setGovernancePoolCap`  *(external)*
      - `_calculateLpPoolCap`  *(internal)*
      - `governancePoolCap`  *(external)*
   *Via call chains:*
      - `initializeLPDeposits` *(external)* → _calculateLpPoolCap
      - `setNormalBallMax` *(external)* → _calculateLpPoolCap
      - `setLpEdgeTarget` *(external)* → _calculateLpPoolCap
      - `setReserveRatio` *(external)* → _calculateLpPoolCap
      - `setTicketPrice` *(external)* → _calculateLpPoolCap

### State Variable: `referralFee`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setReferralFee`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setReferralFee`  *(external)*
      - `_validateAndTrackReferrals`  *(internal)*
      - `referralFee`  *(external)*

### State Variable: `referralWinShare`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setReferralWinShare`  *(external)*

**READERS** (3 direct, 5 total via call chains):
   *Direct:*
      - `setReferralWinShare`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `referralWinShare`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState

### State Variable: `protocolFee`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setProtocolFee`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setProtocolFee`  *(external)*
      - `_transferProtocolFee`  *(internal)*
      - `protocolFee`  *(external)*

### State Variable: `protocolFeeThreshold`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setProtocolFeeThreshold`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setProtocolFeeThreshold`  *(external)*
      - `_transferProtocolFee`  *(internal)*
      - `protocolFeeThreshold`  *(external)*

### State Variable: `protocolFeeAddress`
**Type:** address
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setProtocolFeeAddress`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setProtocolFeeAddress`  *(external)*
      - `_transferProtocolFee`  *(internal)*
      - `protocolFeeAddress`  *(external)*

### State Variable: `maxReferrers`
**Type:** uint256
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setMaxReferrers`  *(external)*

**READERS** (3 direct, 4 total via call chains):
   *Direct:*
      - `setMaxReferrers`  *(external)*
      - `_validateBuyTicketInputs`  *(internal)*
      - `maxReferrers`  *(external)*
   *Via call chains:*
      - `buyTickets` *(external)* → _validateBuyTicketInputs

### State Variable: `initialized`
**Type:** bool
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `initialize`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `initialize`  *(external)*
      - `initializeLPDeposits`  *(external)*
      - `initialized`  *(external)*

### State Variable: `allowTicketPurchases`
**Type:** bool
**Visibility:** public

**WRITERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `initializeJackpot`  *(external)*
      - `enableTicketPurchases`  *(external)*
      - `disableTicketPurchases`  *(external)*

**READERS** (4 direct, 5 total via call chains):
   *Direct:*
      - `enableTicketPurchases`  *(external)*
      - `disableTicketPurchases`  *(external)*
      - `_validateBuyTicketInputs`  *(internal)*
      - `allowTicketPurchases`  *(external)*
   *Via call chains:*
      - `buyTickets` *(external)* → _validateBuyTicketInputs

### State Variable: `emergencyMode`
**Type:** bool
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `enableEmergencyMode`  *(external)*
      - `disableEmergencyMode`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `enableEmergencyMode`  *(external)*
      - `disableEmergencyMode`  *(external)*
      - `emergencyMode`  *(external)*

### State Variable: `entropyBaseGasLimit`
**Type:** uint32
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setEntropyBaseGasLimit`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setEntropyBaseGasLimit`  *(external)*
      - `_calculateEntropyGasLimit`  *(internal)*
      - `entropyBaseGasLimit`  *(external)*

### State Variable: `entropyVariableGasLimit`
**Type:** uint32
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `constructor`  *(public)*
      - `setEntropyVariableGasLimit`  *(external)*

**READERS** (3 direct, 3 total via call chains):
   *Direct:*
      - `setEntropyVariableGasLimit`  *(external)*
      - `_calculateEntropyGasLimit`  *(internal)*
      - `entropyVariableGasLimit`  *(external)*

### State Variable: `usdc`
**Type:** IERC20
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `initialize`  *(external)*

**READERS** (9 direct, 9 total via call chains):
   *Direct:*
      - `buyTickets`  *(external)*
      - `claimWinnings`  *(external)*
      - `lpDeposit`  *(external)*
      - `finalizeWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `emergencyRefundTickets`  *(external)*
      - `claimReferralFees`  *(external)*
      - `_transferProtocolFee`  *(internal)*
      - `usdc`  *(external)*

### State Variable: `jackpotLPManager`
**Type:** IJackpotLPManager
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `initialize`  *(external)*

**READERS** (14 direct, 14 total via call chains):
   *Direct:*
      - `lpDeposit`  *(external)*
      - `initiateWithdraw`  *(external)*
      - `finalizeWithdraw`  *(external)*
      - `emergencyWithdrawLP`  *(external)*
      - `scaledEntropyCallback`  *(external)*
      - `initializeLPDeposits`  *(external)*
      - `initializeJackpot`  *(external)*
      - `setNormalBallMax`  *(external)*
      - `setGovernancePoolCap`  *(external)*
      - `setLpEdgeTarget`  *(external)*
      - `setReserveRatio`  *(external)*
      - `setTicketPrice`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `jackpotLPManager`  *(external)*

### State Variable: `jackpotNFT`
**Type:** IJackpotTicketNFT
**Visibility:** public

**WRITERS** (1 direct, 1 total via call chains):
   *Direct:*
      - `initialize`  *(external)*

**READERS** (5 direct, 5 total via call chains):
   *Direct:*
      - `claimWinnings`  *(external)*
      - `emergencyRefundTickets`  *(external)*
      - `getTicketTierIds`  *(external)*
      - `_validateAndStoreTickets`  *(internal)*
      - `jackpotNFT`  *(external)*

### State Variable: `entropy`
**Type:** IScaledEntropyProvider
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `initialize`  *(external)*
      - `setEntropy`  *(external)*

**READERS** (4 direct, 4 total via call chains):
   *Direct:*
      - `runJackpot`  *(external)*
      - `setEntropy`  *(external)*
      - `getEntropyCallbackFee`  *(external)*
      - `entropy`  *(external)*

### State Variable: `payoutCalculator`
**Type:** IPayoutCalculator
**Visibility:** public

**WRITERS** (2 direct, 2 total via call chains):
   *Direct:*
      - `initialize`  *(external)*
      - `setPayoutCalculator`  *(external)*

**READERS** (5 direct, 7 total via call chains):
   *Direct:*
      - `claimWinnings`  *(external)*
      - `setPayoutCalculator`  *(external)*
      - `_setNewDrawingState`  *(internal)*
      - `_calculateDrawingUserWinnings`  *(internal)*
      - `payoutCalculator`  *(external)*
   *Via call chains:*
      - `scaledEntropyCallback` *(external)* → _setNewDrawingState
      - `initializeJackpot` *(external)* → _setNewDrawingState


---

*Generated by MainnetReady - Solidity Enhanced Analyzer*
