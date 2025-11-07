════════════════════════════════════════════════════════════════════════════════
                           **CONTRACT: `Jackpot`**
════════════════════════════════════════════════════════════════════════════════

**File:** `./contracts/Jackpot.sol`


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

**`NORMAL_BALL_COUNT`**
   **Type:** `uint8`
   **Visibility:** internal, constant

**`MAX_BIT_VECTOR_SIZE`**
   **Type:** `uint8`
   **Visibility:** internal, constant

**`MAX_PROTOCOL_FEE`**
   **Type:** `uint256`
   **Visibility:** internal, constant

**`drawingEntries`**
   **Type:** `mapping(uint256 => TicketComboTracker.Tracker)`
   **Visibility:** internal

   **Modified by:** *None*


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


**`referralFees`**
   **Type:** `mapping(address => uint256)`
   **Visibility:** public

   **Modified by:**
      ├─ `claimReferralFees` *(external)*
      ├─ `_validateAndTrackReferrals` *(internal)*
      └─ `_payReferrersWinnings` *(internal)*


**`referralSchemes`**
   **Type:** `mapping(bytes32 => ReferralScheme)`
   **Visibility:** internal

   **Modified by:**
      └─ `_validateAndTrackReferrals` *(internal)*


**`currentDrawingId`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      └─ `_setNewDrawingState` *(internal)* ← `scaledEntropyCallback` *(external)* ← `initializeJackpot` *(external)*


**`ticketPrice`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setTicketPrice` *(external)*


**`normalBallMax`**
   **Type:** `uint8`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setNormalBallMax` *(external)*


**`bonusballMin`**
   **Type:** `uint8`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setBonusballMin` *(external)*


**`drawingDurationInSeconds`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setDrawingDurationInSeconds` *(external)*


**`reserveRatio`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReserveRatio` *(external)*


**`lpEdgeTarget`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setLpEdgeTarget` *(external)*


**`governancePoolCap`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeLPDeposits` *(external)*
      └─ `setGovernancePoolCap` *(external)*


**`referralFee`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReferralFee` *(external)*


**`referralWinShare`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setReferralWinShare` *(external)*


**`protocolFee`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFee` *(external)*


**`protocolFeeThreshold`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFeeThreshold` *(external)*


**`protocolFeeAddress`**
   **Type:** `address`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setProtocolFeeAddress` *(external)*


**`maxReferrers`**
   **Type:** `uint256`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setMaxReferrers` *(external)*


**`initialized`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*


**`allowTicketPurchases`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      ├─ `initializeJackpot` *(external)*
      ├─ `enableTicketPurchases` *(external)*
      └─ `disableTicketPurchases` *(external)*


**`emergencyMode`**
   **Type:** `bool`
   **Visibility:** public

   **Modified by:**
      ├─ `enableEmergencyMode` *(external)*
      └─ `disableEmergencyMode` *(external)*


**`entropyBaseGasLimit`**
   **Type:** `uint32`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setEntropyBaseGasLimit` *(external)*


**`entropyVariableGasLimit`**
   **Type:** `uint32`
   **Visibility:** public

   **Modified by:**
      ├─ `constructor` *(public)*
      └─ `setEntropyVariableGasLimit` *(external)*


**`usdc`**
   **Type:** `IERC20`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*


**`jackpotLPManager`**
   **Type:** `IJackpotLPManager`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*


**`jackpotNFT`**
   **Type:** `IJackpotTicketNFT`
   **Visibility:** public

   **Modified by:**
      └─ `initialize` *(external)*


**`entropy`**
   **Type:** `IScaledEntropyProvider`
   **Visibility:** public

   **Modified by:**
      ├─ `initialize` *(external)*
      └─ `setEntropy` *(external)*


**`payoutCalculator`**
   **Type:** `IPayoutCalculator`
   **Visibility:** public

   **Modified by:**
      ├─ `initialize` *(external)*
      └─ `setPayoutCalculator` *(external)*


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

**`JackpotErrors.NoLPDeposits`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.TicketNotEligibleForRefund`** *(inherited)*

   **Used in:**
      └─ `emergencyRefundTickets`


**`JackpotErrors.NoReferralFeesToClaim`** *(inherited)*

   **Used in:**
      └─ `claimReferralFees`


**`JackpotErrors.TicketPurchasesAlreadyDisabled`** *(inherited)*

   **Used in:**
      └─ `disableTicketPurchases`


**`JackpotErrors.InsufficientEntropyFee`** *(inherited)*

   **Used in:**
      └─ `runJackpot`


**`JackpotErrors.InvalidReferralWinShare`** *(inherited)*

   **Used in:**
      └─ `setReferralWinShare`


**`JackpotErrors.TicketFromFutureDrawing`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.DrawingNotDue`** *(inherited)*

   **Used in:**
      └─ `runJackpot`


