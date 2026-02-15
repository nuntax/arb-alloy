#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use alloy_consensus::{Header as EthHeader, TxType, TypedTransaction};
use alloy_network::{
    BuildResult, Network, NetworkWallet, TransactionBuilder, TransactionBuilderError,
};
use alloy_primitives::{Address, Bytes, ChainId, TxKind, U256};
use alloy_rpc_types_eth::Block;

use arb_alloy_consensus::{ArbReceiptEnvelope, ArbTxEnvelope, ArbTxType, ArbTypedTransaction};
use arb_alloy_rpc_types::{ArbTransaction, ArbTransactionReceipt, ArbTransactionRequest};

/// Alloy `Network` implementation for Arbitrum.
#[derive(Clone, Copy, Debug)]
pub struct Arbitrum {
    _private: (),
}

impl Arbitrum {
    /// Creates a new Arbitrum network marker type.
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for Arbitrum {
    fn default() -> Self {
        Self::new()
    }
}

impl Network for Arbitrum {
    type TxType = ArbTxType;
    type TxEnvelope = ArbTxEnvelope;
    type UnsignedTx = ArbTypedTransaction;
    type ReceiptEnvelope = ArbReceiptEnvelope;
    type Header = EthHeader;

    type TransactionRequest = ArbTransactionRequest;
    type TransactionResponse = ArbTransaction;
    type ReceiptResponse = ArbTransactionReceipt;
    type HeaderResponse = alloy_rpc_types_eth::Header;
    type BlockResponse = Block<Self::TransactionResponse, Self::HeaderResponse>;
}

fn arb_tx_type_from_eth(ty: TxType) -> Option<ArbTxType> {
    match ty {
        TxType::Legacy => Some(ArbTxType::Legacy),
        TxType::Eip2930 => Some(ArbTxType::Eip2930),
        TxType::Eip1559 => Some(ArbTxType::Eip1559),
        TxType::Eip7702 => Some(ArbTxType::Eip7702),
        TxType::Eip4844 => None,
    }
}

impl TransactionBuilder<Arbitrum> for ArbTransactionRequest {
    fn chain_id(&self) -> Option<ChainId> {
        self.inner.chain_id
    }

    fn set_chain_id(&mut self, chain_id: ChainId) {
        self.inner.chain_id = Some(chain_id);
    }

    fn nonce(&self) -> Option<u64> {
        self.inner.nonce
    }

    fn set_nonce(&mut self, nonce: u64) {
        self.inner.nonce = Some(nonce);
    }

    fn take_nonce(&mut self) -> Option<u64> {
        self.inner.nonce.take()
    }

    fn input(&self) -> Option<&Bytes> {
        self.inner.input.input()
    }

    fn set_input<T: Into<Bytes>>(&mut self, input: T) {
        self.inner.input.input = Some(input.into());
    }

    fn set_input_kind<T: Into<Bytes>>(
        &mut self,
        input: T,
        kind: alloy_rpc_types_eth::TransactionInputKind,
    ) {
        match kind {
            alloy_rpc_types_eth::TransactionInputKind::Input => {
                self.inner.input.input = Some(input.into())
            }
            alloy_rpc_types_eth::TransactionInputKind::Data => {
                self.inner.input.data = Some(input.into())
            }
            alloy_rpc_types_eth::TransactionInputKind::Both => {
                let bytes = input.into();
                self.inner.input.input = Some(bytes.clone());
                self.inner.input.data = Some(bytes);
            }
        }
    }

    fn from(&self) -> Option<Address> {
        self.inner.from
    }

    fn set_from(&mut self, from: Address) {
        self.inner.from = Some(from);
    }

    fn kind(&self) -> Option<TxKind> {
        self.inner.to
    }

    fn clear_kind(&mut self) {
        self.inner.to = None;
    }

    fn set_kind(&mut self, kind: TxKind) {
        self.inner.to = Some(kind);
    }

    fn value(&self) -> Option<U256> {
        self.inner.value
    }

    fn set_value(&mut self, value: U256) {
        self.inner.value = Some(value)
    }

    fn gas_price(&self) -> Option<u128> {
        self.inner.gas_price
    }

    fn set_gas_price(&mut self, gas_price: u128) {
        self.inner.gas_price = Some(gas_price);
    }

