window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic will not be allowed to consume more than `AvailableBlockRatio - 1%`."],["BABE_GENESIS_EPOCH_CONFIG","The BABE epoch configuration at genesis."],["MAXIMUM_BLOCK_WEIGHT","We allow for 2 seconds of compute with a 6 second average block time. The storage proof size is not limited so far."],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by  Operational  extrinsics."],["VERSION","Runtime version (Test)."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["OriginCaller",""],["ParasSudoWrapperCall","Contains one variant per dispatchable that can be called by an extrinsic."],["RuntimeCall",""],["RuntimeEvent",""],["RuntimeFreezeReason","A reason for placing a freeze on funds."],["RuntimeHoldReason","A reason for placing a hold on funds."],["RuntimeLockId","An identifier for each lock placed on funds."],["RuntimeSlashReason","A reason for slashing funds."],["StakerStatus","Indicates the initial status of the staker."],["SudoCall","Contains one variant per dispatchable that can be called by an extrinsic."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["native_version","Native version."]],"macro":[["impl_elections_weights","Implements the weight types for the elections module and a specific runtime. This macro should not be called directly; use [`impl_runtime_weights`] instead."]],"mod":[["api",""],["pallet_test_notifier","The module that hosts all the FRAME types needed to add this pallet to a runtime."],["xcm_config",""]],"struct":[["BlockExecutionWeight","Importing a block with 0 Extrinsics."],["BlockWeights","Block weights base values and limits."],["BondingDuration",""],["EndingPeriod",""],["EpochDuration",""],["ExistentialDeposit",""],["ExpectedBlockTime",""],["ExtrinsicBaseWeight","Executing a NO-OP `System::remarks` Extrinsic."],["FirstMessageFactorPercent",""],["GenesisConfig",""],["IndexDeposit",""],["LeasePeriod",""],["MaxAuthorities",""],["MaxElectableTargets",""],["MaxElectingVoters",""],["MaxLocks",""],["MaxNominatorRewardedPerValidator",""],["MaxReserves",""],["MaxSetIdSessionEntries",""],["MinVestedTransfer",""],["MinimumPeriod",""],["OffchainSolutionLengthLimit","A limit for off-chain phragmen unsigned solution length."],["OffchainSolutionWeightLimit","A limit for off-chain phragmen unsigned solution submission."],["OffendingValidatorsThreshold",""],["Offset",""],["OnChainMaxWinners",""],["OnChainSeqPhragmen",""],["OperationalFeeMultiplier","This value increases the priority of `Operational` transactions by adding a “virtual tip” that’s equal to the `OperationalFeeMultiplier * final_fee`."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["ParasUnsignedPriority",""],["ParityDbWeight","`ParityDB` can be enabled with a feature flag, but is still experimental. These weights are available for brave runtime engineers who may want to try this out as default."],["Period",""],["Prefix",""],["RewardCurve",""],["RocksDbWeight","By default, Substrate uses `RocksDB`, so this will be the weight used throughout the runtime."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SS58Prefix",""],["SessionKeys",""],["SessionsPerEra",""],["SlashDeferDuration",""],["SlotDuration",""],["TransactionByteFee",""],["UnvestedFundsAllowedWithdrawReasons",""],["Version",""]],"trait":[["BuildStorage","Complex storage builder stuff."],["GetLastTimestamp",""]],"type":[["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["AuthorityDiscovery",""],["AuthorityDiscoveryConfig",""],["Authorship",""],["Babe",""],["BabeConfig",""],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","`BlockId` type as expected by this runtime."],["Claims",""],["ClaimsConfig",""],["Configuration",""],["ConfigurationConfig",""],["Dmp",""],["Executive","Executive: handles dispatch to the various modules."],["Extrinsic",""],["Grandpa",""],["GrandpaConfig",""],["Hash",""],["Header","Block header type as expected by this runtime."],["Historical",""],["Hrmp",""],["Indices",""],["IndicesConfig",""],["Initializer",""],["Offences",""],["ParaInclusion",""],["ParaInherent",""],["ParaSessionInfo",""],["Paras",""],["ParasDisputes",""],["ParasOrigin",""],["ParasShared",""],["ParasSudoWrapper",""],["Scheduler",""],["Session",""],["SessionConfig",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The `SignedExtension` to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["Staking",""],["StakingConfig",""],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["TestNotifier",""],["Timestamp",""],["TransactionPayment",""],["Ump",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Vesting",""],["VestingConfig",""],["Xcm",""]]};