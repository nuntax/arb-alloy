alloy_core::sol! {
    /// ArbOwnerPublic: read-only chain owner queries (callable by anyone).
    ///
    /// Nitro reference: `nitro/precompiles/ArbOwnerPublic.go`.
    #[sol(rpc)]
    interface ArbOwnerPublic {
        function getAllChainOwners() external view returns (address[] memory);
        function isChainOwner(address account) external view returns (bool);

        function isNativeTokenOwner(address account) external view returns (bool);
        function getAllNativeTokenOwners() external view returns (address[] memory);
        function getNativeTokenManagementFrom() external view returns (uint64);

        function getTransactionFilteringFrom() external view returns (uint64);
        function isTransactionFilterer(address filterer) external view returns (bool);
        function getAllTransactionFilterers() external view returns (address[] memory);
        function getFilteredFundsRecipient() external view returns (address);

        function getNetworkFeeAccount() external view returns (address);
        function getInfraFeeAccount() external view returns (address);

        function getBrotliCompressionLevel() external view returns (uint64);
        function getScheduledUpgrade() external view returns (uint64, uint64);
        function isCalldataPriceIncreaseEnabled() external view returns (bool);
        function getParentGasFloorPerToken() external view returns (uint64);
        function getMaxStylusContractFragments() external view returns (uint8);

        function rectifyChainOwner(address account) external;
    }
}
