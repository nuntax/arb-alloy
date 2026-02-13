use std::sync::OnceLock;

use alloy_consensus::{Transaction, Typed2718};
use alloy_eips::{
    Decodable2718, Encodable2718,
    eip2718::{Eip2718Error, Eip2718Result},
    eip2930::AccessList,
    eip7702::SignedAuthorization,
};
use alloy_primitives::{Address, B256, Bytes, ChainId, Sealable, TxHash, TxKind, U256, keccak256};
use alloy_rlp::{Decodable, Encodable, Header};
use bytes::BufMut;
use serde::{Deserialize, Serialize};

use crate::transactions::ArbTxType;

#[derive(PartialEq, Debug, Clone, Eq, Serialize, Deserialize)]
pub struct TxRetry {
    pub chain_id: U256,
    pub nonce: u64,
    pub from: Address,
    pub gas_fee_cap: U256,
    pub gas_limit: u64,
    pub to: TxKind,
    pub value: U256,
    pub input: Bytes,
    pub ticket_id: B256,
    pub refund_to: Address,
    pub max_refund: U256,
    pub submission_fee_refund: U256,
    #[serde(skip)]
    pub hash: OnceLock<TxHash>,
}

impl TxRetry {
    pub fn from(&self) -> Address {
        self.from
    }

    pub fn tx_hash(&self) -> TxHash {
        *self.hash.get_or_init(|| {
            let buffer = &mut Vec::with_capacity(self.rlp_encoded_fields_length() + 1);
            self.encode_2718(buffer);
            keccak256(buffer)
        })
    }

    pub fn rlp_encode_fields(&self, out: &mut dyn BufMut) {
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.from.encode(out);
        self.gas_fee_cap.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.input.encode(out);
        self.ticket_id.encode(out);
        self.refund_to.encode(out);
        self.max_refund.encode(out);
        self.submission_fee_refund.encode(out);
    }

    pub fn rlp_encoded_fields_length(&self) -> usize {
        self.chain_id.length()
            + self.nonce.length()
            + self.from.length()
            + self.gas_fee_cap.length()
            + self.gas_limit.length()
            + self.to.length()
            + self.value.length()
            + self.input.length()
            + self.ticket_id.length()
            + self.refund_to.length()
            + self.max_refund.length()
            + self.submission_fee_refund.length()
    }

    pub fn rlp_header(&self) -> Header {
        Header { list: true, payload_length: self.rlp_encoded_fields_length() }
    }

    pub fn rlp_encode(&self, out: &mut dyn BufMut) {
        self.rlp_header().encode(out);
        self.rlp_encode_fields(out);
    }

    fn rlp_encoded_length(&self) -> usize {
        self.rlp_header().length_with_payload()
    }

    pub fn rlp_decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let header = Header::decode(buf)?;
        if !header.list {
            return Err(alloy_rlp::Error::Custom("Expected list header"));
        }
        let chain_id: U256 = Decodable::decode(buf)?;
        let nonce: u64 = Decodable::decode(buf)?;
        let from: Address = Decodable::decode(buf)?;
        let gas_fee_cap: U256 = Decodable::decode(buf)?;
        let gas_limit: u64 = Decodable::decode(buf)?;
        let to: TxKind = Decodable::decode(buf)?;
        let value: U256 = Decodable::decode(buf)?;
        let input: Bytes = Decodable::decode(buf)?;
        let ticket_id: B256 = Decodable::decode(buf)?;
        let refund_to: Address = Decodable::decode(buf)?;
        let max_refund: U256 = Decodable::decode(buf)?;
        let submission_fee_refund: U256 = Decodable::decode(buf)?;
        Ok(Self {
            chain_id,
            nonce,
            from,
            gas_fee_cap,
            gas_limit,
            to,
            value,
            input,
            ticket_id,
            refund_to,
            max_refund,
            submission_fee_refund,
            hash: OnceLock::new(),
        })
    }
}

impl Typed2718 for TxRetry {
    fn ty(&self) -> u8 {
        ArbTxType::Retry as u8
    }
}

impl Decodable for TxRetry {
    fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Self::rlp_decode(data)
    }
}

impl Decodable2718 for TxRetry {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> Eip2718Result<Self> {
        if ty != ArbTxType::Retry as u8 {
            return Err(Eip2718Error::UnexpectedType(ty));
        }
        Ok(Self::rlp_decode(buf)?)
    }

    fn fallback_decode(buf: &mut &[u8]) -> Eip2718Result<Self> {
        Ok(Self::decode(buf)?)
    }
}

impl Encodable2718 for TxRetry {
    fn encode_2718_len(&self) -> usize {
        self.rlp_encoded_length() + 1
    }

    fn encode_2718(&self, out: &mut dyn BufMut) {
        out.put_u8(self.ty());
        self.rlp_encode(out);
    }
}

impl Transaction for TxRetry {
    fn chain_id(&self) -> Option<ChainId> {
        Some(self.chain_id.to())
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    fn gas_price(&self) -> Option<u128> {
        Some(self.gas_fee_cap.to())
    }

    fn max_fee_per_gas(&self) -> u128 {
        self.gas_fee_cap.to()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        None
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        None
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.gas_fee_cap.to()
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        base_fee.map(|v| v as u128).unwrap_or_else(|| self.gas_fee_cap.to())
    }

    fn is_dynamic_fee(&self) -> bool {
        false
    }

    fn kind(&self) -> TxKind {
        self.to
    }

    fn is_create(&self) -> bool {
        self.to.is_create()
    }

    fn value(&self) -> U256 {
        self.value
    }

    fn input(&self) -> &Bytes {
        &self.input
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
}

impl Sealable for TxRetry {
    fn hash_slow(&self) -> B256 {
        self.tx_hash()
    }
}
