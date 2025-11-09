════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `Jackpot`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/Jackpot.sol`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**ANALYSIS SUMMARY**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 **Contract Metrics:**
   • Functions: 56 (44 public/external entry points)
   • State Variables: 32 (28 mutable)
   • Events: 36
   • Modifiers: 3
   • Custom Errors: 46

🔒 **Security Findings:**
   • 🔴 8 HIGH/CRITICAL severity issue(s)
   • Total: 101 finding(s) detected

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**NOTE:** Call chains show all potential modification paths through static analysis.
Functions may only modify fields conditionally based on runtime values.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**STATE VARIABLES**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`PRECISE_UNIT`**
   **Type:** `uint256`
   **Visibility:** internal, constant
   **Read by:**
      ├─ `emergencyRefundTickets` *(external)*
      ├─ `setLpEdgeTarget` *(external)*
      ├─ `setReserveRatio` *(external)*
      ├─ `setReferralFee` *(external)*
      ├─ `setReferralWinShare` *(external)*
      ├─ `_calculateLpPoolCap` *(internal)* ← `initializeLPDeposits` *(external)* ← `setNormalBallMax` *(external)* ← `setGovernancePoolCap` *(external)* ← `setLpEdgeTarget` *(external)* ← `setReserveRatio` *(external)* ← `setTicketPrice` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      ├─ `_validateAndTrackReferrals` *(internal)*
      ├─ `_payReferrersWinnings` *(internal)*
      └─ `_transferProtocolFee` *(internal)*


**`NORMAL_BALL_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant
   **Read by:**
      ├─ `_calculateLpPoolCap` *(internal)* ← `initializeLPDeposits` *(external)* ← `setNormalBallMax` *(external)* ← `setGovernancePoolCap` *(external)* ← `setLpEdgeTarget` *(external)* ← `setReserveRatio` *(external)* ← `setTicketPrice` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      └─ `_validateAndStoreTickets` *(internal)*


**`MAX_BIT_VECTOR_SIZE`**
   **Type:** `uint8`
   **Visibility:** internal, constant

**`MAX_PROTOCOL_FEE`**
   **Type:** `uint256`
   **Visibility:** internal, constant
   **Read by:**
      └─ `setProtocolFee` *(external)*


**`drawingEntries`**
   **Type:** `mapping(uint256 => TicketComboTracker.Tracker)`
   **Visibility:** internal

   **Modified by:** *None*

   **Read by:**
      ├─ `checkIfTicketsBought` *(external)*
      ├─ `getSubsetCount` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      ├─ `_validateAndStoreTickets` *(internal)*
      └─ `_calculateDrawingUserWinnings` *(internal)*


**`drawingState`**
   **Type:** `mapping(uint256 => DrawingState)`
   **Visibility:** internal

   **Modified by:**
      ├─ `buyTickets` *(external)*
      ├─ `scaledEntropyCallback` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      ├─ `_payReferrersWinnings` *(internal)*
      ├─ `_lockJackpot` *(internal)* ← `runJackpot` *(external)* ← `lockJackpot` *(external)*
      └─ `_unlockJackpot` *(internal)* ← `unlockJackpot` *(external)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `claimWinnings` *(external)*
      ├─ `lpDeposit` *(external)*
      ├─ `initiateWithdraw` *(external)*
      ├─ `emergencyRefundTickets` *(external)*
      ├─ `runJackpot` *(external)*
      ├─ `scaledEntropyCallback` *(external)*
      ├─ `lockJackpot` *(external)*
      ├─ `unlockJackpot` *(external)*
      ├─ `getDrawingState` *(external)*
      ├─ `getSubsetCount` *(external)*
      ├─ `getUnpackedTicket` *(external)*
      ├─ `getTicketTierIds` *(external)*
      ├─ `getEntropyCallbackFee` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      ├─ `_validateBuyTicketInputs` *(internal)* ← `buyTickets` *(external)*
      └─ `_payReferrersWinnings` *(internal)*


**`referralFees`**
   **Type:** `mapping(address => uint256)`
   **Visibility:** public

   **Modified by:**
      ├─ `claimReferralFees` *(external)*
      ├─ `_validateAndTrackReferrals` *(internal)*
      └─ `_payReferrersWinnings` *(internal)*

   **Read by:**
      ├─ `claimReferralFees` *(external)*
      ├─ `_validateAndTrackReferrals` *(internal)*
      └─ `_payReferrersWinnings` *(internal)*


**`referralSchemes`**
   **Type:** `mapping(bytes32 => ReferralScheme)`
   **Visibility:** internal

   **Modified by:**
      └─ `_validateAndTrackReferrals` *(internal)*

   **Read by:**
      ├─ `getReferralScheme` *(external)*
      ├─ `_validateAndTrackReferrals` *(internal)*
      └─ `_payReferrersWinnings` *(internal)*


**`currentDrawingId`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `claimWinnings` *(external)*
      ├─ `lpDeposit` *(external)*
      ├─ `initiateWithdraw` *(external)*
      ├─ `finalizeWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `emergencyRefundTickets` *(external)*
      ├─ `runJackpot` *(external)*
      ├─ `scaledEntropyCallback` *(external)*
      ├─ `initializeLPDeposits` *(external)*
      ├─ `initializeJackpot` *(external)*
      ├─ `setNormalBallMax` *(external)*
      ├─ `setProtocolFeeThreshold` *(external)*
      ├─ `setProtocolFee` *(external)*
      ├─ `setGovernancePoolCap` *(external)*
      ├─ `setDrawingDurationInSeconds` *(external)*
      ├─ `setBonusballMin` *(external)*
      ├─ `setLpEdgeTarget` *(external)*
      ├─ `setReserveRatio` *(external)*
      ├─ `setReferralFee` *(external)*
      ├─ `setReferralWinShare` *(external)*
      ├─ `setProtocolFeeAddress` *(external)*
      ├─ `setTicketPrice` *(external)*
      ├─ `setMaxReferrers` *(external)*
      ├─ `setPayoutCalculator` *(external)*
      ├─ `setEntropy` *(external)*
      ├─ `setEntropyBaseGasLimit` *(external)*
      ├─ `setEntropyVariableGasLimit` *(external)*
      ├─ `enableEmergencyMode` *(external)*
      ├─ `disableEmergencyMode` *(external)*
      ├─ `enableTicketPurchases` *(external)*
      ├─ `disableTicketPurchases` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      ├─ `_validateAndStoreTickets` *(internal)*
      ├─ `_calculateDrawingUserWinnings` *(internal)*
      ├─ `_payReferrersWinnings` *(internal)*
      ├─ `_transferProtocolFee` *(internal)*
      ├─ `_lockJackpot` *(internal)* ← `runJackpot` *(external)* ← `lockJackpot` *(external)*
      └─ `_unlockJackpot` *(internal)* ← `unlockJackpot` *(external)*


**`ticketPrice`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setTicketPrice` *(external)*

   **Read by:**
      ├─ `setTicketPrice` *(external)*
      ├─ `_calculateLpPoolCap` *(internal)* ← `initializeLPDeposits` *(external)* ← `setNormalBallMax` *(external)* ← `setGovernancePoolCap` *(external)* ← `setLpEdgeTarget` *(external)* ← `setReserveRatio` *(external)* ← `setTicketPrice` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`normalBallMax`**
   **Type:** `uint8`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setNormalBallMax` *(external)*

   **Read by:**
      ├─ `initializeLPDeposits` *(external)*
      ├─ `setNormalBallMax` *(external)*
      ├─ `setGovernancePoolCap` *(external)*
      ├─ `setLpEdgeTarget` *(external)*
      ├─ `setReserveRatio` *(external)*
      ├─ `setTicketPrice` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`bonusballMin`**
   **Type:** `uint8`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setBonusballMin` *(external)*

   **Read by:**
      ├─ `setBonusballMin` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`drawingDurationInSeconds`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setDrawingDurationInSeconds` *(external)*

   **Read by:**
      ├─ `scaledEntropyCallback` *(external)*
      └─ `setDrawingDurationInSeconds` *(external)*


**`reserveRatio`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReserveRatio` *(external)*

   **Read by:**
      └─ `setReserveRatio` *(external)*


