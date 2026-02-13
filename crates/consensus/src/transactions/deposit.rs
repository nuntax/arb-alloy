use std::sync::OnceLock;

use alloy_consensus::Transaction;
use alloy_consensus::Typed2718;
use alloy_eips::Decodable2718;
use alloy_eips::Encodable2718;
use alloy_eips::eip2718::Eip2718Error;
use alloy_eips::eip2718::Eip2718Result;
use alloy_eips::eip2930::AccessList;
use alloy_eips::eip7702::SignedAuthorization;
use alloy_primitives::B256;
use alloy_primitives::Bytes;
use alloy_primitives::ChainId;
use alloy_primitives::FixedBytes;
use alloy_primitives::Selector;
use alloy_primitives::TxHash;
use alloy_primitives::TxKind;
use alloy_primitives::{Address, Sealable, U256, keccak256};
use alloy_rlp::Decodable;
use alloy_rlp::Encodable;
use alloy_rlp::Header;
use bytes::BufMut;
use serde::{Deserialize, Serialize};

use crate::transactions::ArbTxType;

#[derive(PartialEq, Debug, Clone, Eq, Serialize, Deserialize)]
pub struct TxDeposit {
    pub chain_id: U256,
    pub request_id: FixedBytes<32>,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    #[serde(skip)]
    pub hash: OnceLock<TxHash>,
}

impl TxDeposit {
    pub fn from(&self) -> Address {
        self.from
    }

    pub fn tx_hash(&self) -> TxHash {
        *self.hash.get_or_init(|| {
            let buffer = &mut Vec::with_capacity(self.rlp_encoded_fields_length());
            self.encode_2718(buffer);
            keccak256(buffer)
        })
    }
    pub fn decode_fields_sequencer(
        buf: &mut &[u8],
        chain_id: U256,
        request_id: FixedBytes<32>,
        from: Address,
    ) -> Result<Self, alloy_rlp::Error> {
        let value: U256 = Decodable::decode(buf)?;
        let to: Address = Decodable::decode(buf)?;
        Ok(Self {
            chain_id,
            request_id,
            from,
            to,
            value,
            hash: OnceLock::new(),
        })
    }
    pub fn rlp_encode_fields(&self, out: &mut dyn BufMut) {
        self.chain_id.encode(out);
        self.request_id.encode(out);
        self.from.encode(out);
        self.to.encode(out);
        self.value.encode(out);
    }
    pub fn rlp_encoded_fields_length(&self) -> usize {
        self.chain_id.length()
            + self.request_id.length()
            + self.from.length()
            + self.to.length()
            + self.value.length()
    }
    pub fn rlp_header(&self) -> Header {
        Header {
            list: true,
            payload_length: self.rlp_encoded_fields_length(),
        }
    }
    pub fn rlp_encode(&self, out: &mut dyn BufMut) {
        self.rlp_header().encode(out);
        self.rlp_encode_fields(out);
    }
    pub fn rlp_decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let header = Header::decode(buf)?;
        if !header.list {
            return Err(alloy_rlp::Error::Custom("Expected list header"));
        }
        Self::rlp_decode_fields(buf)
    }
    fn rlp_encoded_length(&self) -> usize {
        self.rlp_header().length_with_payload()
    }

    pub fn rlp_decode_fields(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let chain_id: U256 = Decodable::decode(buf)?;
        let request_id: FixedBytes<32> = Decodable::decode(buf)?;
        let from: Address = Decodable::decode(buf)?;
        let to: Address = Decodable::decode(buf)?;
        let value: U256 = Decodable::decode(buf)?;
        Ok(Self {
            chain_id,
            request_id,
            from,
            to,
            value,
            hash: OnceLock::new(),
        })
    }
}
impl Typed2718 for TxDeposit {
    #[doc = " Returns the EIP-2718 type flag."]
    fn ty(&self) -> u8 {
        ArbTxType::DepositTx as u8
    }
}
impl Decodable for TxDeposit {
    fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Self::rlp_decode(data)
    }
}
impl Decodable2718 for TxDeposit {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> Eip2718Result<Self> {
        if ty != ArbTxType::DepositTx as u8 {
            return Err(Eip2718Error::UnexpectedType(ty));
        }
        let tx = Self::rlp_decode(buf)?;
        Ok(tx)
    }

    fn fallback_decode(buf: &mut &[u8]) -> Eip2718Result<Self> {
        Ok(Self::decode(buf)?)
    }
}

impl Encodable2718 for TxDeposit {
    #[doc = " The length of the 2718 encoded envelope. This is the length of the type"]
    #[doc = " flag + the length of the inner encoding."]
    fn encode_2718_len(&self) -> usize {
        self.rlp_encoded_length() + 1
    }

    #[doc = " Encode the transaction according to [EIP-2718] rules. First a 1-byte"]
    #[doc = " type flag in the range 0x0-0x7f, then the body of the transaction."]
    #[doc = ""]
    #[doc = " [EIP-2718] inner encodings are unspecified, and produce an opaque"]
    #[doc = " bytestring."]
    #[doc = ""]
    #[doc = " [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718"]
    fn encode_2718(&self, out: &mut dyn BufMut) {
        out.put_u8(self.ty());
        self.rlp_encode(out);
    }
}

impl Transaction for TxDeposit {
    #[doc = " Get `chain_id`."]
    fn chain_id(&self) -> Option<ChainId> {
        Some(self.chain_id.to())
    }

    #[doc = " Get `nonce`."]
    fn nonce(&self) -> u64 {
        0
    }

    #[doc = " Get `gas_limit`."]
    fn gas_limit(&self) -> u64 {
        0 // Deposits do not have a gas limit in the same way as user transactions
    }

