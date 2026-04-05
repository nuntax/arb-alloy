use alloy_consensus::{
    SignableTransaction, Signed, Transaction, TxEip1559, TxEip2930, TxEip7702, TxLegacy, Typed2718,
};
use alloy_eips::{eip2930::AccessList, eip7702::SignedAuthorization};
use alloy_primitives::{B256, Bytes, ChainId, Selector, Signature, TxKind, U256};
use bytes::BufMut;

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

impl Typed2718 for ArbitrumTypedTransaction {
    fn ty(&self) -> u8 {
        match self {
            Self::Legacy(tx) => tx.ty(),
            Self::Eip2930(tx) => tx.ty(),
            Self::Eip1559(tx) => tx.ty(),
            Self::Eip7702(tx) => tx.ty(),
            Self::Deposit(tx) => tx.ty(),
            Self::SubmitRetryable(tx) => tx.ty(),
            Self::Unsigned(tx) => tx.ty(),
            Self::Contract(tx) => tx.ty(),
            Self::Retry(tx) => tx.ty(),
            Self::Internal(tx) => tx.ty(),
        }
    }
}

impl Transaction for ArbitrumTypedTransaction {
    fn chain_id(&self) -> Option<ChainId> {
        match self {
            Self::Legacy(tx) => tx.chain_id(),
            Self::Eip2930(tx) => tx.chain_id(),
            Self::Eip1559(tx) => tx.chain_id(),
            Self::Eip7702(tx) => tx.chain_id(),
            Self::Deposit(tx) => tx.chain_id(),
            Self::SubmitRetryable(tx) => tx.chain_id(),
            Self::Unsigned(tx) => tx.chain_id(),
            Self::Contract(tx) => tx.chain_id(),
            Self::Retry(tx) => tx.chain_id(),
            Self::Internal(tx) => tx.chain_id(),
        }
    }

    fn nonce(&self) -> u64 {
        match self {
            Self::Legacy(tx) => tx.nonce(),
            Self::Eip2930(tx) => tx.nonce(),
            Self::Eip1559(tx) => tx.nonce(),
            Self::Eip7702(tx) => tx.nonce(),
            Self::Deposit(tx) => tx.nonce(),
            Self::SubmitRetryable(tx) => tx.nonce(),
            Self::Unsigned(tx) => tx.nonce(),
            Self::Contract(tx) => tx.nonce(),
            Self::Retry(tx) => tx.nonce(),
            Self::Internal(tx) => tx.nonce(),
        }
    }

    fn gas_limit(&self) -> u64 {
        match self {
            Self::Legacy(tx) => tx.gas_limit(),
            Self::Eip2930(tx) => tx.gas_limit(),
            Self::Eip1559(tx) => tx.gas_limit(),
            Self::Eip7702(tx) => tx.gas_limit(),
            Self::Deposit(tx) => tx.gas_limit(),
            Self::SubmitRetryable(tx) => tx.gas_limit(),
            Self::Unsigned(tx) => tx.gas_limit(),
            Self::Contract(tx) => tx.gas_limit(),
            Self::Retry(tx) => tx.gas_limit(),
            Self::Internal(tx) => tx.gas_limit(),
        }
    }