**`lpEdgeTarget`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setLpEdgeTarget` *(external)*

   **Read by:**
      ├─ `setLpEdgeTarget` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`governancePoolCap`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeLPDeposits` *(external)*
      └─ `setGovernancePoolCap` *(external)*

   **Read by:**
      ├─ `setGovernancePoolCap` *(external)*
      └─ `_calculateLpPoolCap` *(internal)* ← `initializeLPDeposits` *(external)* ← `setNormalBallMax` *(external)* ← `setGovernancePoolCap` *(external)* ← `setLpEdgeTarget` *(external)* ← `setReserveRatio` *(external)* ← `setTicketPrice` *(external)*


**`referralFee`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReferralFee` *(external)*

   **Read by:**
      ├─ `setReferralFee` *(external)*
      └─ `_validateAndTrackReferrals` *(internal)*


**`referralWinShare`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReferralWinShare` *(external)*

   **Read by:**
      ├─ `setReferralWinShare` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`protocolFee`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFee` *(external)*

   **Read by:**
      ├─ `setProtocolFee` *(external)*
      └─ `_transferProtocolFee` *(internal)*


**`protocolFeeThreshold`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFeeThreshold` *(external)*

   **Read by:**
      ├─ `setProtocolFeeThreshold` *(external)*
      └─ `_transferProtocolFee` *(internal)*


**`protocolFeeAddress`**
   **Type:** `address`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFeeAddress` *(external)*

   **Read by:**
      ├─ `setProtocolFeeAddress` *(external)*
      └─ `_transferProtocolFee` *(internal)*


**`maxReferrers`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setMaxReferrers` *(external)*

   **Read by:**
      ├─ `setMaxReferrers` *(external)*
      └─ `_validateBuyTicketInputs` *(internal)* ← `buyTickets` *(external)*


**`initialized`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*

   **Read by:**
      ├─ `initialize` *(external)*
      └─ `initializeLPDeposits` *(external)*


**`allowTicketPurchases`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeJackpot` *(external)*
      ├─ `enableTicketPurchases` *(external)*
      └─ `disableTicketPurchases` *(external)*

   **Read by:**
      ├─ `enableTicketPurchases` *(external)*
      ├─ `disableTicketPurchases` *(external)*
      └─ `_validateBuyTicketInputs` *(internal)* ← `buyTickets` *(external)*


**`emergencyMode`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      ├─ `enableEmergencyMode` *(external)*
      └─ `disableEmergencyMode` *(external)*

   **Read by:**
      ├─ `enableEmergencyMode` *(external)*
      └─ `disableEmergencyMode` *(external)*


**`entropyBaseGasLimit`**
   **Type:** `uint32`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setEntropyBaseGasLimit` *(external)*

   **Read by:**
      ├─ `setEntropyBaseGasLimit` *(external)*
      └─ `_calculateEntropyGasLimit` *(internal)*


**`entropyVariableGasLimit`**
   **Type:** `uint32`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setEntropyVariableGasLimit` *(external)*

   **Read by:**
      ├─ `setEntropyVariableGasLimit` *(external)*
      └─ `_calculateEntropyGasLimit` *(internal)*


**`usdc`**
   **Type:** `IERC20`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*

   **Read by:**
      ├─ `buyTickets` *(external)*
      ├─ `claimWinnings` *(external)*
      ├─ `lpDeposit` *(external)*
      ├─ `finalizeWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `emergencyRefundTickets` *(external)*
      ├─ `claimReferralFees` *(external)*
      └─ `_transferProtocolFee` *(internal)*


**`jackpotLPManager`**
   **Type:** `IJackpotLPManager`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*

   **Read by:**
      ├─ `lpDeposit` *(external)*
      ├─ `initiateWithdraw` *(external)*
      ├─ `finalizeWithdraw` *(external)*
      ├─ `emergencyWithdrawLP` *(external)*
      ├─ `scaledEntropyCallback` *(external)*
      ├─ `initializeLPDeposits` *(external)*
      ├─ `initializeJackpot` *(external)*
      ├─ `setNormalBallMax` *(external)*
      ├─ `setGovernancePoolCap` *(external)*
      ├─ `setLpEdgeTarget` *(external)*
      ├─ `setReserveRatio` *(external)*
      ├─ `setTicketPrice` *(external)*
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`jackpotNFT`**
   **Type:** `IJackpotTicketNFT`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*

   **Read by:**
      ├─ `claimWinnings` *(external)*
      ├─ `emergencyRefundTickets` *(external)*
      ├─ `getTicketTierIds` *(external)*
      └─ `_validateAndStoreTickets` *(internal)*


**`entropy`**
   **Type:** `IScaledEntropyProvider`
   **Visibility:** public

   **Modified by:**
      ├─ `initialize` *(external)*
      └─ `setEntropy` *(external)*

   **Read by:**
      ├─ `runJackpot` *(external)*
      ├─ `setEntropy` *(external)*
      └─ `getEntropyCallbackFee` *(external)*


