alloy_core::sol! {
    /// ArbRetryableTx: retryable ticket management.
    ///
    /// Provides methods to redeem, cancel, and inspect retryable tickets
    /// created via L1→L2 messaging.
    ///
    /// Nitro reference: `nitro/precompiles/ArbRetryableTx.go`.
    #[sol(rpc)]
    interface ArbRetryableTx {
        /// Returns the default lifetime of a retryable ticket in seconds.
        function getLifetime() external view returns (uint256);

        /// Returns the timeout timestamp for a retryable ticket.
        function getTimeout(bytes32 ticketId) external view returns (uint256);

        /// Returns the beneficiary address for a retryable ticket.
        function getBeneficiary(bytes32 ticketId) external view returns (address);

        /// Returns the address currently redeeming a ticket (zero if none).
        function getCurrentRedeemer() external view returns (address);

        /// Redeems a retryable ticket, executing its stored transaction.
        /// Returns the transaction ID of the retry attempt.
        function redeem(bytes32 ticketId) external returns (bytes32);

        /// Extends the timeout of a retryable ticket by one lifetime period.
        /// Returns the new timeout timestamp.
        function keepalive(bytes32 ticketId) external payable returns (uint256);

        /// Cancels a retryable ticket and refunds remaining value to the beneficiary.
        function cancel(bytes32 ticketId) external;

        /// Emitted when a retryable ticket is successfully redeemed.
        event TicketRedeemed(bytes32 indexed ticketId);

        /// Emitted when a retryable ticket is created.
        event TicketCreated(bytes32 indexed ticketId);

        /// Emitted when a retryable ticket is canceled.
        event Canceled(bytes32 indexed ticketId);

        /// Emitted when a retryable ticket's lifetime is extended.
        event LifetimeExtended(bytes32 indexed ticketId, uint256 newTimeout);
    }
}
