use alloc::{string::String, vec::Vec};
use alloy_primitives::{B256, U256};
use serde::{Deserialize, Serialize};

/// Returned by `arbdebug_pricingModel`.
///
/// Nitro reference: `nitro/execution/gethexec/api.go` -> `PricingModelHistory`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingModelHistory {
    /// First block in the queried range.
    pub start: u64,
    /// Last block in the queried range.
    pub end: u64,
    /// Block step between samples.
    pub step: u64,

    // -- L2 pricing arrays (one entry per sampled block) --
    /// Block timestamps.
    pub timestamp: Vec<u64>,
    /// L2 base-fee at each sample.
    pub base_fee: Vec<U256>,
    /// Backlogged gas at each sample.
    pub gas_backlog: Vec<u64>,
    /// Gas used at each sample.
    pub gas_used: Vec<u64>,

    // -- L2 pricing scalars (constant for the range) --
    /// Minimum L2 base-fee.
    pub min_base_fee: U256,
    /// L2 speed limit (gas/sec).
    pub speed_limit: u64,
    /// Per-block gas limit.
    pub per_block_gas_limit: u64,
    /// Per-transaction gas limit.
    pub per_tx_gas_limit: u64,
    /// L2 pricing inertia parameter.
    pub pricing_inertia: u64,
    /// L2 backlog tolerance parameter.
    pub backlog_tolerance: u64,

    // -- L1 pricing arrays --
    /// Estimated L1 base-fee at each sample.
    pub l1_base_fee_estimate: Vec<U256>,
    /// L1 surplus/deficit at each sample.
    pub l1_last_surplus: Vec<U256>,
    /// L1 funds due at each sample.
    pub l1_funds_due: Vec<U256>,
    /// L1 funds due for rewards at each sample.
    pub l1_funds_due_for_rewards: Vec<U256>,
    /// Calldata units since last L1 price update.
    pub l1_units_since_update: Vec<u64>,
    /// Timestamp of last L1 price update.
    pub l1_last_update_time: Vec<u64>,

    // -- L1 pricing scalars --
    /// L1 price equilibration units.
    pub l1_equilibration_units: U256,
    /// Fixed per-batch cost (can be negative, signed).
    pub l1_per_batch_cost: i64,
    /// Amortized cost cap in basis points.
    pub l1_amortized_cost_cap_bips: u64,
    /// L1 pricing inertia.
    pub l1_pricing_inertia: u64,
    /// L1 per-unit reward.
    pub l1_per_unit_reward: u64,
    /// Address receiving L1 pricer rewards.
    pub l1_pay_reward_to: String,
}

/// Returned by `arbdebug_timeoutQueueHistory`.
///
/// Nitro reference: `nitro/execution/gethexec/api.go` -> `TimeoutQueueHistory`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeoutQueueHistory {
    /// First block in the range.
    pub start: u64,
    /// Last block in the range.
    pub end: u64,
    /// Block step between samples.
    pub step: u64,
    /// Number of pending retryable tickets at each sample.
    pub count: Vec<u64>,
}

/// Returned by `arbdebug_timeoutQueue`.
///
/// Nitro reference: `nitro/execution/gethexec/api.go` -> `TimeoutQueue`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeoutQueue {
    /// Block number at which the queue was sampled.
    pub block_number: u64,
    /// Retryable ticket IDs in the queue.
    pub tickets: Vec<B256>,
    /// Timeout timestamps for each ticket.
    pub timeouts: Vec<u64>,
}
