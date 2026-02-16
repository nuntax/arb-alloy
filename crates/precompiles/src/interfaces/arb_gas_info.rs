alloy_core::sol! {
    /// ArbGasInfo: gas pricing and L1/L2 fee information.
    ///
    /// Provides methods to query the current gas pricing model, L1 calldata
    /// costs, and various pricing parameters used by ArbOS.
    ///
    /// Nitro reference: `nitro/precompiles/ArbGasInfo.go`.
    #[sol(rpc)]
    interface ArbGasInfo {
        /// Returns gas prices in wei with a specific aggregator.
        /// Returns: (perL2Tx, perL1CalldataUnit, perStorageAllocation,
        ///           perArbGasBase, perArbGasCongestion, perArbGasTotal)
        function getPricesInWeiWithAggregator(address aggregator)
            external
            view
            returns (uint256, uint256, uint256, uint256, uint256, uint256);

        /// Returns gas prices in wei (using default aggregator).
        function getPricesInWei()
            external
            view
            returns (uint256, uint256, uint256, uint256, uint256, uint256);

        /// Returns gas prices in ArbGas units with a specific aggregator.
        /// Returns: (perL2Tx, perL1Calldata, perStorageAllocation)
        function getPricesInArbGasWithAggregator(address aggregator)
            external
            view
            returns (uint256, uint256, uint256);

        /// Returns gas prices in ArbGas units.
        function getPricesInArbGas()
            external
            view
            returns (uint256, uint256, uint256);

        /// Returns gas accounting parameters.
        /// Returns: (speedLimitPerSecond, gasPoolMax, maxPerBlockGasLimit)
        function getGasAccountingParams()
            external
            view
            returns (uint256, uint256, uint256);

        /// Returns the maximum gas allowed per transaction.
        function getMaxTxGasLimit() external view returns (uint256);

        /// Returns the minimum L2 base fee in wei.
        function getMinimumGasPrice() external view returns (uint256);

        /// Returns the current L1 base-fee estimate in wei.
        function getL1BaseFeeEstimate() external view returns (uint256);

        /// Returns the L1 base-fee estimate inertia parameter.
        function getL1BaseFeeEstimateInertia() external view returns (uint64);

        /// Returns the L1 pricer reward rate (wei per calldata unit).
        function getL1RewardRate() external view returns (uint64);

        /// Returns the address receiving L1 pricer rewards.
        function getL1RewardRecipient() external view returns (address);

        /// Returns the L1 gas price estimate (wei per gas).
        function getL1GasPriceEstimate() external view returns (uint256);

        /// Returns the L1 gas fees charged to the current transaction.
        function getCurrentTxL1GasFees() external view returns (uint256);

        /// Returns the current gas backlog (excess gas above speed limit).
        function getGasBacklog() external view returns (uint64);

        /// Returns the L2 pricing inertia parameter.
        function getPricingInertia() external view returns (uint64);

        /// Returns the L2 gas backlog tolerance.
        function getGasBacklogTolerance() external view returns (uint64);

        /// Returns the L1 pricing surplus (positive) or deficit (negative).
        function getL1PricingSurplus() external view returns (int256);

        /// Returns the per-batch gas charge (can be negative).
        function getPerBatchGasCharge() external view returns (int64);

        /// Returns the amortized cost cap in basis points.
        function getAmortizedCostCapBips() external view returns (uint64);

        /// Returns the available L1 fee funds in wei.
        function getL1FeesAvailable() external view returns (uint256);

        /// Returns the L1 pricing equilibration units.
        function getL1PricingEquilibrationUnits() external view returns (uint256);

        /// Returns the timestamp of the last L1 pricing update.
        function getLastL1PricingUpdateTime() external view returns (uint64);

        /// Returns funds due for L1 pricing rewards.
        function getL1PricingFundsDueForRewards() external view returns (uint256);

        /// Returns calldata units processed since the last L1 pricing update.
        function getL1PricingUnitsSinceUpdate() external view returns (uint64);

        /// Returns the last recorded L1 pricing surplus.
        function getLastL1PricingSurplus() external view returns (int256);

        /// Returns the maximum block gas limit.
        function getMaxBlockGasLimit() external view returns (uint64);
    }
}