**`payoutCalculator`**
   **Type:** `IPayoutCalculator`
   **Visibility:** public

   **Modified by:**
      ├─ `initialize` *(external)*
      └─ `setPayoutCalculator` *(external)*

   **Read by:**
      ├─ `claimWinnings` *(external)*
      ├─ `setPayoutCalculator` *(external)*
      ├─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*
      └─ `_calculateDrawingUserWinnings` *(internal)*


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**EVENTS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`TicketOrderProcessed`**
   **Parameters:** `address` buyer *(indexed)*, `address` recipient *(indexed)*, `uint256` currentDrawingId *(indexed)*, `uint256` numberOfTickets, `uint256` lpEarnings, `uint256` referralFees

   **Emitted in:**
      └─ `buyTickets`


**`TicketPurchased`**
   **Parameters:** `address` recipient *(indexed)*, `uint256` currentDrawingId *(indexed)*, `bytes32` source *(indexed)*, `uint256` userTicketId, `uint8[]` normals, `uint8` bonusball, `bytes32` referralScheme

   **Emitted in:**
      └─ `_validateAndStoreTickets`


**`ReferralFeeCollected`**
   **Parameters:** `address` referrer *(indexed)*, `uint256` amount

   **Emitted in:**
      ├─ `_validateAndTrackReferrals`
      └─ `_payReferrersWinnings`


**`ReferralSchemeAdded`**
   **Parameters:** `bytes32` referralSchemeId *(indexed)*, `address[]` referrers, `uint256[]` referralSplit

   **Emitted in:**
      └─ `_validateAndTrackReferrals`


**`TicketWinningsClaimed`**
   **Parameters:** `address` userAddress *(indexed)*, `uint256` drawingId *(indexed)*, `uint256` userTicketId, `uint256` matchedNormals, `bool` bonusballMatch, `uint256` winningsAmount

   **Emitted in:**
      └─ `claimWinnings`


**`TicketRefunded`**
   **Parameters:** `uint256` ticketId *(indexed)*

   **Emitted in:**
      └─ `emergencyRefundTickets`


**`ReferralFeesClaimed`**
   **Parameters:** `address` userAddress *(indexed)*, `uint256` amount

   **Emitted in:**
      └─ `claimReferralFees`


**`JackpotSettled`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` totalTicketsSold, `uint256` userWinnings, `uint8` winningBonusball, `uint256` winningNumbers, `uint256` newDrawingAccumulator

   **Emitted in:**
      └─ `scaledEntropyCallback`


**`WinnersCalculated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256[]` winningNormals, `uint256` winningBonusball, `uint256[]` uniqueResult, `uint256[]` dupResult

   **Emitted in:**
      └─ `_calculateDrawingUserWinnings`


**`NewDrawingInitialized`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` lpPoolTotal, `uint256` prizePool, `uint256` ticketPrice, `uint256` normalBallMax, `uint8` bonusballMax, `uint256` referralWinShare, `uint256` drawingTime

   **Emitted in:**
      └─ `_setNewDrawingState`


**`JackpotRunRequested`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` entropyGasLimit, `uint256` fee

   **Emitted in:**
      └─ `runJackpot`


**`LpEarningsUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` amount

   **Emitted in:**
      └─ `_payReferrersWinnings`


**`ProtocolFeeCollected`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` amount

   **Emitted in:**
      └─ `_transferProtocolFee`


**`NormalBallMaxUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint8` oldValue, `uint8` newValue

   **Emitted in:**
      └─ `setNormalBallMax`


**`ProtocolFeeThresholdUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setProtocolFeeThreshold`


**`ProtocolFeeUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setProtocolFee`


**`GovernancePoolCapUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setGovernancePoolCap`


**`DrawingDurationUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setDrawingDurationInSeconds`


**`BonusballMinUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint8` oldValue, `uint8` newValue

   **Emitted in:**
      └─ `setBonusballMin`


**`LpEdgeTargetUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setLpEdgeTarget`


**`ReserveRatioUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setReserveRatio`


**`ReferralFeeUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setReferralFee`


**`ReferralWinShareUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setReferralWinShare`


**`ProtocolFeeAddressUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `address` oldAddress *(indexed)*, `address` newAddress *(indexed)*

   **Emitted in:**
      └─ `setProtocolFeeAddress`


**`TicketPriceUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setTicketPrice`


**`MaxReferrersUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint256` oldValue, `uint256` newValue

   **Emitted in:**
      └─ `setMaxReferrers`


**`PayoutCalculatorUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `address` oldPayoutCalculator, `address` newPayoutCalculator

   **Emitted in:**
      └─ `setPayoutCalculator`


**`EntropyBaseGasLimitUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint32` oldValue, `uint32` newValue

   **Emitted in:**
      └─ `setEntropyBaseGasLimit`


**`EntropyVariableGasLimitUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `uint32` oldValue, `uint32` newValue

   **Emitted in:**
      └─ `setEntropyVariableGasLimit`


**`JackpotLocked`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `_lockJackpot`


**`JackpotUnlocked`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `_unlockJackpot`


**`TicketPurchasesEnabled`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `enableTicketPurchases`


**`TicketPurchasesDisabled`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `disableTicketPurchases`


**`EntropyUpdated`**
   **Parameters:** `uint256` drawingId *(indexed)*, `address` oldEntropy, `address` newEntropy

   **Emitted in:**
      └─ `setEntropy`


**`EmergencyModeEnabled`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `enableEmergencyMode`


**`EmergencyModeDisabled`**
   **Parameters:** `uint256` drawingId *(indexed)*

   **Emitted in:**
      └─ `disableEmergencyMode`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**MODIFIERS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`onlyEntropy()`**

   **Used in:**
      └─ `scaledEntropyCallback`


**`noEmergencyMode()`**

   **Used in:**
      ├─ `buyTickets`
      ├─ `lpDeposit`
      ├─ `initiateWithdraw`
      ├─ `finalizeWithdraw`
      └─ `runJackpot`


**`onlyEmergencyMode()`**

   **Used in:**
      ├─ `emergencyWithdrawLP`
      └─ `emergencyRefundTickets`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**CUSTOM ERRORS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`JackpotErrors.InvalidReferralFee`** *(inherited)*

   **Used in:**
      └─ `setReferralFee`


**`JackpotErrors.InvalidGovernancePoolCap`** *(inherited)*

   **Used in:**
      ├─ `setGovernancePoolCap`
      └─ `initializeLPDeposits`


**`JackpotErrors.InvalidBonusballMin`** *(inherited)*

   **Used in:**
      └─ `setBonusballMin`


**`JackpotErrors.NoLPDeposits`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.InvalidMaxReferrers`** *(inherited)*

   **Used in:**
      └─ `setMaxReferrers`


**`JackpotErrors.InvalidTicketCount`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.NoPrizePool`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.EmergencyEnabled`** *(inherited)*

   **Used in:**
      └─ `noEmergencyMode`


