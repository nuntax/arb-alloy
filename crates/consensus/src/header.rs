use alloy_consensus::Header;
use alloy_primitives::{B256, Bytes};
use core::fmt;

/// Exact byte length of Arbitrum's header info encoding in `Header.extra_data`.
pub const ARB_HEADER_EXTRA_DATA_LEN: usize = 32 + 8 + 8 + 8;

/// Decoded Arbitrum information embedded in `Header.extra_data`.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ArbHeaderInfo {
    /// Merkle root of the delayed send queue.
    pub send_root: B256,
    /// Number of sends included so far.
    #[serde(with = "alloy_serde::quantity")]
    pub send_count: u64,
    /// L1 block number observed by ArbOS for this L2 block.
    #[serde(with = "alloy_serde::quantity")]
    pub l1_block_number: u64,
    /// ArbOS format version encoded into the header.
    #[serde(with = "alloy_serde::quantity")]
    pub arbos_format_version: u64,
}

/// Error while decoding Arbitrum header info from `Header.extra_data`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArbHeaderDecodeError {
    /// The `extra_data` byte length did not match Arbitrum's expected format.
    InvalidLength {
        /// Number of bytes present in `extra_data`.
        got: usize,
    },
}

impl fmt::Display for ArbHeaderDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { got } => {
                write!(
                    f,
                    "invalid Arbitrum header extraData length: got {got}, expected {ARB_HEADER_EXTRA_DATA_LEN}"
                )
            }
        }
    }
}

impl std::error::Error for ArbHeaderDecodeError {}

impl ArbHeaderInfo {
    /// Returns true when this header info encodes an ArbOS format version.
    pub const fn is_arbitrum(&self) -> bool {
        self.arbos_format_version > 0
    }

    /// Decodes Arbitrum header info from the raw `extra_data` bytes.
    pub fn decode_extra_data(extra_data: &[u8]) -> Result<Self, ArbHeaderDecodeError> {
        if extra_data.len() != ARB_HEADER_EXTRA_DATA_LEN {
            return Err(ArbHeaderDecodeError::InvalidLength {
                got: extra_data.len(),
            });
        }

        let mut send_root = [0u8; 32];
        send_root.copy_from_slice(&extra_data[..32]);

        let mut send_count_bytes = [0u8; 8];
        send_count_bytes.copy_from_slice(&extra_data[32..40]);

        let mut l1_block_number_bytes = [0u8; 8];
        l1_block_number_bytes.copy_from_slice(&extra_data[40..48]);

        let mut arbos_format_version_bytes = [0u8; 8];
        arbos_format_version_bytes.copy_from_slice(&extra_data[48..56]);

        Ok(Self {
            send_root: send_root.into(),
            send_count: u64::from_be_bytes(send_count_bytes),
            l1_block_number: u64::from_be_bytes(l1_block_number_bytes),
            arbos_format_version: u64::from_be_bytes(arbos_format_version_bytes),
        })
    }

    /// Decodes Arbitrum header info from an Ethereum header.
    pub fn decode_header(header: &Header) -> Result<Self, ArbHeaderDecodeError> {
        Self::decode_extra_data(header.extra_data.as_ref())
    }

    /// Encodes this info into Arbitrum header `extra_data` bytes.
    pub fn encode_extra_data(&self) -> Bytes {
        let mut out = [0u8; ARB_HEADER_EXTRA_DATA_LEN];
        out[..32].copy_from_slice(self.send_root.as_slice());
        out[32..40].copy_from_slice(&self.send_count.to_be_bytes());
        out[40..48].copy_from_slice(&self.l1_block_number.to_be_bytes());
        out[48..56].copy_from_slice(&self.arbos_format_version.to_be_bytes());
        Bytes::copy_from_slice(&out)
    }

    /// Returns the L1 block number for a parent header, matching Nitro behavior.
    ///
    /// If `extra_data` does not decode or has `arbos_format_version == 0`,
    /// this falls back to the header's L2 block number.
    pub fn parent_l1_block_number(header: &Header) -> u64 {
        match Self::decode_header(header) {
            Ok(info) if info.is_arbitrum() => info.l1_block_number,
            _ => header.number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_info_roundtrip() {
        let info = ArbHeaderInfo {
            send_root: B256::from([0x11; 32]),
            send_count: 42,
            l1_block_number: 99_001,
            arbos_format_version: 32,
        };

        let bytes = info.encode_extra_data();
        let decoded = ArbHeaderInfo::decode_extra_data(bytes.as_ref()).unwrap();

        assert_eq!(decoded, info);
    }

    #[test]
    fn decode_rejects_invalid_length() {
        let err = ArbHeaderInfo::decode_extra_data(&[0u8; 55]).unwrap_err();
        assert_eq!(err, ArbHeaderDecodeError::InvalidLength { got: 55 });
    }

    #[test]
    fn parent_l1_block_number_fallback_for_legacy_headers() {
        let header = Header { number: 1234, extra_data: Bytes::new(), ..Default::default() };
        assert_eq!(ArbHeaderInfo::parent_l1_block_number(&header), 1234);
    }

    #[test]
    fn parent_l1_block_number_uses_decoded_info() {
        let info = ArbHeaderInfo {
            send_root: B256::from([0xAA; 32]),
            send_count: 1,
            l1_block_number: 8_888_888,
            arbos_format_version: 50,
        };

        let header = Header { number: 7777, extra_data: info.encode_extra_data(), ..Default::default() };

        assert_eq!(ArbHeaderInfo::parent_l1_block_number(&header), 8_888_888);
    }
}