    #[doc = " Get `gas_price`."]
    fn gas_price(&self) -> Option<u128> {
        None // Deposits do not have a gas price in the same way as user transactions
    }

    #[doc = " For dynamic fee transactions returns the maximum fee per gas the caller is willing to pay."]
    #[doc = ""]
    #[doc = " For legacy fee transactions this is `gas_price`."]
    #[doc = ""]
    #[doc = " This is also commonly referred to as the \"Gas Fee Cap\"."]
    fn max_fee_per_gas(&self) -> u128 {
        0 // Deposits do not have a max fee per gas in the same way as user transactions
    }

    #[doc = " For dynamic fee transactions returns the Priority fee the caller is paying to the block"]
    #[doc = " author."]
    #[doc = ""]
    #[doc = " This will return `None` for legacy fee transactions"]
    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        None // Deposits do not have a max priority fee per gas in the same way as user transactions
    }

    #[doc = " Max fee per blob gas for EIP-4844 transaction."]
    #[doc = ""]
    #[doc = " Returns `None` for non-eip4844 transactions."]
    #[doc = ""]
    #[doc = " This is also commonly referred to as the \"Blob Gas Fee Cap\"."]
    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        None // Deposits do not have a max fee per blob gas in the same way as user transactions
    }

    #[doc = " Return the max priority fee per gas if the transaction is a dynamic fee transaction, and"]
    #[doc = " otherwise return the gas price."]
    #[doc = ""]
    #[doc = " # Warning"]
    #[doc = ""]
    #[doc = " This is different than the `max_priority_fee_per_gas` method, which returns `None` for"]
    #[doc = " legacy fee transactions."]
    fn priority_fee_or_price(&self) -> u128 {
        0 // Deposits do not have a priority fee or price in the same way as user transactions
    }

    #[doc = " Returns the effective gas price for the given base fee."]
    #[doc = ""]
    #[doc = " If the transaction is a legacy fee transaction, the gas price is returned."]
    fn effective_gas_price(&self, _base_fee: Option<u64>) -> u128 {
        0
    }

    #[doc = " Returns `true` if the transaction supports dynamic fees."]
    fn is_dynamic_fee(&self) -> bool {
        false
    }

    #[doc = " Returns the transaction kind."]
    fn kind(&self) -> TxKind {
        TxKind::Call(self.to)
    }

    #[doc = " Returns true if the transaction is a contract creation."]
    #[doc = " We don\'t provide a default implementation via `kind` as it copies the 21-byte"]
    #[doc = " [`TxKind`] for this simple check. A proper implementation shouldn\'t allocate."]
    fn is_create(&self) -> bool {
        false // Deposits do not create a contract
    }

    #[doc = " Get `value`."]
    fn value(&self) -> U256 {
        self.value
    }

    #[doc = " Get `data`."]
    fn input(&self) -> &Bytes {
        static EMPTY_BYTES: Bytes = Bytes::new();
        &EMPTY_BYTES
    }

    #[doc = " Returns the EIP-2930 `access_list` for the particular transaction type. Returns `None` for"]
    #[doc = " older transaction types."]
    fn access_list(&self) -> Option<&AccessList> {
        None // Deposits do not have an access list in the same way as user transactions
    }

    #[doc = " Blob versioned hashes for eip4844 transaction. For previous transaction types this is"]
    #[doc = " `None`."]
    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        None // Deposits do not have blob versioned hashes in the same way as user transactions
    }

    #[doc = " Returns the [`SignedAuthorization`] list of the transaction."]
    #[doc = ""]
    #[doc = " Returns `None` if this transaction is not EIP-7702."]
    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        None // Deposits do not have an authorization list in the same way as user transactions
    }

    #[doc = " Returns the effective tip for this transaction."]
    #[doc = ""]
    #[doc = " For dynamic fee transactions: `min(max_fee_per_gas - base_fee, max_priority_fee_per_gas)`."]
    #[doc = " For legacy fee transactions: `gas_price - base_fee`."]
    fn effective_tip_per_gas(&self, _base_fee: u64) -> Option<u128> {
        None // Deposits do not have an effective tip in the same way as user transactions
    }

    #[doc = " Get the transaction\'s address of the contract that will be called, or the address that will"]
    #[doc = " receive the transfer."]
    #[doc = ""]
    #[doc = " Returns `None` if this is a `CREATE` transaction."]
    fn to(&self) -> Option<Address> {
        Some(self.to)
    }

    #[doc = " Returns the first 4bytes of the calldata for a function call."]
    #[doc = ""]
    #[doc = " The selector specifies the function to be called."]
    fn function_selector(&self) -> Option<&Selector> {
        None
    }

    #[doc = " Returns the number of blobs of this transaction."]
    #[doc = ""]
    #[doc = " This is convenience function for `len(blob_versioned_hashes)`."]
    #[doc = ""]
    #[doc = " Returns `None` for non-eip4844 transactions."]
    fn blob_count(&self) -> Option<u64> {
        None
    }

    #[doc = " Returns the total gas for all blobs in this transaction."]
    #[doc = ""]
    #[doc = " Returns `None` for non-eip4844 transactions."]
    #[inline]
    fn blob_gas_used(&self) -> Option<u64> {
        None
    }

    #[doc = " Returns the number of blobs of [`SignedAuthorization`] in this transactions"]
    #[doc = ""]
    #[doc = " This is convenience function for `len(authorization_list)`."]
    #[doc = ""]
    #[doc = " Returns `None` for non-eip7702 transactions."]
    fn authorization_count(&self) -> Option<u64> {
        None
    }
}

impl Sealable for TxDeposit {
    fn hash_slow(&self) -> B256 {
        self.tx_hash()
    }
}
