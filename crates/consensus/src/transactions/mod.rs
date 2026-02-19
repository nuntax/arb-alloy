use core::fmt::Display;

use alloy_consensus::{
    Sealed, Signed, TransactionEnvelope, TxEip1559, TxEip2930, TxEip7702, TxLegacy,
};
use alloy_primitives::{Address, Sealable, TxHash};
pub use contract::TxContract;
pub use deposit::TxDeposit;
pub use retry::TxRetry;
pub use unsigned::TxUnsigned;

use crate::transactions::{internal::ArbInternalTx, submit_retryable::SubmitRetryableTx};
/// Batch posting report decoder utilities. Unused at the moment, remainder from sequencer reader.
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
    Deposit(Sealed<TxDeposit>),
    /// Arbitrum submit-retryable transaction.
    #[envelope(ty = 0x69)]
    SubmitRetryable(Sealed<SubmitRetryableTx>),
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
    Internal(Sealed<ArbInternalTx>),
}

impl ArbTxEnvelope {
    /// Returns the transaction hash.
    pub fn hash(&self) -> TxHash {
        match self {
            Self::Legacy(tx) => *tx.hash(),
            Self::Eip2930(tx) => *tx.hash(),
            Self::Eip1559(tx) => *tx.hash(),
            Self::Eip7702(tx) => *tx.hash(),
            Self::SubmitRetryable(tx) => tx.hash(),
            Self::Deposit(tx) => tx.hash(),
            Self::Unsigned(tx) => tx.hash(),
            Self::Contract(tx) => tx.hash(),
            Self::Retry(tx) => tx.hash(),
            Self::Internal(tx) => tx.hash(),
        }
    }
    /// Recover the sender address.
    pub fn sender(&self) -> Result<Address, alloy_primitives::SignatureError> {
        match self {
            Self::Legacy(tx) => tx.recover_signer(),
            Self::Eip2930(tx) => tx.recover_signer(),
            Self::Eip1559(tx) => tx.recover_signer(),
            Self::Eip7702(tx) => tx.recover_signer(),
            Self::SubmitRetryable(tx) => Ok(tx.from()),
            Self::Deposit(tx) => Ok(tx.from()),
            Self::Unsigned(tx) => Ok(tx.from()),
            Self::Contract(tx) => Ok(tx.from()),
            Self::Retry(tx) => Ok(tx.from()),
            Self::Internal(tx) => Ok(tx.from()),
        }
    }
}

impl From<ArbInternalTx> for ArbTxEnvelope {
    fn from(tx: ArbInternalTx) -> Self {
        Self::Internal(tx.seal_slow())
    }
}
impl From<TxDeposit> for ArbTxEnvelope {
    fn from(tx: TxDeposit) -> Self {
        Self::Deposit(tx.seal_slow())
    }
}
impl From<SubmitRetryableTx> for ArbTxEnvelope {
    fn from(tx: SubmitRetryableTx) -> Self {
        Self::SubmitRetryable(tx.seal_slow())
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Legacy => write!(f, "Legacy"),
            Self::Eip2930 => write!(f, "EIP-2930"),
            Self::Eip1559 => write!(f, "EIP-1559"),
            Self::Eip7702 => write!(f, "EIP-7702"),
            Self::Deposit => write!(f, "Deposit"),
            Self::SubmitRetryable => write!(f, "SubmitRetryable"),
            Self::Unsigned => write!(f, "Unsigned"),
            Self::Contract => write!(f, "Contract"),
            Self::Retry => write!(f, "Retry"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}
