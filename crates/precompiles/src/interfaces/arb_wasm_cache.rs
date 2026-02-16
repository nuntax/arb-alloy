alloy_core::sol! {
    /// ArbWasmCache: Stylus WASM cache management.
    ///
    /// Nitro reference: `nitro/precompiles/ArbWasmCache.go`.
    #[sol(rpc)]
    interface ArbWasmCache {
        /// Returns true if the account is a cache manager.
        function isCacheManager(address account) external view returns (bool);

        /// Returns all registered cache managers.
        function allCacheManagers() external view returns (address[] memory);

        /// Returns true if the codehash is currently cached.
        function codehashIsCached(bytes32 codehash) external view returns (bool);

        /// Caches a Stylus program by address.
        function cacheProgram(address program) external;

        /// Evicts a cached codehash.
        function evictCodehash(bytes32 codehash) external;
    }
}
