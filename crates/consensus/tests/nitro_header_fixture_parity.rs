#![allow(missing_docs)]

use alloy_primitives::{B256, hex};
use arb_alloy_consensus::{ArbHeaderInfo, header::ARB_HEADER_EXTRA_DATA_LEN};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FixtureFile {
    vectors: Vec<HeaderFixture>,
}

#[derive(Debug, Deserialize)]
struct HeaderFixture {
    name: String,
    extra_data: String,
    expect: HeaderExpect,
}

#[derive(Debug, Deserialize)]
struct HeaderExpect {
    send_root: String,
    send_count: u64,
    l1_block_number: u64,
    arbos_format_version: u64,
}

#[test]
fn nitro_header_vectors_roundtrip_and_accessors() {
    let file: FixtureFile = serde_json::from_str(include_str!("../testdata/nitro_header_fixtures.json"))
        .expect("fixture JSON should deserialize");

    for fixture in file.vectors {
        let extra_data = parse_hex_bytes(&fixture.extra_data);
        assert_eq!(
            extra_data.len(),
            ARB_HEADER_EXTRA_DATA_LEN,
            "{}: unexpected extra_data length",
            fixture.name
        );

        let decoded = ArbHeaderInfo::decode_extra_data(extra_data.as_ref())
            .unwrap_or_else(|e| panic!("{}: decode_extra_data failed: {e}", fixture.name));

        assert_eq!(
            decoded.send_root,
            parse_b256(&fixture.expect.send_root),
            "{}: send_root mismatch",
            fixture.name
        );
        assert_eq!(
            decoded.send_count,
            fixture.expect.send_count,
            "{}: send_count mismatch",
            fixture.name
        );
        assert_eq!(
            decoded.l1_block_number,
            fixture.expect.l1_block_number,
            "{}: l1_block_number mismatch",
            fixture.name
        );
        assert_eq!(
            decoded.arbos_format_version,
            fixture.expect.arbos_format_version,
            "{}: arbos_format_version mismatch",
            fixture.name
        );

        let reencoded = decoded.encode_extra_data();
        assert_eq!(
            reencoded.as_ref(),
            extra_data.as_slice(),
            "{}: re-encoded bytes mismatch",
            fixture.name
        );
    }
}

fn parse_b256(s: &str) -> B256 {
    s.parse::<B256>().expect("valid hex B256")
}

fn parse_hex_bytes(s: &str) -> Vec<u8> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(stripped).expect("valid hex bytes")
}
