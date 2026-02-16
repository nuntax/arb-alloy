alloy_core::sol! {
    /// ArbAddressTable: address compression table for calldata optimization.
    ///
    /// Nitro reference: `nitro/precompiles/ArbAddressTable.go`.
    #[sol(rpc)]
    interface ArbAddressTable {
        /// Checks if an address is registered in the table.
        function addressExists(address account) external view returns (bool);

        /// Compresses an address using the table.
        function compress(address account) external view returns (bytes memory);

        /// Looks up an address and returns its index.
        function lookup(address account) external view returns (uint256);

        /// Looks up an index and returns the corresponding address.
        function lookupIndex(uint256 index) external view returns (address);

        /// Returns the number of addresses in the table.
        function size() external view returns (uint256);

        /// Decompresses an address from a buffer at the given offset.
        /// Returns (decompressed address, bytes consumed).
        function decompress(bytes calldata buf, uint256 offset)
            external
            returns (address, uint256);

        /// Registers an address in the table. Returns the assigned index.
        function register(address account) external returns (uint256);
    }
}
