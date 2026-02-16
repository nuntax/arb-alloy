alloy_core::sol! {
    /// ArbAggregator: batch poster management.
    ///
    /// Nitro reference: `nitro/precompiles/ArbAggregator.go`.
    #[sol(rpc)]
    interface ArbAggregator {
        /// Returns the preferred aggregator for an account (deprecated).
        function getPreferredAggregator(address account)
            external
            view
            returns (address, bool);

        /// Returns the default aggregator (deprecated).
        function getDefaultAggregator() external view returns (address);

        /// Returns all registered batch posters.
        function getBatchPosters() external view returns (address[] memory);

        /// Returns the fee collector for a batch poster.
        function getFeeCollector(address batchPoster) external view returns (address);

        /// Returns the base fee for an aggregator (deprecated).
        function getTxBaseFee(address aggregator) external view returns (uint256);

        /// Adds a new batch poster (owner only).
        function addBatchPoster(address newBatchPoster) external;

        /// Sets the fee collector for a batch poster.
        function setFeeCollector(address batchPoster, address newFeeCollector) external;

        /// Sets the base fee for an aggregator (deprecated, owner only).
        function setTxBaseFee(address aggregator, uint256 feeInL1Gas) external;
    }
}
