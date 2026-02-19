use alloy_consensus::{TxEip1559, TxEip2930, TxEip7702, TxLegacy};

use crate::transactions::{
    ArbTxEnvelope, TxContract, TxDeposit, TxRetry, TxUnsigned, internal::ArbInternalTx,
    submit_retryable::SubmitRetryableTx,
};

/// Unsigned transaction variants supported by the Arbitrum network implementation.
#[derive(Debug, Clone)]
pub enum ArbitrumTypedTransaction {
    /// Legacy Ethereum transaction.
    Legacy(TxLegacy),
    /// EIP-2930 transaction.
    Eip2930(TxEip2930),
    /// EIP-1559 transaction.
    Eip1559(TxEip1559),
    /// EIP-7702 transaction.
    Eip7702(TxEip7702),
    /// Arbitrum deposit transaction.
    Deposit(TxDeposit),
    /// Arbitrum submit-retryable transaction.
    SubmitRetryable(SubmitRetryableTx),
    /// Arbitrum unsigned user transaction.
    Unsigned(TxUnsigned),
    /// Arbitrum contract transaction.
    Contract(TxContract),
    /// Arbitrum retry transaction.
    Retry(TxRetry),
    /// Arbitrum internal system transaction.
    Internal(ArbInternalTx),
}

impl From<ArbTxEnvelope> for ArbitrumTypedTransaction {
    fn from(envelope: ArbTxEnvelope) -> Self {
        match envelope {
            ArbTxEnvelope::Legacy(tx) => Self::Legacy(tx.tx().clone()),
            ArbTxEnvelope::Eip2930(tx) => Self::Eip2930(tx.tx().clone()),
            ArbTxEnvelope::Eip1559(tx) => Self::Eip1559(tx.tx().clone()),
            ArbTxEnvelope::Eip7702(tx) => Self::Eip7702(tx.tx().clone()),
            ArbTxEnvelope::Deposit(tx) => Self::Deposit(tx.clone_inner()),
            ArbTxEnvelope::SubmitRetryable(tx) => Self::SubmitRetryable(tx.clone_inner()),
            ArbTxEnvelope::Unsigned(tx) => Self::Unsigned(tx.clone_inner()),
            ArbTxEnvelope::Contract(tx) => Self::Contract(tx.clone_inner()),
            ArbTxEnvelope::Retry(tx) => Self::Retry(tx.clone_inner()),
            ArbTxEnvelope::Internal(tx) => Self::Internal(tx.clone_inner()),
        }
    }
}
