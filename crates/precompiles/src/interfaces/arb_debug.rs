alloy_core::sol! {
    /// ArbDebug: debug-only precompile.
    ///
    /// Only available on debug/dev nodes. Not accessible in production.
    ///
    /// Nitro reference: `nitro/precompiles/ArbDebug.go`.
    #[sol(rpc)]
    interface ArbDebug {
        function events(bool flag, bytes32 value)
            external
            payable
            returns (address, uint256);

        function eventsView() external view;

        function customRevert(uint64 number) external pure;

        function becomeChainOwner() external;

        function overwriteContractCode(address account, bytes calldata code)
            external
            returns (bytes memory);

        function panic() external pure;

        function legacyError() external pure;

        event Basic(bool flag, bytes32 indexed value);
        event Mixed(
            bool indexed flag,
            bool not,
            bytes32 indexed value,
            address indexed conn,
            address caller
        );
        event Store(
            bool indexed flag,
            address indexed field,
            uint24 number,
            bytes32 value,
            bytes store
        );
    }
}
