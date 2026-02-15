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
/// Batch posting report decoder utilities.
pub mod batchpostingreport;
/// Arbitrum contract transaction type (`0x66`).
pub mod contract;
/// Arbitrum deposit transaction type (`0x64`).
pub mod deposit;
/// Arbitrum internal system transaction type (`0x6a`).
pub mod internal;
/// Arbitrum retry transaction type (`0x68`).
pub mod retry;
/// Arbitrum submit-retryable transaction type (`0x69`).
pub mod submit_retryable;
/// Typed unsigned transaction enum used by builders.
pub mod typed;
/// Arbitrum unsigned transaction type (`0x65`).
pub mod unsigned;
/// Shared decode helpers for sequencer-originated payloads.
pub mod util;

#[cfg(test)]
mod nitro_hash_tests;

/// Arbitrum transaction envelope that includes Ethereum and Nitro transaction variants.
#[derive(Debug, Clone, TransactionEnvelope)]
#[envelope(tx_type_name = ArbTxType)]
pub enum ArbTxEnvelope {
    /// Legacy Ethereum signed transaction.
    #[envelope(ty = 0)]
    Legacy(Signed<TxLegacy>),
    /// EIP-2930 signed transaction.
    #[envelope(ty = 1)]
    Eip2930(Signed<TxEip2930>),
    /// EIP-1559 signed transaction.
    #[envelope(ty = 2)]
    Eip1559(Signed<TxEip1559>),
    /// EIP-7702 signed transaction.
    #[envelope(ty = 4)]
    Eip7702(Signed<TxEip7702>),
    /// Arbitrum deposit transaction.
    #[envelope(ty = 0x64)]
    DepositTx(Sealed<TxDeposit>),
    /// Arbitrum submit-retryable transaction.
    #[envelope(ty = 0x69)]
    SubmitRetryableTx(Sealed<SubmitRetryableTx>),
    /// Arbitrum unsigned user transaction.
    #[envelope(ty = 0x65)]
    Unsigned(Sealed<TxUnsigned>),
    /// Arbitrum contract transaction.
    #[envelope(ty = 0x66)]
    Contract(Sealed<TxContract>),
    /// Arbitrum retry transaction.
    #[envelope(ty = 0x68)]
    Retry(Sealed<TxRetry>),
    /// Arbitrum internal system transaction.
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
