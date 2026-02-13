use std::fmt::Display;

use alloy_consensus::{
    Sealed, Signed, TransactionEnvelope, TxEip1559, TxEip2930, TxEip7702, TxLegacy,
};
use alloy_primitives::{Address, Sealable, TxHash};
pub use contract::TxContract;
pub use deposit::TxDeposit;
pub use retry::TxRetry;
pub use unsigned::TxUnsigned;

use crate::transactions::{internal::ArbitrumInternalTx, submit_retryable::SubmitRetryableTx};
pub mod batchpostingreport;
pub mod contract;
pub mod deposit;
pub mod internal;
pub mod retry;
pub mod retryable;
pub mod submit_retryable;
pub mod typed;
pub mod unsigned;
pub mod util;

#[cfg(test)]
mod nitro_hash_tests;
#[derive(Debug, Clone, TransactionEnvelope)]
#[envelope(tx_type_name = ArbTxType)]
pub enum ArbTxEnvelope {
    #[envelope(ty = 0)]
    Legacy(Signed<TxLegacy>),
    #[envelope(ty = 1)]
    Eip2930(Signed<TxEip2930>),
    #[envelope(ty = 2)]
    Eip1559(Signed<TxEip1559>),
    #[envelope(ty = 4)]
    Eip7702(Signed<TxEip7702>),
    #[envelope(ty = 0x64)]
    DepositTx(Sealed<TxDeposit>),
    #[envelope(ty = 0x69)]
    SubmitRetryableTx(Sealed<SubmitRetryableTx>),
    #[envelope(ty = 0x65)]
    Unsigned(Sealed<TxUnsigned>),
    #[envelope(ty = 0x66)]
    Contract(Sealed<TxContract>),
    #[envelope(ty = 0x68)]
    Retry(Sealed<TxRetry>),
    #[envelope(ty = 0x6a)]
    ArbitrumInternal(Sealed<ArbitrumInternalTx>),
}

impl ArbTxEnvelope {
    /// Returns the transaction type.
    pub fn hash(&self) -> TxHash {
        match self {
            ArbTxEnvelope::Legacy(tx) => *tx.hash(),
            ArbTxEnvelope::Eip2930(tx) => *tx.hash(),
            ArbTxEnvelope::Eip1559(tx) => *tx.hash(),
            ArbTxEnvelope::Eip7702(tx) => *tx.hash(),
            ArbTxEnvelope::SubmitRetryableTx(tx) => tx.hash(),
            ArbTxEnvelope::DepositTx(tx) => tx.hash(),
            ArbTxEnvelope::Unsigned(tx) => tx.hash(),
            ArbTxEnvelope::Contract(tx) => tx.hash(),
            ArbTxEnvelope::Retry(tx) => tx.hash(),
            ArbTxEnvelope::ArbitrumInternal(tx) => tx.hash(),
        }
    }
    /// Recover the sender address.
    pub fn sender(&self) -> Result<Address, alloy_primitives::SignatureError> {
        match self {
            ArbTxEnvelope::Legacy(tx) => tx.recover_signer(),
            ArbTxEnvelope::Eip2930(tx) => tx.recover_signer(),
            ArbTxEnvelope::Eip1559(tx) => tx.recover_signer(),
            ArbTxEnvelope::Eip7702(tx) => tx.recover_signer(),
            ArbTxEnvelope::SubmitRetryableTx(tx) => Ok(tx.from()),
            ArbTxEnvelope::DepositTx(tx) => Ok(tx.from()),
            ArbTxEnvelope::Unsigned(tx) => Ok(tx.from()),
            ArbTxEnvelope::Contract(tx) => Ok(tx.from()),
            ArbTxEnvelope::Retry(tx) => Ok(tx.from()),
            ArbTxEnvelope::ArbitrumInternal(tx) => Ok(tx.from()),
        }
    }
}

impl From<ArbitrumInternalTx> for ArbTxEnvelope {
    fn from(tx: ArbitrumInternalTx) -> Self {
        ArbTxEnvelope::ArbitrumInternal(tx.seal_slow())
    }
}
impl From<TxDeposit> for ArbTxEnvelope {
    fn from(tx: TxDeposit) -> Self {
        ArbTxEnvelope::DepositTx(tx.seal_slow())
    }
}
impl From<SubmitRetryableTx> for ArbTxEnvelope {
    fn from(tx: SubmitRetryableTx) -> Self {
        ArbTxEnvelope::SubmitRetryableTx(tx.seal_slow())
    }
}
impl From<TxUnsigned> for ArbTxEnvelope {
    fn from(tx: TxUnsigned) -> Self {
        ArbTxEnvelope::Unsigned(tx.seal_slow())
    }
}
impl From<TxContract> for ArbTxEnvelope {
    fn from(tx: TxContract) -> Self {
        ArbTxEnvelope::Contract(tx.seal_slow())
    }
}
impl From<TxRetry> for ArbTxEnvelope {
    fn from(tx: TxRetry) -> Self {
        ArbTxEnvelope::Retry(tx.seal_slow())
    }
}
impl From<Signed<TxLegacy>> for ArbTxEnvelope {
    fn from(tx: Signed<TxLegacy>) -> Self {
        ArbTxEnvelope::Legacy(tx)
    }
}
impl From<Signed<TxEip2930>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip2930>) -> Self {
        ArbTxEnvelope::Eip2930(tx)
    }
}
impl From<Signed<TxEip1559>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip1559>) -> Self {
        ArbTxEnvelope::Eip1559(tx)
    }
}
impl From<Signed<TxEip7702>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip7702>) -> Self {
        ArbTxEnvelope::Eip7702(tx)
    }
}

impl Display for ArbTxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArbTxType::Legacy => write!(f, "Legacy"),
            ArbTxType::Eip2930 => write!(f, "EIP-2930"),
            ArbTxType::Eip1559 => write!(f, "EIP-1559"),
            ArbTxType::Eip7702 => write!(f, "EIP-7702"),
            ArbTxType::DepositTx => write!(f, "DepositTx"),
            ArbTxType::SubmitRetryableTx => write!(f, "SubmitRetryableTx"),
            ArbTxType::Unsigned => write!(f, "Unsigned"),
            ArbTxType::Contract => write!(f, "Contract"),
            ArbTxType::Retry => write!(f, "Retry"),
            ArbTxType::ArbitrumInternal => write!(f, "ArbitrumInternal"),
        }
    }
}