**`JackpotErrors.UnauthorizedEntropyCaller`** *(inherited)*

   **Used in:**
      └─ `onlyEntropy`


**`JackpotErrors.LPDepositsNotInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.JackpotNotLocked`** *(inherited)*

   **Used in:**
      ├─ `scaledEntropyCallback`
      └─ `unlockJackpot`


**`JackpotErrors.DrawingNotDue`** *(inherited)*

   **Used in:**
      └─ `runJackpot`


**`JackpotErrors.WithdrawAmountZero`** *(inherited)*

   **Used in:**
      └─ `initiateWithdraw`


**`JackpotErrors.InvalidReserveRatio`** *(inherited)*

   **Used in:**
      └─ `setReserveRatio`


**`JackpotErrors.TicketPurchasesAlreadyDisabled`** *(inherited)*

   **Used in:**
      └─ `disableTicketPurchases`


**`JackpotErrors.ContractNotInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeLPDeposits`


**`JackpotErrors.LPDepositsAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeLPDeposits`


**`JackpotErrors.TicketPurchasesDisabled`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.TooManyReferrers`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.InvalidNormalsCount`** *(inherited)*

   **Used in:**
      └─ `_validateAndStoreTickets`


**`JackpotErrors.ReferralSplitSumInvalid`** *(inherited)*

   **Used in:**
      └─ `_validateAndTrackReferrals`


**`JackpotErrors.InvalidBonusball`** *(inherited)*

   **Used in:**
      └─ `_validateAndStoreTickets`


**`JackpotErrors.EmergencyModeNotEngaged`** *(inherited)*

   **Used in:**
      └─ `onlyEmergencyMode`


**`JackpotErrors.ZeroAddress`** *(inherited)*

   **Used in:**
      ├─ `_validateAndTrackReferrals`
      ├─ `setPayoutCalculator`
      ├─ `setEntropy`
      ├─ `initialize`
      └─ `setProtocolFeeAddress`


**`JackpotErrors.InvalidRecipient`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.ReferralSplitLengthMismatch`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.InvalidProtocolFee`** *(inherited)*

   **Used in:**
      └─ `setProtocolFee`


**`JackpotErrors.InvalidReferralSplitBps`** *(inherited)*

   **Used in:**
      └─ `_validateAndTrackReferrals`


**`JackpotErrors.NoTicketsToClaim`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.InvalidReferralWinShare`** *(inherited)*

   **Used in:**
      └─ `setReferralWinShare`


**`JackpotErrors.TicketFromFutureDrawing`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.InvalidTicketPrice`** *(inherited)*

   **Used in:**
      └─ `setTicketPrice`


**`JackpotErrors.NoReferralFeesToClaim`** *(inherited)*

   **Used in:**
      └─ `claimReferralFees`


**`JackpotErrors.DepositAmountZero`** *(inherited)*

   **Used in:**
      └─ `lpDeposit`


**`JackpotErrors.ContractAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initialize`


**`JackpotErrors.JackpotAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.InvalidDrawingDuration`** *(inherited)*

   **Used in:**
      └─ `setDrawingDurationInSeconds`


**`JackpotErrors.EmergencyModeAlreadyEnabled`** *(inherited)*

   **Used in:**
      └─ `enableEmergencyMode`


**`JackpotErrors.InsufficientEntropyFee`** *(inherited)*

   **Used in:**
      └─ `runJackpot`


**`JackpotErrors.InvalidLpEdgeTarget`** *(inherited)*

   **Used in:**
      └─ `setLpEdgeTarget`


**`JackpotErrors.TicketNotEligibleForRefund`** *(inherited)*

   **Used in:**
      └─ `emergencyRefundTickets`


**`JackpotErrors.JackpotLocked`** *(inherited)*

   **Used in:**
      ├─ `lpDeposit`
      ├─ `_validateBuyTicketInputs`
      ├─ `runJackpot`
      ├─ `initiateWithdraw`
      └─ `lockJackpot`


**`JackpotErrors.NotTicketOwner`** *(inherited)*

   **Used in:**
      ├─ `claimWinnings`
      └─ `emergencyRefundTickets`


**`JackpotErrors.EmergencyModeAlreadyDisabled`** *(inherited)*

   **Used in:**
      └─ `disableEmergencyMode`


**`JackpotErrors.NoTicketsProvided`** *(inherited)*

   **Used in:**
      └─ `emergencyRefundTickets`


**`JackpotErrors.TicketPurchasesAlreadyEnabled`** *(inherited)*

   **Used in:**
      └─ `enableTicketPurchases`


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**FUNCTIONS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**`constructor(uint256 _drawingDurationInSeconds, uint8 _normalBallMax, uint8 _bonusballMin, uint256 _lpEdgeTarget, uint256 _reserveRatio, uint256 _referralFee, uint256 _referralWinShare, uint256 _protocolFee, uint256 _protocolFeeThreshold, uint256 _ticketPrice, uint256 _maxReferrers, uint32 _entropyBaseGasLimit)`**
   **Visibility:** public
   **State Mutability:** nonpayable
   **Line:** 296

   **Modifiers:**
      └─ `Ownable`


**`buyTickets(Ticket[] _tickets, address _recipient, address[] _referrers, uint256[] _referralSplit, bytes32 _source)`** → `uint256[] ticketIds`
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 365

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `noEmergencyMode`


**`claimWinnings(uint256[] _userTicketIds)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 418

   **Modifiers:**
      └─ `nonReentrant`


**`lpDeposit(uint256 _amountToDeposit)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 477

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `noEmergencyMode`


**`initiateWithdraw(uint256 _amountToWithdrawInShares)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 506

   **Modifiers:**
      └─ `noEmergencyMode`


**`finalizeWithdraw()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 531

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `noEmergencyMode`


**`emergencyWithdrawLP()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 552

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `onlyEmergencyMode`


**`emergencyRefundTickets(uint256[] _userTicketIds)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 581

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `onlyEmergencyMode`


**`claimReferralFees()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 618

   **Modifiers:**
      └─ `nonReentrant`


**`runJackpot()`**
   **Visibility:** external
   **State Mutability:** payable
   **Line:** 646

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `noEmergencyMode`


**`scaledEntropyCallback(bytes32, uint256[][] _randomNumbers, bytes)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 717

   **Modifiers:**
      ├─ `nonReentrant`
      └─ `onlyEntropy`


**`initialize(IERC20 _usdc, IJackpotLPManager _jackpotLPManager, IJackpotTicketNFT _jackpotNFT, IScaledEntropyProvider _entropy, IPayoutCalculator _payoutCalculator)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 776

   **Modifiers:**
      └─ `onlyOwner`


**`initializeLPDeposits(uint256 _governancePoolCap)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 820

   **Modifiers:**
      └─ `onlyOwner`