    fn max_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_fee_per_gas
    }

    fn set_max_fee_per_gas(&mut self, max_fee_per_gas: u128) {
        self.inner.max_fee_per_gas = Some(max_fee_per_gas);
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_priority_fee_per_gas
    }

    fn set_max_priority_fee_per_gas(&mut self, max_priority_fee_per_gas: u128) {
        self.inner.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.inner.gas
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.inner.gas = Some(gas_limit);
    }

    fn access_list(&self) -> Option<&alloy_rpc_types_eth::AccessList> {
        self.inner.access_list.as_ref()
    }

    fn set_access_list(&mut self, access_list: alloy_rpc_types_eth::AccessList) {
        self.inner.access_list = Some(access_list);
    }

    fn complete_type(&self, ty: ArbTxType) -> Result<(), Vec<&'static str>> {
        match ty {
            ArbTxType::Legacy => self.inner.complete_legacy(),
            ArbTxType::Eip2930 => self.inner.complete_2930(),
            ArbTxType::Eip1559 => self.inner.complete_1559(),
            ArbTxType::Eip7702 => self.inner.complete_7702(),
            _ => Err(vec!["unsupported_tx_type"]),
        }
    }

    fn can_submit(&self) -> bool {
        self.inner.from.is_some()
    }

    fn can_build(&self) -> bool {
        let common = self.inner.gas.is_some() && self.inner.nonce.is_some();

        let legacy = self.inner.gas_price.is_some();
        let eip2930 = legacy && self.access_list().is_some();

        let eip1559 =
            self.inner.max_fee_per_gas.is_some() && self.inner.max_priority_fee_per_gas.is_some();

        let eip7702 = eip1559 && self.inner.authorization_list.is_some();

        common && (legacy || eip2930 || eip1559 || eip7702)
    }

    fn output_tx_type(&self) -> ArbTxType {
        match self.inner.preferred_type() {
            TxType::Eip4844 => ArbTxType::Eip1559,
            other => arb_tx_type_from_eth(other).unwrap_or(ArbTxType::Eip1559),
        }
    }

    fn output_tx_type_checked(&self) -> Option<ArbTxType> {
        match self.inner.buildable_type() {
            Some(TxType::Eip4844) => None,
            Some(other) => arb_tx_type_from_eth(other),
            None => None,
        }
    }

    fn prep_for_submission(&mut self) {
        self.inner.transaction_type = Some(self.output_tx_type() as u8);
        self.inner.trim_conflicting_keys();
        self.inner.populate_blob_hashes();
    }

    fn build_unsigned(self) -> BuildResult<ArbTypedTransaction, Arbitrum> {
        let pref = self.inner.preferred_type();
        if pref == TxType::Eip4844 {
            return Err(TransactionBuilderError::InvalidTransactionRequest(
                ArbTxType::Eip1559,
                vec!["eip4844_unsupported"],
            )
            .into_unbuilt(self));
        }

        if let Err((tx_type, missing)) = self.inner.missing_keys() {
            let arb_ty = arb_tx_type_from_eth(tx_type).unwrap_or(ArbTxType::Eip1559);
            return Err(
                TransactionBuilderError::InvalidTransactionRequest(arb_ty, missing)
                    .into_unbuilt(self),
            );
        }

        let typed = self
            .inner
            .clone()
            .build_typed_tx()
            .expect("checked by missing_keys");
        let mapped = match typed {
            TypedTransaction::Legacy(tx) => ArbTypedTransaction::Legacy(tx),
            TypedTransaction::Eip2930(tx) => ArbTypedTransaction::Eip2930(tx),
            TypedTransaction::Eip1559(tx) => ArbTypedTransaction::Eip1559(tx),
            TypedTransaction::Eip7702(tx) => ArbTypedTransaction::Eip7702(tx),
            TypedTransaction::Eip4844(_) => unreachable!("eip4844 is unsupported on Arbitrum"),
        };

        Ok(mapped)
    }

    async fn build<W: NetworkWallet<Arbitrum>>(
        self,
        wallet: &W,
    ) -> Result<<Arbitrum as Network>::TxEnvelope, TransactionBuilderError<Arbitrum>> {
        Ok(wallet.sign_request(self).await?)
    }
}
