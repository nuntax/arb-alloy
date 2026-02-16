alloy_core::sol! {
    /// ArbosActs: internal ArbOS actor.
    ///
    /// These methods are only callable by ArbOS itself (address 0xA4B05).
    /// They are used internally by the sequencer to mark block boundaries
    /// and report batch posting costs. Exposed here for calldata decoding
    /// of `ArbitrumInternalTx` payloads.
    ///
    /// Nitro reference: `nitro/precompiles/ArbosActs.go`.
    #[sol(rpc)]
    interface ArbosActs {
        /// Marks the start of a new L2 block.
        function startBlock(
            uint256 l1BaseFee,
            uint64 l1BlockNumber,
            uint64 l2BlockNumber,
            uint64 timeLastBlock
        ) external;

        /// Reports batch posting costs (legacy V1 format).
        function batchPostingReport(
            uint256 batchTimestamp,
            address batchPosterAddress,
            uint64 batchNumber,
            uint64 batchGas,
            uint256 l1BaseFeeWei
        ) external;

        /// Reports batch posting costs (V2 format with detailed calldata stats).
        function batchPostingReportV2(
            uint256 batchTimestamp,
            address batchPosterAddress,
            uint64 batchNumber,
            uint64 batchCallDataLength,
            uint64 batchCallDataNonZeros,
            uint64 batchExtraGas,
            uint256 l1BaseFeeWei
        ) external;
    }
}
