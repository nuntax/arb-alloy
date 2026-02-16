use alloy_primitives::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

/// Submission payload for `timeboost_sendExpressLaneTransaction`.
///
/// Nitro reference: `nitro/timeboost/types.go` -> `JsonExpressLaneSubmission`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonExpressLaneSubmission {
    /// Chain ID of the target chain.
    pub chain_id: U256,
    /// Auction round number.
    #[serde(with = "alloy_serde::quantity")]
    pub round: u64,
    /// Address of the auction contract.
    pub auction_contract_address: Address,
    /// RLP-encoded transaction bytes.
    pub transaction: Bytes,
    /// Optional conditional-inclusion options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    /// Express-lane sequence number.
    #[serde(with = "alloy_serde::quantity")]
    pub sequence_number: u64,
    /// Signature over the submission.
    pub signature: Bytes,
}