    fn gas_price(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.gas_price(),
            Self::Eip2930(tx) => tx.gas_price(),
            Self::Eip1559(tx) => tx.gas_price(),
            Self::Eip7702(tx) => tx.gas_price(),
            Self::Deposit(tx) => tx.gas_price(),
            Self::SubmitRetryable(tx) => tx.gas_price(),
            Self::Unsigned(tx) => tx.gas_price(),
            Self::Contract(tx) => tx.gas_price(),
            Self::Retry(tx) => tx.gas_price(),
            Self::Internal(tx) => tx.gas_price(),
        }
    }

    fn max_fee_per_gas(&self) -> u128 {
        match self {
            Self::Legacy(tx) => tx.max_fee_per_gas(),
            Self::Eip2930(tx) => tx.max_fee_per_gas(),
            Self::Eip1559(tx) => tx.max_fee_per_gas(),
            Self::Eip7702(tx) => tx.max_fee_per_gas(),
            Self::Deposit(tx) => tx.max_fee_per_gas(),
            Self::SubmitRetryable(tx) => tx.max_fee_per_gas(),
            Self::Unsigned(tx) => tx.max_fee_per_gas(),
            Self::Contract(tx) => tx.max_fee_per_gas(),
            Self::Retry(tx) => tx.max_fee_per_gas(),
            Self::Internal(tx) => tx.max_fee_per_gas(),
        }
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip2930(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip1559(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip7702(tx) => tx.max_priority_fee_per_gas(),
            Self::Deposit(tx) => tx.max_priority_fee_per_gas(),
            Self::SubmitRetryable(tx) => tx.max_priority_fee_per_gas(),
            Self::Unsigned(tx) => tx.max_priority_fee_per_gas(),
            Self::Contract(tx) => tx.max_priority_fee_per_gas(),
            Self::Retry(tx) => tx.max_priority_fee_per_gas(),
            Self::Internal(tx) => tx.max_priority_fee_per_gas(),
        }
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip2930(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip1559(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip7702(tx) => tx.max_fee_per_blob_gas(),
            Self::Deposit(tx) => tx.max_fee_per_blob_gas(),
            Self::SubmitRetryable(tx) => tx.max_fee_per_blob_gas(),
            Self::Unsigned(tx) => tx.max_fee_per_blob_gas(),
            Self::Contract(tx) => tx.max_fee_per_blob_gas(),
            Self::Retry(tx) => tx.max_fee_per_blob_gas(),
            Self::Internal(tx) => tx.max_fee_per_blob_gas(),
        }
    }

    fn priority_fee_or_price(&self) -> u128 {
        match self {
            Self::Legacy(tx) => tx.priority_fee_or_price(),
            Self::Eip2930(tx) => tx.priority_fee_or_price(),
            Self::Eip1559(tx) => tx.priority_fee_or_price(),
            Self::Eip7702(tx) => tx.priority_fee_or_price(),
            Self::Deposit(tx) => tx.priority_fee_or_price(),
            Self::SubmitRetryable(tx) => tx.priority_fee_or_price(),
            Self::Unsigned(tx) => tx.priority_fee_or_price(),
            Self::Contract(tx) => tx.priority_fee_or_price(),
            Self::Retry(tx) => tx.priority_fee_or_price(),
            Self::Internal(tx) => tx.priority_fee_or_price(),
        }
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        match self {
            Self::Legacy(tx) => tx.effective_gas_price(base_fee),
            Self::Eip2930(tx) => tx.effective_gas_price(base_fee),
            Self::Eip1559(tx) => tx.effective_gas_price(base_fee),
            Self::Eip7702(tx) => tx.effective_gas_price(base_fee),
            Self::Deposit(tx) => tx.effective_gas_price(base_fee),
            Self::SubmitRetryable(tx) => tx.effective_gas_price(base_fee),
            Self::Unsigned(tx) => tx.effective_gas_price(base_fee),
            Self::Contract(tx) => tx.effective_gas_price(base_fee),
            Self::Retry(tx) => tx.effective_gas_price(base_fee),
            Self::Internal(tx) => tx.effective_gas_price(base_fee),
        }
    }

    fn is_dynamic_fee(&self) -> bool {
        match self {
            Self::Legacy(tx) => tx.is_dynamic_fee(),
            Self::Eip2930(tx) => tx.is_dynamic_fee(),
            Self::Eip1559(tx) => tx.is_dynamic_fee(),
            Self::Eip7702(tx) => tx.is_dynamic_fee(),
            Self::Deposit(tx) => tx.is_dynamic_fee(),
            Self::SubmitRetryable(tx) => tx.is_dynamic_fee(),
            Self::Unsigned(tx) => tx.is_dynamic_fee(),
            Self::Contract(tx) => tx.is_dynamic_fee(),
            Self::Retry(tx) => tx.is_dynamic_fee(),
            Self::Internal(tx) => tx.is_dynamic_fee(),
        }
    }

    fn kind(&self) -> TxKind {
        match self {
            Self::Legacy(tx) => tx.kind(),
            Self::Eip2930(tx) => tx.kind(),
            Self::Eip1559(tx) => tx.kind(),
            Self::Eip7702(tx) => tx.kind(),
            Self::Deposit(tx) => tx.kind(),
            Self::SubmitRetryable(tx) => tx.kind(),
            Self::Unsigned(tx) => tx.kind(),
            Self::Contract(tx) => tx.kind(),
            Self::Retry(tx) => tx.kind(),
            Self::Internal(tx) => tx.kind(),
        }
    }

    fn is_create(&self) -> bool {
        match self {
            Self::Legacy(tx) => tx.is_create(),
            Self::Eip2930(tx) => tx.is_create(),
            Self::Eip1559(tx) => tx.is_create(),
            Self::Eip7702(tx) => tx.is_create(),
            Self::Deposit(tx) => tx.is_create(),
            Self::SubmitRetryable(tx) => tx.is_create(),
            Self::Unsigned(tx) => tx.is_create(),
            Self::Contract(tx) => tx.is_create(),
            Self::Retry(tx) => tx.is_create(),
            Self::Internal(tx) => tx.is_create(),
        }
    }

    fn value(&self) -> U256 {
        match self {
            Self::Legacy(tx) => tx.value(),
            Self::Eip2930(tx) => tx.value(),
            Self::Eip1559(tx) => tx.value(),
            Self::Eip7702(tx) => tx.value(),
            Self::Deposit(tx) => tx.value(),
            Self::SubmitRetryable(tx) => tx.value(),
            Self::Unsigned(tx) => tx.value(),
            Self::Contract(tx) => tx.value(),
            Self::Retry(tx) => tx.value(),
            Self::Internal(tx) => tx.value(),
        }
    }

    fn input(&self) -> &Bytes {
        match self {
            Self::Legacy(tx) => tx.input(),
            Self::Eip2930(tx) => tx.input(),
            Self::Eip1559(tx) => tx.input(),
            Self::Eip7702(tx) => tx.input(),
            Self::Deposit(tx) => tx.input(),
            Self::SubmitRetryable(tx) => tx.input(),
            Self::Unsigned(tx) => tx.input(),
            Self::Contract(tx) => tx.input(),
            Self::Retry(tx) => tx.input(),
            Self::Internal(tx) => tx.input(),
        }
    }

    fn access_list(&self) -> Option<&AccessList> {
        match self {
            Self::Legacy(tx) => tx.access_list(),
            Self::Eip2930(tx) => tx.access_list(),
            Self::Eip1559(tx) => tx.access_list(),
            Self::Eip7702(tx) => tx.access_list(),
            Self::Deposit(tx) => tx.access_list(),
            Self::SubmitRetryable(tx) => tx.access_list(),
            Self::Unsigned(tx) => tx.access_list(),
            Self::Contract(tx) => tx.access_list(),
            Self::Retry(tx) => tx.access_list(),
            Self::Internal(tx) => tx.access_list(),
        }
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        match self {
            Self::Legacy(tx) => tx.blob_versioned_hashes(),
            Self::Eip2930(tx) => tx.blob_versioned_hashes(),
            Self::Eip1559(tx) => tx.blob_versioned_hashes(),
            Self::Eip7702(tx) => tx.blob_versioned_hashes(),
            Self::Deposit(tx) => tx.blob_versioned_hashes(),
            Self::SubmitRetryable(tx) => tx.blob_versioned_hashes(),
            Self::Unsigned(tx) => tx.blob_versioned_hashes(),
            Self::Contract(tx) => tx.blob_versioned_hashes(),
            Self::Retry(tx) => tx.blob_versioned_hashes(),
            Self::Internal(tx) => tx.blob_versioned_hashes(),
        }
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        match self {
            Self::Legacy(tx) => tx.authorization_list(),
            Self::Eip2930(tx) => tx.authorization_list(),
            Self::Eip1559(tx) => tx.authorization_list(),
            Self::Eip7702(tx) => tx.authorization_list(),
            Self::Deposit(tx) => tx.authorization_list(),
            Self::SubmitRetryable(tx) => tx.authorization_list(),
            Self::Unsigned(tx) => tx.authorization_list(),
            Self::Contract(tx) => tx.authorization_list(),
            Self::Retry(tx) => tx.authorization_list(),
            Self::Internal(tx) => tx.authorization_list(),
        }
    }

    fn effective_tip_per_gas(&self, base_fee: u64) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Eip2930(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Eip1559(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Eip7702(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Deposit(tx) => tx.effective_tip_per_gas(base_fee),
            Self::SubmitRetryable(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Unsigned(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Contract(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Retry(tx) => tx.effective_tip_per_gas(base_fee),
            Self::Internal(tx) => tx.effective_tip_per_gas(base_fee),
        }
    }

    fn to(&self) -> Option<alloy_primitives::Address> {
        match self {
            Self::Legacy(tx) => tx.to(),
            Self::Eip2930(tx) => tx.to(),
            Self::Eip1559(tx) => tx.to(),
            Self::Eip7702(tx) => tx.to(),
            Self::Deposit(tx) => tx.to(),
            Self::SubmitRetryable(tx) => tx.to(),
            Self::Unsigned(tx) => tx.to(),
            Self::Contract(tx) => tx.to(),
            Self::Retry(tx) => tx.to(),
            Self::Internal(tx) => tx.to(),
        }
    }

    fn function_selector(&self) -> Option<&Selector> {
        match self {
            Self::Legacy(tx) => tx.function_selector(),
            Self::Eip2930(tx) => tx.function_selector(),
            Self::Eip1559(tx) => tx.function_selector(),
            Self::Eip7702(tx) => tx.function_selector(),
            Self::Deposit(tx) => tx.function_selector(),
            Self::SubmitRetryable(tx) => tx.function_selector(),
            Self::Unsigned(tx) => tx.function_selector(),
            Self::Contract(tx) => tx.function_selector(),
            Self::Retry(tx) => tx.function_selector(),
            Self::Internal(tx) => tx.function_selector(),
        }
    }

    fn blob_count(&self) -> Option<u64> {
        match self {
            Self::Legacy(tx) => tx.blob_count(),
            Self::Eip2930(tx) => tx.blob_count(),
            Self::Eip1559(tx) => tx.blob_count(),
            Self::Eip7702(tx) => tx.blob_count(),
            Self::Deposit(tx) => tx.blob_count(),
            Self::SubmitRetryable(tx) => tx.blob_count(),
            Self::Unsigned(tx) => tx.blob_count(),
            Self::Contract(tx) => tx.blob_count(),
            Self::Retry(tx) => tx.blob_count(),
            Self::Internal(tx) => tx.blob_count(),
        }
    }

    fn blob_gas_used(&self) -> Option<u64> {
        match self {
            Self::Legacy(tx) => tx.blob_gas_used(),
            Self::Eip2930(tx) => tx.blob_gas_used(),
            Self::Eip1559(tx) => tx.blob_gas_used(),
            Self::Eip7702(tx) => tx.blob_gas_used(),
            Self::Deposit(tx) => tx.blob_gas_used(),
            Self::SubmitRetryable(tx) => tx.blob_gas_used(),
            Self::Unsigned(tx) => tx.blob_gas_used(),
            Self::Contract(tx) => tx.blob_gas_used(),
            Self::Retry(tx) => tx.blob_gas_used(),
            Self::Internal(tx) => tx.blob_gas_used(),
        }
    }

    fn authorization_count(&self) -> Option<u64> {
        match self {
            Self::Legacy(tx) => tx.authorization_count(),
            Self::Eip2930(tx) => tx.authorization_count(),
            Self::Eip1559(tx) => tx.authorization_count(),
            Self::Eip7702(tx) => tx.authorization_count(),
            Self::Deposit(tx) => tx.authorization_count(),
            Self::SubmitRetryable(tx) => tx.authorization_count(),
            Self::Unsigned(tx) => tx.authorization_count(),
            Self::Contract(tx) => tx.authorization_count(),
            Self::Retry(tx) => tx.authorization_count(),
            Self::Internal(tx) => tx.authorization_count(),
        }
    }
}

/// Allows the alloy `EthereumWallet` blanket impl to cover the `Arbitrum` network.
/// Only the Ethereum-compatible variants (Legacy, Eip2930, Eip1559, Eip7702) can be
/// user-signed; Arbitrum-specific types are L1-originated and will panic if reached.
impl SignableTransaction<Signature> for ArbitrumTypedTransaction {
    fn set_chain_id(&mut self, chain_id: ChainId) {
        match self {
            Self::Legacy(tx) => tx.set_chain_id(chain_id),
            Self::Eip2930(tx) => tx.set_chain_id(chain_id),
            Self::Eip1559(tx) => tx.set_chain_id(chain_id),
            Self::Eip7702(tx) => tx.set_chain_id(chain_id),
            _ => {}
        }
    }

    fn encode_for_signing(&self, out: &mut dyn BufMut) {
        match self {
            Self::Legacy(tx) => tx.encode_for_signing(out),
            Self::Eip2930(tx) => tx.encode_for_signing(out),
            Self::Eip1559(tx) => tx.encode_for_signing(out),
            Self::Eip7702(tx) => tx.encode_for_signing(out),
            _ => panic!("Arbitrum-specific transactions are not user-signable"),
        }
    }

    fn payload_len_for_signature(&self) -> usize {
        match self {
            Self::Legacy(tx) => tx.payload_len_for_signature(),
            Self::Eip2930(tx) => tx.payload_len_for_signature(),
            Self::Eip1559(tx) => tx.payload_len_for_signature(),
            Self::Eip7702(tx) => tx.payload_len_for_signature(),
            _ => panic!("Arbitrum-specific transactions are not user-signable"),
        }
    }
}

impl From<Signed<ArbitrumTypedTransaction>> for ArbTxEnvelope {
    fn from(value: Signed<ArbitrumTypedTransaction>) -> Self {
        let sig = *value.signature();
        match value.strip_signature() {
            ArbitrumTypedTransaction::Legacy(tx) => Self::Legacy(Signed::new_unhashed(tx, sig)),
            ArbitrumTypedTransaction::Eip2930(tx) => Self::Eip2930(Signed::new_unhashed(tx, sig)),
            ArbitrumTypedTransaction::Eip1559(tx) => Self::Eip1559(Signed::new_unhashed(tx, sig)),
            ArbitrumTypedTransaction::Eip7702(tx) => Self::Eip7702(Signed::new_unhashed(tx, sig)),
            _ => {
                panic!("Arbitrum-specific transactions cannot be converted from a signed envelope")
            }
        }
    }
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