**`initializeJackpot(uint256 _initialDrawingTime)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 852

   **Modifiers:**
      └─ `onlyOwner`


**`setNormalBallMax(uint8 _normalBallMax)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 886

   **Modifiers:**
      └─ `onlyOwner`


**`setProtocolFeeThreshold(uint256 _protocolFeeThreshold)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 905

   **Modifiers:**
      └─ `onlyOwner`


**`setProtocolFee(uint256 _protocolFee)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 923

   **Modifiers:**
      └─ `onlyOwner`


**`setGovernancePoolCap(uint256 _governancePoolCap)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 946

   **Modifiers:**
      └─ `onlyOwner`


**`setDrawingDurationInSeconds(uint256 _drawingDurationInSeconds)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 969

   **Modifiers:**
      └─ `onlyOwner`


**`setBonusballMin(uint8 _bonusballMin)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 990

   **Modifiers:**
      └─ `onlyOwner`


**`setLpEdgeTarget(uint256 _lpEdgeTarget)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1012

   **Modifiers:**
      └─ `onlyOwner`


**`setReserveRatio(uint256 _reserveRatio)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1036

   **Modifiers:**
      └─ `onlyOwner`


**`setReferralFee(uint256 _referralFee)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1059

   **Modifiers:**
      └─ `onlyOwner`


**`setReferralWinShare(uint256 _referralWinShare)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1080

   **Modifiers:**
      └─ `onlyOwner`


**`setProtocolFeeAddress(address _protocolFeeAddress)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1101

   **Modifiers:**
      └─ `onlyOwner`


**`setTicketPrice(uint256 _ticketPrice)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1127

   **Modifiers:**
      └─ `onlyOwner`


**`setMaxReferrers(uint256 _maxReferrers)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1149

   **Modifiers:**
      └─ `onlyOwner`


**`setPayoutCalculator(IPayoutCalculator _payoutCalculator)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1171

   **Modifiers:**
      └─ `onlyOwner`


**`setEntropy(IScaledEntropyProvider _entropy)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1193

   **Modifiers:**
      └─ `onlyOwner`


**`setEntropyBaseGasLimit(uint32 _entropyBaseGasLimit)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1211

   **Modifiers:**
      └─ `onlyOwner`


**`setEntropyVariableGasLimit(uint32 _entropyVariableGasLimit)`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1229

   **Modifiers:**
      └─ `onlyOwner`


**`enableEmergencyMode()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1250

   **Modifiers:**
      └─ `onlyOwner`


**`disableEmergencyMode()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1270

   **Modifiers:**
      └─ `onlyOwner`


**`lockJackpot()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1289

   **Modifiers:**
      └─ `onlyOwner`


**`unlockJackpot()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1307

   **Modifiers:**
      └─ `onlyOwner`


**`enableTicketPurchases()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1325

   **Modifiers:**
      └─ `onlyOwner`


**`disableTicketPurchases()`**
   **Visibility:** external
   **State Mutability:** nonpayable
   **Line:** 1345

   **Modifiers:**
      └─ `onlyOwner`


**`getDrawingState(uint256 _drawingId)`** → `DrawingState`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1362


**`getReferralScheme(bytes32 _referralSchemeId)`** → `ReferralScheme`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1372


**`checkIfTicketsBought(uint256 _drawingId, Ticket[] _tickets)`** → `bool[]`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1383


**`getSubsetCount(uint256 _drawingId, uint8[] _normals, uint8 _bonusball)`** → `TicketComboTracker.ComboCount`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1401


**`getUnpackedTicket(uint256 _drawingId, uint256 _packedTicket)`** → `uint8[] normals, uint8 bonusball`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1422


**`getTicketTierIds(uint256[] _ticketIds)`** → `uint256[] tierIds`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1438


**`getEntropyCallbackFee()`** → `uint256 fee`
   **Visibility:** external
   **State Mutability:** view
   **Line:** 1461


**`_calculateLpPoolCap(uint256 _normalBallMax)`** → `uint256`
   **Visibility:** internal
   **State Mutability:** view
   **Line:** 1469


**`_setNewDrawingState(uint256 _newLpValue, uint256 _nextDrawingTime)`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1478


**`_validateBuyTicketInputs(Ticket[] _tickets, address _recipient, address[] _referrers, uint256[] _referralSplit)`**
   **Visibility:** internal
   **State Mutability:** view
   **Line:** 1519


**`_validateAndTrackReferrals(address[] _referrers, uint256[] _referralSplit, uint256 _ticketsValue)`** → `uint256 referralFeeTotal, bytes32 referralSchemeId`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1534


**`_validateAndStoreTickets(DrawingState _currentDrawingState, Ticket[] _tickets, address _recipient, bytes32 _referralSchemeId, bytes32 _source)`** → `uint256[] ticketIds`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1571


**`_calculateDrawingUserWinnings(DrawingState _currentDrawingState, uint256[][] _unPackedWinningNumbers)`** → `uint256 winningNumbers, uint256 drawingUserWinnings`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1614


**`_calculateTicketTierId(uint256 _ticketNumbers, uint256 _winningNumbers, uint256 _normalBallMax)`** → `uint256`
   **Visibility:** internal
   **State Mutability:** pure
   **Line:** 1651


**`_payReferrersWinnings(bytes32 _referralSchemeId, uint256 _winningAmount, uint256 _referralWinShare)`** → `uint256`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1671


**`_transferProtocolFee(uint256 _lpEarnings, uint256 _drawingUserWinnings)`** → `uint256 protocolFeeAmount`
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1700


**`_calculateEntropyGasLimit(uint8 _bonusballMax)`** → `uint32`
   **Visibility:** internal
   **State Mutability:** view
   **Line:** 1715


**`_lockJackpot()`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1719


**`_unlockJackpot()`**
   **Visibility:** internal
   **State Mutability:** nonpayable
   **Line:** 1724

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
**SECURITY ANALYSIS**
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### Parameter → State Variable Influences

Shows how function parameters affect state variables:

**`constructor`** - Parameter `_drawingDurationInSeconds`:
   Influences:
      • `drawingDurationInSeconds`

**`constructor`** - Parameter `_normalBallMax`:
   Influences:
      • `normalBallMax`

**`constructor`** - Parameter `_bonusballMin`:
   Influences:
      • `bonusballMin`

**`constructor`** - Parameter `_lpEdgeTarget`:
   Influences:
      • `lpEdgeTarget`

**`constructor`** - Parameter `_reserveRatio`:
   Influences:
      • `reserveRatio`

**`constructor`** - Parameter `_referralFee`:
   Influences:
      • `referralFee`

**`constructor`** - Parameter `_referralWinShare`:
   Influences:
      • `referralWinShare`

