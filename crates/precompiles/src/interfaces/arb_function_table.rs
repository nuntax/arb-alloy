alloy_core::sol! {
    /// ArbFunctionTable: function table for classic Arbitrum contracts.
    ///
    /// Largely vestigial post-Nitro; kept for backward compatibility.
    ///
    /// Nitro reference: `nitro/precompiles/ArbFunctionTable.go`.
    #[sol(rpc)]
    interface ArbFunctionTable {
        /// Returns the function table size for an account.
        function size(address account) external view returns (uint256);

        /// Uploads a function table.
        function upload(bytes calldata buf) external;

        /// Returns function table entry: (functionSize, isPayable, functionId).
        function get(address account, uint256 index)
            external
            view
            returns (uint256, bool, uint256);
    }
}
