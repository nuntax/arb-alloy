use alloy_consensus::{TxEip1559, TxEip2930, TxEip7702, TxLegacy};

use crate::transactions::{
    ArbTxEnvelope, TxContract, TxDeposit, TxRetry, TxUnsigned,
    internal::ArbitrumInternalTx, submit_retryable::SubmitRetryableTx,
};

pub enum ArbitrumTypedTransaction {
    Legacy(TxLegacy),
    Eip2930(TxEip2930),
    Eip1559(TxEip1559),
    Eip7702(TxEip7702),
    DepositTx(TxDeposit),
    SubmitRetryableTx(SubmitRetryableTx),
    Unsigned(TxUnsigned),
    Contract(TxContract),
    Retry(TxRetry),
    ArbitrumInternal(ArbitrumInternalTx),
}

impl From<ArbTxEnvelope> for ArbitrumTypedTransaction {
    fn from(envelope: ArbTxEnvelope) -> Self {
        match envelope {
            ArbTxEnvelope::Legacy(tx) => Self::Legacy(tx.tx().clone()),
            ArbTxEnvelope::Eip2930(tx) => Self::Eip2930(tx.tx().clone()),
            ArbTxEnvelope::Eip1559(tx) => Self::Eip1559(tx.tx().clone()),
            ArbTxEnvelope::Eip7702(tx) => Self::Eip7702(tx.tx().clone()),
            ArbTxEnvelope::DepositTx(tx) => Self::DepositTx(tx.clone_inner()),
            ArbTxEnvelope::SubmitRetryableTx(tx) => Self::SubmitRetryableTx(tx.clone_inner()),
            ArbTxEnvelope::Unsigned(tx) => Self::Unsigned(tx.clone_inner()),
            ArbTxEnvelope::Contract(tx) => Self::Contract(tx.clone_inner()),
            ArbTxEnvelope::Retry(tx) => Self::Retry(tx.clone_inner()),
            ArbTxEnvelope::ArbitrumInternal(tx) => Self::ArbitrumInternal(tx.clone_inner()),
        }
    }
}