**`constructor`** - Parameter `_protocolFee`:
   Influences:
      • `protocolFee`

**`constructor`** - Parameter `_protocolFeeThreshold`:
   Influences:
      • `protocolFeeThreshold`

**`constructor`** - Parameter `_ticketPrice`:
   Influences:
      • `ticketPrice`

**`constructor`** - Parameter `_maxReferrers`:
   Influences:
      • `maxReferrers`

**`constructor`** - Parameter `_entropyBaseGasLimit`:
   Influences:
      • `entropyBaseGasLimit`

**`initialize`** - Parameter `_usdc`:
   Influences:
      • `usdc`

**`initialize`** - Parameter `_jackpotLPManager`:
   Influences:
      • `jackpotLPManager`

**`initialize`** - Parameter `_jackpotNFT`:
   Influences:
      • `jackpotNFT`

**`initialize`** - Parameter `_entropy`:
   Influences:
      • `entropy`

**`initialize`** - Parameter `_payoutCalculator`:
   Influences:
      • `payoutCalculator`

**`initializeLPDeposits`** - Parameter `_governancePoolCap`:
   Influences:
      • `governancePoolCap`

**`setNormalBallMax`** - Parameter `_normalBallMax`:
   Influences:
      • `normalBallMax`

**`setProtocolFeeThreshold`** - Parameter `_protocolFeeThreshold`:
   Influences:
      • `protocolFeeThreshold`

**`setProtocolFee`** - Parameter `_protocolFee`:
   Influences:
      • `protocolFee`

**`setGovernancePoolCap`** - Parameter `_governancePoolCap`:
   Influences:
      • `governancePoolCap`

**`setDrawingDurationInSeconds`** - Parameter `_drawingDurationInSeconds`:
   Influences:
      • `drawingDurationInSeconds`

**`setBonusballMin`** - Parameter `_bonusballMin`:
   Influences:
      • `bonusballMin`

**`setLpEdgeTarget`** - Parameter `_lpEdgeTarget`:
   Influences:
      • `lpEdgeTarget`

**`setReserveRatio`** - Parameter `_reserveRatio`:
   Influences:
      • `reserveRatio`

**`setReferralFee`** - Parameter `_referralFee`:
   Influences:
      • `referralFee`

**`setReferralWinShare`** - Parameter `_referralWinShare`:
   Influences:
      • `referralWinShare`

**`setProtocolFeeAddress`** - Parameter `_protocolFeeAddress`:
   Influences:
      • `protocolFeeAddress`

**`setTicketPrice`** - Parameter `_ticketPrice`:
   Influences:
      • `ticketPrice`

**`setMaxReferrers`** - Parameter `_maxReferrers`:
   Influences:
      • `maxReferrers`

**`setPayoutCalculator`** - Parameter `_payoutCalculator`:
   Influences:
      • `payoutCalculator`

**`setEntropy`** - Parameter `_entropy`:
   Influences:
      • `entropy`

**`setEntropyBaseGasLimit`** - Parameter `_entropyBaseGasLimit`:
   Influences:
      • `entropyBaseGasLimit`

**`setEntropyVariableGasLimit`** - Parameter `_entropyVariableGasLimit`:
   Influences:
      • `entropyVariableGasLimit`

**`_validateAndTrackReferrals`** - Parameter `_referralSplit`:
   Influences:
      • `referralFees`

**`_validateAndTrackReferrals`** - Parameter `_ticketsValue`:
   Influences:
      • `referralFees`

**`_payReferrersWinnings`** - Parameter `_referralSchemeId`:
   Influences:
      • `referralFees`

**`_payReferrersWinnings`** - Parameter `_winningAmount`:
   Influences:
      • `drawingState`
      • `referralFees`

**`_payReferrersWinnings`** - Parameter `_referralWinShare`:
   Influences:
      • `drawingState`
      • `referralFees`

### Data Flow Security Findings

#### 🟡 MEDIUM Severity

1. **Function:** `constructor`
   - **Source:** Function parameter `_drawingDurationInSeconds`
   - **Sink:** State modification: `drawingDurationInSeconds`
   - **Status:** ⚠️ No validation detected

2. **Function:** `constructor`
   - **Source:** Function parameter `_drawingDurationInSeconds`
   - **Sink:** State modification: `drawingDurationInSeconds`
   - **Status:** ⚠️ No validation detected

3. **Function:** `constructor`
   - **Source:** Function parameter `_normalBallMax`
   - **Sink:** State modification: `normalBallMax`
   - **Status:** ⚠️ No validation detected

4. **Function:** `constructor`
   - **Source:** Function parameter `_normalBallMax`
   - **Sink:** State modification: `normalBallMax`
   - **Status:** ⚠️ No validation detected

5. **Function:** `constructor`
   - **Source:** Function parameter `_bonusballMin`
   - **Sink:** State modification: `bonusballMin`
   - **Status:** ⚠️ No validation detected

6. **Function:** `constructor`
   - **Source:** Function parameter `_bonusballMin`
   - **Sink:** State modification: `bonusballMin`
   - **Status:** ⚠️ No validation detected

7. **Function:** `constructor`
   - **Source:** Function parameter `_lpEdgeTarget`
   - **Sink:** State modification: `lpEdgeTarget`
   - **Status:** ⚠️ No validation detected

8. **Function:** `constructor`
   - **Source:** Function parameter `_lpEdgeTarget`
   - **Sink:** State modification: `lpEdgeTarget`
   - **Status:** ⚠️ No validation detected

9. **Function:** `constructor`
   - **Source:** Function parameter `_reserveRatio`
   - **Sink:** State modification: `reserveRatio`
   - **Status:** ⚠️ No validation detected

10. **Function:** `constructor`
   - **Source:** Function parameter `_reserveRatio`
   - **Sink:** State modification: `reserveRatio`
   - **Status:** ⚠️ No validation detected

11. **Function:** `constructor`
   - **Source:** Function parameter `_referralFee`
   - **Sink:** State modification: `referralFee`
   - **Status:** ⚠️ No validation detected

12. **Function:** `constructor`
   - **Source:** Function parameter `_referralFee`
   - **Sink:** State modification: `referralFee`
   - **Status:** ⚠️ No validation detected

13. **Function:** `constructor`
   - **Source:** Function parameter `_referralWinShare`
   - **Sink:** State modification: `referralWinShare`
   - **Status:** ⚠️ No validation detected

14. **Function:** `constructor`
   - **Source:** Function parameter `_referralWinShare`
   - **Sink:** State modification: `referralWinShare`
   - **Status:** ⚠️ No validation detected

15. **Function:** `constructor`
   - **Source:** Function parameter `_protocolFee`
   - **Sink:** State modification: `protocolFee`
   - **Status:** ⚠️ No validation detected

