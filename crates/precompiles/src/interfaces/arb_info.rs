alloy_core::sol! {
    /// ArbInfo: account balance and code queries.
    ///
    /// Nitro reference: `nitro/precompiles/ArbInfo.go`.
    #[sol(rpc)]
    interface ArbInfo {
        /// Returns the ETH balance of an account.
        function getBalance(address account) external view returns (uint256);

        /// Returns the deployed bytecode of an account.
        function getCode(address account) external view returns (bytes memory);
    }
}
