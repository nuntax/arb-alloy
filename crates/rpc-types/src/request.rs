use serde::{Deserialize, Serialize};

use arb_alloy_consensus::ArbTxEnvelope;

/// Arbitrum transaction request wrapper around Ethereum request fields.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbTransactionRequest {
    /// Underlying Ethereum transaction request payload.
    #[serde(flatten)]
    pub inner: alloy_rpc_types_eth::TransactionRequest,
}

impl From<alloy_rpc_types_eth::TransactionRequest> for ArbTransactionRequest {
    fn from(inner: alloy_rpc_types_eth::TransactionRequest) -> Self {
        Self { inner }
    }
}

impl From<ArbTxEnvelope> for ArbTransactionRequest {
    fn from(tx: ArbTxEnvelope) -> Self {
        Self {
            inner: alloy_rpc_types_eth::TransactionRequest::from_transaction(tx),
        }
    }
}

impl From<arb_alloy_consensus::ArbTypedTransaction> for ArbTransactionRequest {
    fn from(tx: arb_alloy_consensus::ArbTypedTransaction) -> Self {
        let inner = match tx {
            arb_alloy_consensus::ArbTypedTransaction::Legacy(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Eip2930(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Eip1559(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Eip7702(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Deposit(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::SubmitRetryable(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Unsigned(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Contract(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Retry(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
            arb_alloy_consensus::ArbTypedTransaction::Internal(tx) => {
                alloy_rpc_types_eth::TransactionRequest::from_transaction(tx)
            }
        };

        Self { inner }
    }
}