16. **Function:** `constructor`
   - **Source:** Function parameter `_protocolFee`
   - **Sink:** State modification: `protocolFee`
   - **Status:** ⚠️ No validation detected

17. **Function:** `constructor`
   - **Source:** Function parameter `_protocolFeeThreshold`
   - **Sink:** State modification: `protocolFeeThreshold`
   - **Status:** ⚠️ No validation detected

18. **Function:** `constructor`
   - **Source:** Function parameter `_protocolFeeThreshold`
   - **Sink:** State modification: `protocolFeeThreshold`
   - **Status:** ⚠️ No validation detected

19. **Function:** `constructor`
   - **Source:** Function parameter `_ticketPrice`
   - **Sink:** State modification: `ticketPrice`
   - **Status:** ⚠️ No validation detected

20. **Function:** `constructor`
   - **Source:** Function parameter `_ticketPrice`
   - **Sink:** State modification: `ticketPrice`
   - **Status:** ⚠️ No validation detected

21. **Function:** `constructor`
   - **Source:** Function parameter `_maxReferrers`
   - **Sink:** State modification: `maxReferrers`
   - **Status:** ⚠️ No validation detected

22. **Function:** `constructor`
   - **Source:** Function parameter `_maxReferrers`
   - **Sink:** State modification: `maxReferrers`
   - **Status:** ⚠️ No validation detected

23. **Function:** `constructor`
   - **Source:** Function parameter `_entropyBaseGasLimit`
   - **Sink:** State modification: `entropyBaseGasLimit`
   - **Status:** ⚠️ No validation detected

24. **Function:** `constructor`
   - **Source:** Function parameter `_entropyBaseGasLimit`
   - **Sink:** State modification: `entropyBaseGasLimit`
   - **Status:** ⚠️ No validation detected

25. **Function:** `initialize`
   - **Source:** Function parameter `_usdc`
   - **Sink:** State modification: `usdc`
   - **Status:** ✅ Validated

26. **Function:** `initialize`
   - **Source:** Function parameter `_usdc`
   - **Sink:** State modification: `usdc`
   - **Status:** ✅ Validated

27. **Function:** `initialize`
   - **Source:** Function parameter `_jackpotLPManager`
   - **Sink:** State modification: `jackpotLPManager`
   - **Status:** ✅ Validated

28. **Function:** `initialize`
   - **Source:** Function parameter `_jackpotLPManager`
   - **Sink:** State modification: `jackpotLPManager`
   - **Status:** ✅ Validated

29. **Function:** `initialize`
   - **Source:** Function parameter `_jackpotNFT`
   - **Sink:** State modification: `jackpotNFT`
   - **Status:** ✅ Validated

30. **Function:** `initialize`
   - **Source:** Function parameter `_jackpotNFT`
   - **Sink:** State modification: `jackpotNFT`
   - **Status:** ✅ Validated

31. **Function:** `initialize`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

32. **Function:** `initialize`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

33. **Function:** `initialize`
   - **Source:** Function parameter `_payoutCalculator`
   - **Sink:** State modification: `payoutCalculator`
   - **Status:** ✅ Validated

34. **Function:** `initialize`
   - **Source:** Function parameter `_payoutCalculator`
   - **Sink:** State modification: `payoutCalculator`
   - **Status:** ✅ Validated

35. **Function:** `initializeLPDeposits`
   - **Source:** Function parameter `_governancePoolCap`
   - **Sink:** State modification: `governancePoolCap`
   - **Status:** ✅ Validated

36. **Function:** `initializeLPDeposits`
   - **Source:** Function parameter `_governancePoolCap`
   - **Sink:** State modification: `governancePoolCap`
   - **Status:** ✅ Validated

37. **Function:** `setNormalBallMax`
   - **Source:** Function parameter `_normalBallMax`
   - **Sink:** State modification: `normalBallMax`
   - **Status:** ⚠️ No validation detected

38. **Function:** `setNormalBallMax`
   - **Source:** Function parameter `_normalBallMax`
   - **Sink:** State modification: `normalBallMax`
   - **Status:** ⚠️ No validation detected

39. **Function:** `setProtocolFeeThreshold`
   - **Source:** Function parameter `_protocolFeeThreshold`
   - **Sink:** State modification: `protocolFeeThreshold`
   - **Status:** ⚠️ No validation detected

40. **Function:** `setProtocolFeeThreshold`
   - **Source:** Function parameter `_protocolFeeThreshold`
   - **Sink:** State modification: `protocolFeeThreshold`
   - **Status:** ⚠️ No validation detected

41. **Function:** `setProtocolFee`
   - **Source:** Function parameter `_protocolFee`
   - **Sink:** State modification: `protocolFee`
   - **Status:** ✅ Validated

42. **Function:** `setProtocolFee`
   - **Source:** Function parameter `_protocolFee`
   - **Sink:** State modification: `protocolFee`
   - **Status:** ✅ Validated

43. **Function:** `setGovernancePoolCap`
   - **Source:** Function parameter `_governancePoolCap`
   - **Sink:** State modification: `governancePoolCap`
   - **Status:** ✅ Validated

44. **Function:** `setGovernancePoolCap`
   - **Source:** Function parameter `_governancePoolCap`
   - **Sink:** State modification: `governancePoolCap`
   - **Status:** ✅ Validated

45. **Function:** `setDrawingDurationInSeconds`
   - **Source:** Function parameter `_drawingDurationInSeconds`
   - **Sink:** State modification: `drawingDurationInSeconds`
   - **Status:** ✅ Validated

46. **Function:** `setDrawingDurationInSeconds`
   - **Source:** Function parameter `_drawingDurationInSeconds`
   - **Sink:** State modification: `drawingDurationInSeconds`
   - **Status:** ✅ Validated

47. **Function:** `setBonusballMin`
   - **Source:** Function parameter `_bonusballMin`
   - **Sink:** State modification: `bonusballMin`
   - **Status:** ✅ Validated

48. **Function:** `setBonusballMin`
   - **Source:** Function parameter `_bonusballMin`
   - **Sink:** State modification: `bonusballMin`
   - **Status:** ✅ Validated

49. **Function:** `setLpEdgeTarget`
   - **Source:** Function parameter `_lpEdgeTarget`
   - **Sink:** State modification: `lpEdgeTarget`
   - **Status:** ✅ Validated

50. **Function:** `setLpEdgeTarget`
   - **Source:** Function parameter `_lpEdgeTarget`
   - **Sink:** State modification: `lpEdgeTarget`
   - **Status:** ✅ Validated

51. **Function:** `setReserveRatio`
   - **Source:** Function parameter `_reserveRatio`
   - **Sink:** State modification: `reserveRatio`
   - **Status:** ✅ Validated

