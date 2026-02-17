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
            Self::Legacy(tx) => *tx.hash(),
            Self::Eip2930(tx) => *tx.hash(),
            Self::Eip1559(tx) => *tx.hash(),
            Self::Eip7702(tx) => *tx.hash(),
            Self::SubmitRetryableTx(tx) => tx.hash(),
            Self::DepositTx(tx) => tx.hash(),
            Self::Unsigned(tx) => tx.hash(),
            Self::Contract(tx) => tx.hash(),
            Self::Retry(tx) => tx.hash(),
            Self::ArbitrumInternal(tx) => tx.hash(),
        }
    }
    /// Recover the sender address.
    pub fn sender(&self) -> Result<Address, alloy_primitives::SignatureError> {
        match self {
            Self::Legacy(tx) => tx.recover_signer(),
            Self::Eip2930(tx) => tx.recover_signer(),
            Self::Eip1559(tx) => tx.recover_signer(),
            Self::Eip7702(tx) => tx.recover_signer(),
            Self::SubmitRetryableTx(tx) => Ok(tx.from()),
            Self::DepositTx(tx) => Ok(tx.from()),
            Self::Unsigned(tx) => Ok(tx.from()),
            Self::Contract(tx) => Ok(tx.from()),
            Self::Retry(tx) => Ok(tx.from()),
            Self::ArbitrumInternal(tx) => Ok(tx.from()),
        }
    }
}

impl From<ArbitrumInternalTx> for ArbTxEnvelope {
    fn from(tx: ArbitrumInternalTx) -> Self {
        Self::ArbitrumInternal(tx.seal_slow())
    }
}
impl From<TxDeposit> for ArbTxEnvelope {
    fn from(tx: TxDeposit) -> Self {
        Self::DepositTx(tx.seal_slow())
    }
}
impl From<SubmitRetryableTx> for ArbTxEnvelope {
    fn from(tx: SubmitRetryableTx) -> Self {
        Self::SubmitRetryableTx(tx.seal_slow())
    }
}
impl From<TxUnsigned> for ArbTxEnvelope {
    fn from(tx: TxUnsigned) -> Self {
        Self::Unsigned(tx.seal_slow())
    }
}
impl From<TxContract> for ArbTxEnvelope {
    fn from(tx: TxContract) -> Self {
        Self::Contract(tx.seal_slow())
    }
}
impl From<TxRetry> for ArbTxEnvelope {
    fn from(tx: TxRetry) -> Self {
        Self::Retry(tx.seal_slow())
    }
}
impl From<Signed<TxLegacy>> for ArbTxEnvelope {
    fn from(tx: Signed<TxLegacy>) -> Self {
        Self::Legacy(tx)
    }
}
impl From<Signed<TxEip2930>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip2930>) -> Self {
        Self::Eip2930(tx)
    }
}
impl From<Signed<TxEip1559>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip1559>) -> Self {
        Self::Eip1559(tx)
    }
}
impl From<Signed<TxEip7702>> for ArbTxEnvelope {
    fn from(tx: Signed<TxEip7702>) -> Self {
        Self::Eip7702(tx)
    }
}

impl Display for ArbTxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legacy => write!(f, "Legacy"),
            Self::Eip2930 => write!(f, "EIP-2930"),
            Self::Eip1559 => write!(f, "EIP-1559"),
            Self::Eip7702 => write!(f, "EIP-7702"),
            Self::DepositTx => write!(f, "DepositTx"),
            Self::SubmitRetryableTx => write!(f, "SubmitRetryableTx"),
            Self::Unsigned => write!(f, "Unsigned"),
            Self::Contract => write!(f, "Contract"),
            Self::Retry => write!(f, "Retry"),
            Self::ArbitrumInternal => write!(f, "ArbitrumInternal"),
        }
    }
}
