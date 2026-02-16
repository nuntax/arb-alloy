alloy_core::sol! {
    /// ArbStatistics: chain statistics.
    ///
    /// Returns mostly legacy pre-Nitro statistics. The first return value
    /// (block number) is the only meaningful field post-Nitro.
    ///
    /// Nitro reference: `nitro/precompiles/ArbStatistics.go`.
    #[sol(rpc)]
    interface ArbStatistics {
        /// Returns chain statistics.
        /// Returns: (blockNumber, classicNumAccounts, classicStorageSum,
        ///           classicGasSum, classicNumTxes, classicNumContracts)
        function getStats()
            external
            view
            returns (uint256, uint256, uint256, uint256, uint256, uint256);
    }
}
