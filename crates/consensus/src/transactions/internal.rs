use alloc::vec::Vec;

use alloy_consensus::{Transaction, Typed2718};
use alloy_eips::{
    Decodable2718, Encodable2718,
    eip2718::{Eip2718Error, Eip2718Result},
    eip2930::AccessList,
    eip7702::SignedAuthorization,
};
use alloy_primitives::{
    Address, B256, Bytes, ChainId, Sealable, Selector, TxHash, TxKind, U256, address, keccak256,
};
use alloy_rlp::{Decodable, Encodable, Header};
use bytes::BufMut;
use serde::{Deserialize, Serialize};

use crate::transactions::ArbTxType;

/// Arbitrum internal system transaction (ArbOS).
///
/// Nitro encodes internal txs as a type-0x6a EIP-2718 envelope with an RLP list
/// of `[chain_id, data]`, where `data` is the ABI-encoded calldata for ArbOSActs
/// (e.g. `startBlock`, `batchPostingReport`, `batchPostingReportV2`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrumInternalTx {
    /// Arbitrum chain identifier.
    #[serde(with = "alloy_serde::quantity")]
    pub chain_id: ChainId,
    /// ArbOS calldata payload.
    #[serde(rename = "input", alias = "data")]
    pub data: Bytes,
}

impl ArbitrumInternalTx {
    /// Canonical ArbOS sender/recipient address for internal transactions.
    pub const ARBOS_ADDRESS: Address = address!("0x00000000000000000000000000000000000a4b05");

    /// Creates a new internal transaction.
    pub const fn new(chain_id: ChainId, data: Bytes) -> Self {
        Self { chain_id, data }
    }

    /// Returns the canonical internal transaction sender.
    pub const fn from(&self) -> Address {
        Self::ARBOS_ADDRESS
    }

    /// Computes the EIP-2718 transaction hash.
    pub fn tx_hash(&self) -> TxHash {
        let mut buf = Vec::with_capacity(self.encode_2718_len());
        self.encode_2718(&mut buf);
        keccak256(&buf)
    }

    /// Encodes the inner RLP fields (without list header or type byte).
    pub fn rlp_encode_fields(&self, out: &mut dyn BufMut) {
        self.chain_id.encode(out);
        self.data.encode(out);
    }

    /// Returns the encoded RLP payload length for the inner fields.
    pub fn rlp_encoded_fields_length(&self) -> usize {
        self.chain_id.length() + self.data.length()
    }

    /// Returns the RLP list header for the inner payload.
    pub fn rlp_header(&self) -> Header {
        Header {
            list: true,
            payload_length: self.rlp_encoded_fields_length(),
        }
    }

    /// Encodes the transaction in RLP list form (without type byte).
    pub fn rlp_encode(&self, out: &mut dyn BufMut) {
        self.rlp_header().encode(out);
        self.rlp_encode_fields(out);
    }

    fn rlp_encoded_length(&self) -> usize {
        self.rlp_header().length_with_payload()
    }

    /// Decodes the transaction from its RLP list form (without type byte).
    pub fn rlp_decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let header = Header::decode(buf)?;
        if !header.list {
            return Err(alloy_rlp::Error::Custom("Expected list header"));
        }
        let chain_id: ChainId = Decodable::decode(buf)?;
        let data: Bytes = Decodable::decode(buf)?;
        Ok(Self { chain_id, data })
    }
}

impl Typed2718 for ArbitrumInternalTx {
    fn ty(&self) -> u8 {
        ArbTxType::ArbitrumInternal as u8
    }
}

impl Decodable for ArbitrumInternalTx {
    fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Self::rlp_decode(data)
    }
}

impl Decodable2718 for ArbitrumInternalTx {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> Eip2718Result<Self> {
        if ty != ArbTxType::ArbitrumInternal as u8 {
            return Err(Eip2718Error::UnexpectedType(ty));
        }
        let tx = Self::rlp_decode(buf)?;
        Ok(tx)
    }

    fn fallback_decode(buf: &mut &[u8]) -> Eip2718Result<Self> {
        Ok(Self::decode(buf)?)
    }
}

impl Encodable2718 for ArbitrumInternalTx {
    fn encode_2718_len(&self) -> usize {
        self.rlp_encoded_length() + 1
    }

    fn encode_2718(&self, out: &mut dyn BufMut) {
        out.put_u8(self.ty());
        self.rlp_encode(out);
    }
}

impl Transaction for ArbitrumInternalTx {
    fn chain_id(&self) -> Option<ChainId> {
        Some(self.chain_id)
    }

    fn nonce(&self) -> u64 {
        0
    }

    fn gas_limit(&self) -> u64 {
        0
    }

    fn gas_price(&self) -> Option<u128> {
        None
    }

    fn max_fee_per_gas(&self) -> u128 {
        0
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        None
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        None
    }

    fn priority_fee_or_price(&self) -> u128 {
        0
    }

    fn effective_gas_price(&self, _base_fee: Option<u64>) -> u128 {
        0
    }

    fn is_dynamic_fee(&self) -> bool {
        false
    }

    fn kind(&self) -> TxKind {
        TxKind::Call(Self::ARBOS_ADDRESS)
    }

    fn is_create(&self) -> bool {
        false
    }

    fn value(&self) -> U256 {
        U256::ZERO
    }

    fn input(&self) -> &Bytes {
        &self.data
    }

    fn access_list(&self) -> Option<&AccessList> {
        None
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        None
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        None
    }

    fn effective_tip_per_gas(&self, _base_fee: u64) -> Option<u128> {
        None
    }

    fn to(&self) -> Option<Address> {
        Some(Self::ARBOS_ADDRESS)
    }

    fn function_selector(&self) -> Option<&Selector> {
        self.input()
            .get(..4)
            .and_then(|s| TryFrom::try_from(s).ok())
    }
}

impl Sealable for ArbitrumInternalTx {
    fn hash_slow(&self) -> B256 {
        self.tx_hash()
    }
}
