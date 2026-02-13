pub mod receipt;
pub mod request;
pub mod transaction;

pub use receipt::ArbTransactionReceipt;
pub use request::ArbTransactionRequest;
pub use transaction::ArbTransaction;

use alloy_primitives::Bytes;
use serde::{Deserialize, Serialize};

/// Returned by `arb_maintenanceStatus`.
/// Nitro reference: `nitro/execution/interface.go` -> `MaintenanceStatus`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbMaintenanceStatus {
    pub is_running: bool,
}

/// Returned by `arb_getMinRequiredNitroVersion`.
/// Nitro reference: `nitro/arbnode/nitro-version-alerter/server.go` -> `MinRequiredNitroVersionResult`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbMinRequiredNitroVersion {
    pub node_version: String,
    pub node_version_date: String,
    pub upgrade_deadline: String,
}

/// Returned by `arb_getRawBlockMetadata`.
/// Nitro reference: `nitro/execution/gethexec/api.go` -> `NumberAndBlockMetadata`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbRawBlockMetadata {
    #[serde(with = "alloy_serde::quantity")]
    pub block_number: u64,
    pub raw_metadata: Bytes,
}