**`JackpotErrors.ReferralSplitSumInvalid`** *(inherited)*

   **Used in:**
      └─ `_validateAndTrackReferrals`


**`JackpotErrors.LPDepositsNotInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.ZeroAddress`** *(inherited)*

   **Used in:**
      ├─ `setProtocolFeeAddress`
      ├─ `setPayoutCalculator`
      ├─ `setEntropy`
      ├─ `initialize`
      └─ `_validateAndTrackReferrals`


**`JackpotErrors.EmergencyEnabled`** *(inherited)*

   **Used in:**
      └─ `noEmergencyMode`


**`JackpotErrors.InvalidTicketCount`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.JackpotAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeJackpot`


**`JackpotErrors.NotTicketOwner`** *(inherited)*

   **Used in:**
      ├─ `claimWinnings`
      └─ `emergencyRefundTickets`


**`JackpotErrors.NoPrizePool`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.NoTicketsToClaim`** *(inherited)*

   **Used in:**
      └─ `claimWinnings`


**`JackpotErrors.InvalidGovernancePoolCap`** *(inherited)*

   **Used in:**
      ├─ `initializeLPDeposits`
      └─ `setGovernancePoolCap`


**`JackpotErrors.InvalidMaxReferrers`** *(inherited)*

   **Used in:**
      └─ `setMaxReferrers`


**`JackpotErrors.WithdrawAmountZero`** *(inherited)*

   **Used in:**
      └─ `initiateWithdraw`


**`JackpotErrors.LPDepositsAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeLPDeposits`


**`JackpotErrors.ReferralSplitLengthMismatch`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.JackpotLocked`** *(inherited)*

   **Used in:**
      ├─ `runJackpot`
      ├─ `initiateWithdraw`
      ├─ `lpDeposit`
      ├─ `lockJackpot`
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.InvalidBonusball`** *(inherited)*

   **Used in:**
      └─ `_validateAndStoreTickets`


**`JackpotErrors.InvalidDrawingDuration`** *(inherited)*

   **Used in:**
      └─ `setDrawingDurationInSeconds`


**`JackpotErrors.TicketPurchasesAlreadyEnabled`** *(inherited)*

   **Used in:**
      └─ `enableTicketPurchases`


**`JackpotErrors.DepositAmountZero`** *(inherited)*

   **Used in:**
      └─ `lpDeposit`


**`JackpotErrors.TicketPurchasesDisabled`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.TooManyReferrers`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.ContractAlreadyInitialized`** *(inherited)*

   **Used in:**
      └─ `initialize`


**`JackpotErrors.InvalidBonusballMin`** *(inherited)*

   **Used in:**
      └─ `setBonusballMin`


**`JackpotErrors.InvalidLpEdgeTarget`** *(inherited)*

   **Used in:**
      └─ `setLpEdgeTarget`


**`JackpotErrors.EmergencyModeAlreadyDisabled`** *(inherited)*

   **Used in:**
      └─ `disableEmergencyMode`


**`JackpotErrors.InvalidReferralSplitBps`** *(inherited)*

   **Used in:**
      └─ `_validateAndTrackReferrals`


**`JackpotErrors.InvalidProtocolFee`** *(inherited)*

   **Used in:**
      └─ `setProtocolFee`


**`JackpotErrors.InvalidReferralFee`** *(inherited)*

   **Used in:**
      └─ `setReferralFee`


**`JackpotErrors.JackpotNotLocked`** *(inherited)*

   **Used in:**
      ├─ `unlockJackpot`
      └─ `scaledEntropyCallback`


**`JackpotErrors.InvalidReserveRatio`** *(inherited)*

   **Used in:**
      └─ `setReserveRatio`


**`JackpotErrors.InvalidRecipient`** *(inherited)*

   **Used in:**
      └─ `_validateBuyTicketInputs`


**`JackpotErrors.InvalidNormalsCount`** *(inherited)*

   **Used in:**
      └─ `_validateAndStoreTickets`


**`JackpotErrors.NoTicketsProvided`** *(inherited)*

   **Used in:**
      └─ `emergencyRefundTickets`


**`JackpotErrors.ContractNotInitialized`** *(inherited)*

   **Used in:**
      └─ `initializeLPDeposits`


**`JackpotErrors.InvalidTicketPrice`** *(inherited)*

   **Used in:**
      └─ `setTicketPrice`


**`JackpotErrors.EmergencyModeAlreadyEnabled`** *(inherited)*

   **Used in:**
      └─ `enableEmergencyMode`


**`JackpotErrors.UnauthorizedEntropyCaller`** *(inherited)*

   **Used in:**
      └─ `onlyEntropy`


**`JackpotErrors.EmergencyModeNotEngaged`** *(inherited)*

   **Used in:**
      └─ `onlyEmergencyMode`


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

════════════════════════════════════════════════════════════════════════════════
*Generated by MainnetReady - Solidity Enhanced Analyzer*
