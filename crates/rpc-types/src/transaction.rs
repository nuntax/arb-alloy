use alloy_consensus::Transaction as TransactionTrait;
use alloy_eips::{Typed2718, eip2930::AccessList, eip7702::SignedAuthorization};
use alloy_network_primitives::TransactionResponse;
use alloy_primitives::{Address, BlockHash, Bytes, ChainId, TxKind, B256, U256};
use serde::{Deserialize, Serialize};

use arb_alloy_consensus::ArbTxEnvelope;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbTransaction {
    #[serde(flatten)]
    pub inner: alloy_rpc_types_eth::Transaction<ArbTxEnvelope>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<B256>,
}

impl AsRef<ArbTxEnvelope> for ArbTransaction {
    fn as_ref(&self) -> &ArbTxEnvelope {
        self.inner.as_ref()
    }
}

impl TransactionTrait for ArbTransaction {
    fn chain_id(&self) -> Option<ChainId> {
        self.inner.chain_id()
    }

    fn nonce(&self) -> u64 {
        self.inner.nonce()
    }

    fn gas_limit(&self) -> u64 {
        self.inner.gas_limit()
    }

    fn gas_price(&self) -> Option<u128> {
        TransactionTrait::gas_price(&self.inner)
    }

    fn max_fee_per_gas(&self) -> u128 {
        TransactionTrait::max_fee_per_gas(&self.inner)
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_priority_fee_per_gas()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        self.inner.max_fee_per_blob_gas()
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.inner.priority_fee_or_price()
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        self.inner.effective_gas_price(base_fee)
    }

    fn is_dynamic_fee(&self) -> bool {
        self.inner.is_dynamic_fee()
    }

    fn kind(&self) -> TxKind {
        self.inner.kind()
    }

    fn is_create(&self) -> bool {
        self.inner.is_create()
    }

    fn value(&self) -> U256 {
        self.inner.value()
    }

    fn input(&self) -> &Bytes {
        self.inner.input()
    }

    fn access_list(&self) -> Option<&AccessList> {
        self.inner.access_list()
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        self.inner.blob_versioned_hashes()
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        self.inner.authorization_list()
    }
}

impl TransactionResponse for ArbTransaction {
    fn tx_hash(&self) -> B256 {
        self.inner.tx_hash()
    }

    fn block_hash(&self) -> Option<BlockHash> {
        self.inner.block_hash
    }

    fn block_number(&self) -> Option<u64> {
        self.inner.block_number
    }

    fn transaction_index(&self) -> Option<u64> {
        self.inner.transaction_index
    }

    fn from(&self) -> Address {
        self.inner.from()
    }
}

impl Typed2718 for ArbTransaction {
    fn ty(&self) -> u8 {
        self.inner.ty()
    }
}
