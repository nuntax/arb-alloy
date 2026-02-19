use alloc::vec::Vec;
use alloy_primitives::Address;
use alloy_rpc_types_eth::BlockNumberOrTag;
use serde::{Deserialize, Serialize};

/// Filter parameters for `arbtrace_filter`.
///
/// Nitro's `arbtrace` namespace forwards all calls as raw JSON to a fallback
/// trace client (classic Arbitrum node or Erigon). The filter schema follows
/// the OpenEthereum `trace_filter` convention.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceFilter {
    /// Start block (inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockNumberOrTag>,
    /// End block (inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockNumberOrTag>,
    /// Restrict to transactions from these senders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_address: Option<Vec<Address>>,
    /// Restrict to transactions targeting these addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_address: Option<Vec<Address>>,
    /// Offset for pagination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<u64>,
    /// Maximum number of results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}
