use std::env;

use alloy_eips::BlockId;
use alloy_eips::Typed2718;
use alloy_network_primitives::{BlockResponse, ReceiptResponse, TransactionResponse};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::BlockNumberOrTag;
use reqwest::Url;

use arb_alloy::{network::Arbitrum, provider::ArbProviderExt};

#[test]
fn deserialize_internal_tx_from_rpc_shape() -> Result<(), Box<dyn std::error::Error>> {
    let raw = r#"{
        "blockHash":"0x17e4bcc759042396d9668b5bdcfc7cb293eadf53686e65753b5774e038ccbdbe",
        "blockNumber":"0x19adf086",
        "from":"0x00000000000000000000000000000000000a4b05",
        "gas":"0x0",
        "gasPrice":"0x0",
        "hash":"0x26ae3f2abe865feb7927210b03f5137af48b5f2ee25e748cb411c0f5f9ebb9de",
        "input":"0x6bf6a42d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000174c8760000000000000000000000000000000000000000000000000000000019adf0860000000000000000000000000000000000000000000000000000000000000000",
        "nonce":"0x0",
        "to":"0x00000000000000000000000000000000000a4b05",
        "transactionIndex":"0x0",
        "value":"0x0",
        "type":"0x6a",
        "chainId":"0xa4b1",
        "v":"0x0",
        "r":"0x0",
        "s":"0x0"
    }"#;

    let tx: alloy_rpc_types_eth::Transaction<arb_alloy::consensus::ArbTxEnvelope> =
        serde_json::from_str(raw)?;
    assert_eq!(tx.as_ref().ty(), 0x6a);
    Ok(())
}

#[test]
fn deserialize_internal_receipt_from_rpc_shape() -> Result<(), Box<dyn std::error::Error>> {
    let raw = r#"{
        "blockHash":"0x5ed6c1968fb0bb3e119774f6c29891a50307306019d3086b5bf6a63162b7cfc0",
        "blockNumber":"0x19adf31d",
        "contractAddress":null,
        "cumulativeGasUsed":"0x0",
        "effectiveGasPrice":"0x13524a0",
        "from":"0x00000000000000000000000000000000000a4b05",
        "gasUsed":"0x0",
        "gasUsedForL1":"0x0",
        "l1BlockNumber":"0x174c883",
        "logs":[],
        "logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "status":"0x1",
        "timeboosted":false,
        "to":"0x00000000000000000000000000000000000a4b05",
        "transactionHash":"0xc234a3c1d6418a2908aac0e1b203a9e36772136ece179c0c9d0fe466f98c76a9",
        "transactionIndex":"0x0",
        "type":"0x6a"
    }"#;

    let receipt: arb_alloy::rpc_types::ArbTransactionReceipt = serde_json::from_str(raw)?;
    assert_eq!(receipt.inner.inner.ty(), 0x6a);
    Ok(())
}

fn rpc_url() -> Result<Option<Url>, Box<dyn std::error::Error>> {
    let url = match env::var("ARB_RPC_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return Ok(None),
    };
    Ok(Some(url.parse()?))
}

fn strict_mode() -> bool {
    env::var("ARB_RPC_STRICT")
        .ok()
        .is_some_and(|v| v.trim() == "1" || v.trim().eq_ignore_ascii_case("true"))
}

fn try_rpc(
    name: &str,
    result: Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = result {
        if strict_mode() {
            return Err(err);
        }
        eprintln!("{} failed (non-fatal in non-strict mode): {}", name, err);
    }
    Ok(())
}

#[tokio::test]
async fn arb_rpc_consensus_methods() -> Result<(), Box<dyn std::error::Error>> {
    let Some(url) = rpc_url()? else {
        eprintln!("ARB_RPC_URL not set; skipping arb-alloy integration test");
        return Ok(());
    };
    let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(url);
    let eth_block = provider.get_block_number().await?;

    try_rpc(
        "arb_getL1Confirmations",
        provider
            .arb_get_l1_confirmations(eth_block)
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    try_rpc(
        "arb_findBatchContainingBlock",
        provider
            .arb_find_batch_containing_block(eth_block)
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    Ok(())
}

#[tokio::test]
async fn arb_rpc_version_methods() -> Result<(), Box<dyn std::error::Error>> {
    let Some(url) = rpc_url()? else {
        eprintln!("ARB_RPC_URL not set; skipping arb-alloy integration test");
        return Ok(());
    };
    let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(url);

    try_rpc(
        "arb_getMinRequiredNitroVersion",
        provider
            .arb_get_min_required_nitro_version()
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    Ok(())
}

#[tokio::test]
async fn arb_rpc_execution_methods() -> Result<(), Box<dyn std::error::Error>> {
    let Some(url) = rpc_url()? else {
        eprintln!("ARB_RPC_URL not set; skipping arb-alloy integration test");
        return Ok(());
    };
    let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(url);

    try_rpc(
        "arb_checkPublisherHealth",
        provider
            .arb_check_publisher_health()
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    try_rpc(
        "arb_maintenanceStatus",
        provider
            .arb_maintenance_status()
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    try_rpc(
        "arb_getRawBlockMetadata",
        provider
            .arb_get_raw_block_metadata(BlockNumberOrTag::Latest, BlockNumberOrTag::Latest)
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
    )?;
    Ok(())
}

#[tokio::test]
async fn arb_block_and_receipts() -> Result<(), Box<dyn std::error::Error>> {
    let Some(url) = rpc_url()? else {
        eprintln!("ARB_RPC_URL not set; skipping arb-alloy integration test");
        return Ok(());
    };
    let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(url);

    let block = provider.get_block(BlockId::latest()).full().await?;
    let Some(block) = block else {
        eprintln!("latest block not found; skipping");
        return Ok(());
    };

    let block_hash = block.header().hash;
    assert!(
        block.transactions().is_full(),
        "expected full transactions in block response"
    );

    let txs = match block.transactions().as_transactions() {
        Some(txs) => txs,
        None => {
            eprintln!("block did not return full transactions; skipping receipt checks");
            return Ok(());
        }
    };
    if txs.is_empty() {
        eprintln!("latest block has no transactions; skipping receipt checks");
        return Ok(());
    }

    for tx in txs {
        let tx_hash = tx.tx_hash();
        let receipt = provider.get_transaction_receipt(tx_hash).await?;
        let Some(receipt) = receipt else {
            panic!("missing receipt for tx {}", tx_hash);
        };

        assert_eq!(receipt.transaction_hash(), tx_hash);
        assert_eq!(receipt.block_hash(), Some(block_hash));
    }

    Ok(())
}