52. **Function:** `setReserveRatio`
   - **Source:** Function parameter `_reserveRatio`
   - **Sink:** State modification: `reserveRatio`
   - **Status:** ✅ Validated

53. **Function:** `setReferralFee`
   - **Source:** Function parameter `_referralFee`
   - **Sink:** State modification: `referralFee`
   - **Status:** ✅ Validated

54. **Function:** `setReferralFee`
   - **Source:** Function parameter `_referralFee`
   - **Sink:** State modification: `referralFee`
   - **Status:** ✅ Validated

55. **Function:** `setReferralWinShare`
   - **Source:** Function parameter `_referralWinShare`
   - **Sink:** State modification: `referralWinShare`
   - **Status:** ✅ Validated

56. **Function:** `setReferralWinShare`
   - **Source:** Function parameter `_referralWinShare`
   - **Sink:** State modification: `referralWinShare`
   - **Status:** ✅ Validated

57. **Function:** `setProtocolFeeAddress`
   - **Source:** Function parameter `_protocolFeeAddress`
   - **Sink:** State modification: `protocolFeeAddress`
   - **Status:** ✅ Validated

58. **Function:** `setProtocolFeeAddress`
   - **Source:** Function parameter `_protocolFeeAddress`
   - **Sink:** State modification: `protocolFeeAddress`
   - **Status:** ✅ Validated

59. **Function:** `setTicketPrice`
   - **Source:** Function parameter `_ticketPrice`
   - **Sink:** State modification: `ticketPrice`
   - **Status:** ✅ Validated

60. **Function:** `setTicketPrice`
   - **Source:** Function parameter `_ticketPrice`
   - **Sink:** State modification: `ticketPrice`
   - **Status:** ✅ Validated

61. **Function:** `setMaxReferrers`
   - **Source:** Function parameter `_maxReferrers`
   - **Sink:** State modification: `maxReferrers`
   - **Status:** ✅ Validated

62. **Function:** `setMaxReferrers`
   - **Source:** Function parameter `_maxReferrers`
   - **Sink:** State modification: `maxReferrers`
   - **Status:** ✅ Validated

63. **Function:** `setPayoutCalculator`
   - **Source:** Function parameter `_payoutCalculator`
   - **Sink:** State modification: `payoutCalculator`
   - **Status:** ✅ Validated

64. **Function:** `setPayoutCalculator`
   - **Source:** Function parameter `_payoutCalculator`
   - **Sink:** State modification: `payoutCalculator`
   - **Status:** ✅ Validated

65. **Function:** `setEntropy`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

66. **Function:** `setEntropy`
   - **Source:** Function parameter `_entropy`
   - **Sink:** State modification: `entropy`
   - **Status:** ✅ Validated

67. **Function:** `setEntropyBaseGasLimit`
   - **Source:** Function parameter `_entropyBaseGasLimit`
   - **Sink:** State modification: `entropyBaseGasLimit`
   - **Status:** ⚠️ No validation detected

68. **Function:** `setEntropyBaseGasLimit`
   - **Source:** Function parameter `_entropyBaseGasLimit`
   - **Sink:** State modification: `entropyBaseGasLimit`
   - **Status:** ⚠️ No validation detected

69. **Function:** `setEntropyVariableGasLimit`
   - **Source:** Function parameter `_entropyVariableGasLimit`
   - **Sink:** State modification: `entropyVariableGasLimit`
   - **Status:** ⚠️ No validation detected

70. **Function:** `setEntropyVariableGasLimit`
   - **Source:** Function parameter `_entropyVariableGasLimit`
   - **Sink:** State modification: `entropyVariableGasLimit`
   - **Status:** ⚠️ No validation detected

#### ⚠️ LOW Severity

1. **Function:** `constructor`
   - **Source:** msg.sender
   - **Sink:** State modification: `protocolFeeAddress`
   - **Status:** ⚠️ No validation detected

2. **Function:** `constructor`
   - **Source:** msg.sender
   - **Sink:** State modification: `protocolFeeAddress`
   - **Status:** ⚠️ No validation detected

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

2. **In function:** `claimWinnings`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

3. **In function:** `lpDeposit`
   - **Ignored call:** `usdc.safeTransferFrom()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransferFrom`

4. **In function:** `finalizeWithdraw`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

5. **In function:** `emergencyWithdrawLP`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

6. **In function:** `emergencyRefundTickets`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

7. **In function:** `claimReferralFees`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

8. **In function:** `_transferProtocolFee`
   - **Ignored call:** `usdc.safeTransfer()`
   - **Risk:** 🔴 **HIGH** - This can lead to silent failures
   - **Recommendation:** Always check the return value of `safeTransfer`

### 🟡 MEDIUM Severity

1. **In function:** `claimWinnings`
   - **Ignored call:** `jackpotNFT.burnTicket()`

2. **In function:** `lpDeposit`
   - **Ignored call:** `jackpotLPManager.processDeposit()`

3. **In function:** `initiateWithdraw`
   - **Ignored call:** `jackpotLPManager.processInitiateWithdraw()`

4. **In function:** `emergencyRefundTickets`
   - **Ignored call:** `jackpotNFT.burnTicket()`

5. **In function:** `_validateAndStoreTickets`
   - **Ignored call:** `jackpotNFT.mintTicket()`

### ⚠️ LOW Severity

1. **In function:** `buyTickets`
   - **Ignored call:** `_validateBuyTicketInputs()`

2. **In function:** `runJackpot`
   - **Ignored call:** `_lockJackpot()`

3. **In function:** `scaledEntropyCallback`
   - **Ignored call:** `_setNewDrawingState()`

4. **In function:** `initializeLPDeposits`
   - **Ignored call:** `jackpotLPManager.initializeLP()`

5. **In function:** `initializeLPDeposits`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

6. **In function:** `initializeJackpot`
   - **Ignored call:** `_setNewDrawingState()`

7. **In function:** `setNormalBallMax`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

8. **In function:** `setGovernancePoolCap`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

9. **In function:** `setLpEdgeTarget`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

10. **In function:** `setReserveRatio`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

11. **In function:** `setTicketPrice`
   - **Ignored call:** `jackpotLPManager.setLPPoolCap()`

12. **In function:** `lockJackpot`
   - **Ignored call:** `_lockJackpot()`

13. **In function:** `unlockJackpot`
   - **Ignored call:** `_unlockJackpot()`

14. **In function:** `_setNewDrawingState`
   - **Ignored call:** `jackpotLPManager.initializeDrawingLP()`

15. **In function:** `_setNewDrawingState`
   - **Ignored call:** `TicketComboTracker.init()`

16. **In function:** `_setNewDrawingState`
   - **Ignored call:** `payoutCalculator.setDrawingTierInfo()`

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
