use std::str::FromStr;

use alloy_core::hex::FromHex;
use alloy_primitives::{Address, FixedBytes, U256};
use serde::*;
use serde_json::Value;

/// Root JSON object for a sequencer feed payload batch.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    /// Feed schema version.
    pub version: u8,
    /// Optional list of broadcast feed messages.
    pub messages: Option<Vec<BroadcastFeedMessage>>,
}

/// Single sequencer feed message entry with ordering metadata.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastFeedMessage {
    /// Monotonic sequence number assigned by the feed server.
    pub sequence_number: u64,
    /// Message body and message-level metadata.
    #[serde(rename = "message")]
    pub message_with_meta_data: MessageWithMetadata,
}

/// Message payload with delayed inbox progress metadata.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageWithMetadata {
    /// L1 incoming message payload.
    #[serde(rename = "message")]
    pub l1_incoming_message: L1IncomingMessage,
    /// Number of delayed messages consumed before this message.
    pub delayed_messages_read: u64,
}

/// Batch data tokenization stats used by newer batch posting reports.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchDataStats {
    /// Total byte length of batch data.
    #[serde(rename = "Length")]
    pub length: u64,
    /// Number of non-zero bytes in batch data.
    #[serde(rename = "NonZeros")]
    pub non_zeros: u64,
}

/// L1 inbox message plus optional batch accounting metadata.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessage {
    /// L1 message header.
    pub header: Header,
    /// Hex-encoded L2 payload bytes.
    #[serde(rename = "l2Msg")]
    pub l2msg: String,
    /// Legacy gas accounting for batch posting report messages.
    #[serde(rename = "batchGasCost")]
    pub legacy_batch_gas_cost: Option<u64>,
    /// Newer tokenized batch data stats.
    #[serde(rename = "batchDataTokens")]
    pub batch_data_stats: Option<BatchDataStats>,
}

/// Raw L1 inbox message header as provided by feed JSON.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// L1 message kind code.
    pub kind: u8,
    /// Hex-encoded sender address.
    pub sender: String,
    /// L1 block number for this message.
    pub block_number: u64,
    /// L1 timestamp for this message.
    pub timestamp: u64,
    /// Request identifier (shape varies by message type).
    pub request_id: Value,
    /// L1 base fee (shape varies by message type).
    pub base_fee_l1: Value,
}

/// Normalized header data used by sequencer decoders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1Header {
    /// L1 message kind code.
    pub kind: u8,
    /// L1 block number for this message.
    pub block_number: u64,
    /// L1 timestamp for this message.
    pub timestamp: u64,
    /// Parsed request identifier when present.
    pub request_id: Option<FixedBytes<32>>,
    /// Parsed L1 base fee when present.
    pub base_fee_l1: Option<U256>,
    /// Parsed poster/sender address.
    pub poster: Address,
    /// Delayed message cursor for this message.
    pub delayed_messages_read: u64,
}
impl L1Header {
    /// Converts the raw JSON header plus delayed count into a normalized header.
    pub fn from_header(header: &Header, delayed_messages_read: u64) -> Result<Self, String> {
        let poster = Address::from_str(&header.sender)
            .map_err(|e| format!("failed to parse poster address: {}", e))?;
        let request_id_str = header.request_id.as_str();
        let request_id = match request_id_str {
            Some(s) => {
                let bytes = <[u8; 32]>::from_hex(s.trim_start_matches("0x"))
                    .map_err(|e| format!("failed to parse request_id hex string '{}': {}", s, e))?;
                Some(FixedBytes::from(bytes))
            }
            None => None,
        };
        let base_fee_l1_u64 = header.base_fee_l1.as_u64();
        let base_fee_l1 = base_fee_l1_u64.map(|x| U256::from(x));
        Ok(L1Header {
            kind: header.kind,
            block_number: header.block_number,
            timestamp: header.timestamp,
            request_id,
            base_fee_l1,
            poster,
            delayed_messages_read,
        })
    }
}

/// Arbitrum L1 message type discriminator used by feed payloads.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Contains an L2 message envelope.
    L2Message = 3,
    /// End-of-block marker.
    EndOfBlock = 6,
    /// L2 transaction funded by L1 value.
    L2FundedByL1 = 7,
    /// Rollup event message.
    RollupEvent = 8,
    /// Submit retryable ticket message.
    SubmitRetryable = 9,
    /// Batch used for gas estimation.
    BatchForGasEstimation = 10,
    /// ArbOS initialize message.
    Initialize = 11,
    /// ETH deposit message.
    EthDeposit = 12,
    /// Batch posting report message.
    BatchPostingReport = 13,
    /// Unknown or unsupported message type.
    Invalid = 0xFF,
}
impl MessageType {
    /// Converts a raw numeric kind into a typed enum variant.
    pub fn from_u8(value: u8) -> Self {
        match value {
            3 => MessageType::L2Message,
            6 => MessageType::EndOfBlock,
            7 => MessageType::L2FundedByL1,
            8 => MessageType::RollupEvent,
            9 => MessageType::SubmitRetryable,
            10 => MessageType::BatchForGasEstimation,
            11 => MessageType::Initialize,
            12 => MessageType::EthDeposit,
            13 => MessageType::BatchPostingReport,
            _ => MessageType::Invalid,
        }
    }
    #[allow(dead_code)]
    /// Returns the canonical numeric discriminator for this message type.
    pub fn to_u8(&self) -> u8 {
        match self {
            MessageType::L2Message => 3,
            MessageType::EndOfBlock => 6,
            MessageType::L2FundedByL1 => 7,
            MessageType::RollupEvent => 8,
            MessageType::SubmitRetryable => 9,
            MessageType::BatchForGasEstimation => 10,
            MessageType::Initialize => 11,
            MessageType::EthDeposit => 12,
            MessageType::BatchPostingReport => 13,
            MessageType::Invalid => 0xFF,
        }
    }
}
