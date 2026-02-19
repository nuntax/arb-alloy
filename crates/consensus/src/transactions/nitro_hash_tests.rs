use alloy_primitives::hex::FromHex;
use alloy_primitives::{B256, Bytes, FixedBytes, TxHash, U256, address};

use super::{ArbInternalTx, SubmitRetryableTx, TxContract, TxDeposit, TxRetry, TxUnsigned};
use alloy_primitives::TxKind;

#[test]
fn nitro_hash_vectors_arbitrum_types() {
    let chain_id = U256::from(42161u64);

    let addr1 = address!("0x1111111111111111111111111111111111111111");
    let addr2 = address!("0x2222222222222222222222222222222222222222");
    let addr3 = address!("0x3333333333333333333333333333333333333333");
    let addr4 = address!("0x4444444444444444444444444444444444444444");
    let addr5 = address!("0x5555555555555555555555555555555555555555");

    let req1 = B256::from([0xaa; 32]);
    let req2 = B256::from([0xbb; 32]);
    let req3 = FixedBytes::from([0xcc; 32]);

    let unsigned = TxUnsigned {
        chain_id,
        from: addr1,
        nonce: 7,
        gas_fee_cap: U256::from(12345u64),
        gas_limit: 21000,
        to: TxKind::Call(addr2),
        value: U256::from(100u64),
        input: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
    };
    assert_eq!(
        unsigned.tx_hash(),
        TxHash::from_hex("0x815fe243a019e4903168fa746b13bc64eeb677d3f9f87ce40ff8ce95fe25eae4",)
            .unwrap()
    );

    let contract = TxContract {
        chain_id,
        request_id: req1,
        from: addr1,
        gas_fee_cap: U256::from(12345u64),
        gas_limit: 50000,
        to: TxKind::Call(addr2),
        value: U256::from(1u64),
        input: Bytes::from(vec![0xab, 0xcd, 0xef]),
    };
    assert_eq!(
        contract.tx_hash(),
        TxHash::from_hex("0xb05db1de136f0945ec6ba3bdee8d432a02ba7a2b760501fbcfb2f32cef2e7825",)
            .unwrap()
    );

    let retry = TxRetry {
        chain_id,
        nonce: 3,
        from: addr1,
        gas_fee_cap: U256::from(555u64),
        gas_limit: 80000,
        to: TxKind::Call(addr2),
        value: U256::from(5u64),
        input: Bytes::from(vec![0x01, 0x02]),
        ticket_id: req2,
        refund_to: addr3,
        max_refund: U256::from(1000u64),
        submission_fee_refund: U256::from(2000u64),
    };
    assert_eq!(
        retry.tx_hash(),
        TxHash::from_hex("0x365cb4747eec1a5703fde8496018afffc055ba835361159935bdefd051510bd5",)
            .unwrap()
    );

    let submit = SubmitRetryableTx::new(
        chain_id,
        req1,
        addr1,
        U256::from(100u64),
        U256::from(200u64),
        U256::from(300u64),
        U256::from(400000u64),
        TxKind::Call(addr2),
        U256::from(50u64),
        addr4,
        U256::from(600u64),
        addr5,
        Bytes::from(vec![0x01, 0x02, 0x03, 0x04]),
    );
    assert_eq!(
        submit.tx_hash(),
        TxHash::from_hex("0x7c21633a95b29d2af5bf71d2efe4f36352e5aa8cf1980c968245d4b1069ad95e",)
            .unwrap()
    );

    let deposit = TxDeposit {
        chain_id,
        request_id: req3,
        from: addr1,
        to: addr2,
        value: U256::from(123u64),
    };
    assert_eq!(
        deposit.tx_hash(),
        TxHash::from_hex("0xd53dd73b1b2534d4023508fa3c26f4c7ea74b923c0f190a8874dd689a2c1cd21",)
            .unwrap()
    );

    let internal = ArbInternalTx::new(42161u64, Bytes::from(vec![0x11, 0x22, 0x33, 0x44, 0x55]));
    assert_eq!(
        internal.tx_hash(),
        TxHash::from_hex("0x5546c6af5de10e9bcaf1943ac8feddeb2ba2bb1f4252fddd69b16763fb454ab4",)
            .unwrap()
    );
}
