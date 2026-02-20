use std::env;

use alloy_eips::{BlockId, Typed2718};
use alloy_network_primitives::{BlockResponse, ReceiptResponse, TransactionResponse};
use alloy_primitives::{B256, b256};
use alloy_provider::{Provider, ProviderBuilder};
use arb_alloy::{
    consensus::{ArbReceiptEnvelope, ArbTxEnvelope},
    network::Arbitrum,
    provider::ArbProviderExt,
};
use reqwest::Url;

fn rpc_url_from_env() -> Result<Option<Url>, Box<dyn std::error::Error>> {
    let value = match env::var("ARB_RPC_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return Ok(None),
    };
    Ok(Some(value.parse()?))
}

fn tx_variant_name(tx: &ArbTxEnvelope) -> &'static str {
    match tx {
        ArbTxEnvelope::Legacy(_) => "Legacy",
        ArbTxEnvelope::Eip2930(_) => "Eip2930",
        ArbTxEnvelope::Eip1559(_) => "Eip1559",
        ArbTxEnvelope::Eip7702(_) => "Eip7702",
        ArbTxEnvelope::Deposit(_) => "Deposit",
        ArbTxEnvelope::SubmitRetryable(_) => "SubmitRetryable",
        ArbTxEnvelope::Unsigned(_) => "Unsigned",
        ArbTxEnvelope::Contract(_) => "Contract",
        ArbTxEnvelope::Retry(_) => "Retry",
        ArbTxEnvelope::Internal(_) => "Internal",
    }
}

fn receipt_variant_name<T>(receipt: &ArbReceiptEnvelope<T>) -> &'static str {
    match receipt {
        ArbReceiptEnvelope::Legacy(_) => "Legacy",
        ArbReceiptEnvelope::Eip2930(_) => "Eip2930",
        ArbReceiptEnvelope::Eip1559(_) => "Eip1559",
        ArbReceiptEnvelope::Eip4844(_) => "Eip4844",
        ArbReceiptEnvelope::Eip7702(_) => "Eip7702",
        ArbReceiptEnvelope::Deposit(_) => "Deposit",
        ArbReceiptEnvelope::Unsigned(_) => "Unsigned",
        ArbReceiptEnvelope::Contract(_) => "Contract",
        ArbReceiptEnvelope::Retry(_) => "Retry",
        ArbReceiptEnvelope::SubmitRetryable(_) => "SubmitRetryable",
        ArbReceiptEnvelope::Internal(_) => "Internal",
    }
}

fn matching_tx_and_receipt_envelope_types<T>(
    tx: &ArbTxEnvelope,
    receipt: &ArbReceiptEnvelope<T>,
) -> bool {
    matches!(
        (tx, receipt),
        (ArbTxEnvelope::Legacy(_), ArbReceiptEnvelope::Legacy(_))
            | (ArbTxEnvelope::Eip2930(_), ArbReceiptEnvelope::Eip2930(_))
            | (ArbTxEnvelope::Eip1559(_), ArbReceiptEnvelope::Eip1559(_))
            | (ArbTxEnvelope::Eip7702(_), ArbReceiptEnvelope::Eip7702(_))
            | (ArbTxEnvelope::Deposit(_), ArbReceiptEnvelope::Deposit(_))
            | (
                ArbTxEnvelope::SubmitRetryable(_),
                ArbReceiptEnvelope::SubmitRetryable(_)
            )
            | (ArbTxEnvelope::Unsigned(_), ArbReceiptEnvelope::Unsigned(_))
            | (ArbTxEnvelope::Contract(_), ArbReceiptEnvelope::Contract(_))
            | (ArbTxEnvelope::Retry(_), ArbReceiptEnvelope::Retry(_))
            | (ArbTxEnvelope::Internal(_), ArbReceiptEnvelope::Internal(_))
    )
}

#[derive(Clone, Copy)]
enum KnownKind {
    SubmitRetryable,
    Deposit,
    Internal,
}

impl KnownKind {
    const fn ty(self) -> u8 {
        match self {
            Self::SubmitRetryable => 0x69,
            Self::Deposit => 0x64,
            Self::Internal => 0x6a,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::SubmitRetryable => "SubmitRetryable",
            Self::Deposit => "Deposit",
            Self::Internal => "Internal",
        }
    }

    fn matches_tx(self, tx: &ArbTxEnvelope) -> bool {
        match self {
            Self::SubmitRetryable => matches!(tx, ArbTxEnvelope::SubmitRetryable(_)),
            Self::Deposit => matches!(tx, ArbTxEnvelope::Deposit(_)),
            Self::Internal => matches!(tx, ArbTxEnvelope::Internal(_)),
        }
    }

    fn matches_receipt<T>(self, receipt: &ArbReceiptEnvelope<T>) -> bool {
        match self {
            Self::SubmitRetryable => matches!(receipt, ArbReceiptEnvelope::SubmitRetryable(_)),
            Self::Deposit => matches!(receipt, ArbReceiptEnvelope::Deposit(_)),
            Self::Internal => matches!(receipt, ArbReceiptEnvelope::Internal(_)),
        }
    }
}

async fn assert_known_tx_and_receipt_kind(
    provider: &impl Provider<Arbitrum>,
    tx_hash: B256,
    expected_kind: KnownKind,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = provider
        .get_transaction_by_hash(tx_hash)
        .await?
        .ok_or_else(|| format!("missing transaction for known hash {tx_hash}"))?;
    let tx_envelope = tx.as_ref();
    let tx_ty = tx.ty();

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .ok_or_else(|| format!("missing receipt for known hash {tx_hash}"))?;
    let receipt_envelope = &receipt.inner.inner;
    let receipt_ty = receipt_envelope.ty();

    if tx_ty != expected_kind.ty() {
        return Err(format!(
            "known tx type mismatch for {tx_hash}: expected 0x{:02x} ({}) got 0x{tx_ty:02x} ({})",
            expected_kind.ty(),
            expected_kind.name(),
            tx_variant_name(tx_envelope)
        )
        .into());
    }

    if receipt_ty != expected_kind.ty() {
        return Err(format!(
            "known receipt type mismatch for {tx_hash}: expected 0x{:02x} ({}) got 0x{receipt_ty:02x} ({})",
            expected_kind.ty(),
            expected_kind.name(),
            receipt_variant_name(receipt_envelope)
        )
        .into());
    }

    if !expected_kind.matches_tx(tx_envelope) {
        return Err(format!(
            "known tx variant mismatch for {tx_hash}: expected {} got {}",
            expected_kind.name(),
            tx_variant_name(tx_envelope)
        )
        .into());
    }

    if !expected_kind.matches_receipt(receipt_envelope) {
        return Err(format!(
            "known receipt variant mismatch for {tx_hash}: expected {} got {}",
            expected_kind.name(),
            receipt_variant_name(receipt_envelope)
        )
        .into());
    }

    if !matching_tx_and_receipt_envelope_types(tx_envelope, receipt_envelope) {
        return Err(format!(
            "known tx/receipt parity mismatch for {tx_hash}: tx={} receipt={}",
            tx_variant_name(tx_envelope),
            receipt_variant_name(receipt_envelope)
        )
        .into());
    }

    println!(
        "ok known tx={} type=0x{:02x} variant={}",
        tx_hash,
        expected_kind.ty(),
        expected_kind.name()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(url) = rpc_url_from_env()? else {
        eprintln!("Set ARB_RPC_URL to an Arbitrum JSON-RPC endpoint.");
        eprintln!("Example: ARB_RPC_URL=https://arb1.arbitrum.io/rpc cargo run");
        return Ok(());
    };

    let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(url);

    println!("Connected. Fetching latest L2 block number...");
    let latest = provider.get_block_number().await?;
    println!("Latest block: {latest}");

    match provider.arb_check_publisher_health().await {
        Ok(()) => println!("arb_checkPublisherHealth: healthy"),
        Err(err) => println!("arb_checkPublisherHealth failed: {err}"),
    }

    match provider.arb_maintenance_status().await {
        Ok(status) => println!("arb_maintenanceStatus: is_running={}", status.is_running),
        Err(err) => println!("arb_maintenanceStatus failed: {err}"),
    }

    match provider.arb_get_l1_confirmations(latest).await {
        Ok(confirmations) => println!("arb_getL1Confirmations({latest}): {confirmations}"),
        Err(err) => println!("arb_getL1Confirmations({latest}) failed: {err}"),
    }

    let known_hashes = [
        (
            b256!("ba468eff535d02c61cc4dba52987287e5412c23fea8dd5ea63ba91f3a18b24b4"),
            KnownKind::SubmitRetryable,
        ),
        (
            b256!("982d30564efe4ceec675a09c72637d1f9490558131f31d4c37c9c4c8e08d7724"),
            KnownKind::Deposit,
        ),
        (
            b256!("51f2698bcf39d55c0d2a9c49d9192980c5b2c0cc1a2f935d0a8f79df3885a43b"),
            KnownKind::Internal,
        ),
    ];
    println!(
        "Checking {} known special tx hashes for exact decode parity...",
        known_hashes.len()
    );
    for (tx_hash, kind) in known_hashes {
        assert_known_tx_and_receipt_kind(&provider, tx_hash, kind).await?;
    }

    println!("Fetching latest full block for header + receipt decode checks...");
    let latest_block = provider.get_block(BlockId::latest()).full().await?;
    let Some(block) = latest_block else {
        println!("Latest block response was empty; skipping decode checks.");
        return Ok(());
    };

    let header = block.header();
    let block_hash = header.hash;
    println!(
        "Header: number={} hash={} parent={} timestamp={}",
        header.number, header.hash, header.parent_hash, header.timestamp
    );

    let txs = match block.transactions().as_transactions() {
        Some(txs) if !txs.is_empty() => txs,
        _ => {
            println!("Latest block has no full transactions; skipping receipt checks.");
            return Ok(());
        }
    };

    let sample_size = txs.len().min(5);
    println!("Checking {sample_size} tx/receipt pairs for envelope parity...");
    for tx in txs.iter().take(sample_size) {
        let tx_hash = tx.tx_hash();
        let tx_envelope = tx.as_ref();
        let tx_ty = tx.ty();

        let receipt = provider
            .get_transaction_receipt(tx_hash)
            .await?
            .ok_or_else(|| format!("missing receipt for tx {tx_hash}"))?;
        let receipt_envelope = &receipt.inner.inner;
        let receipt_ty = receipt_envelope.ty();

        if receipt.transaction_hash() != tx_hash {
            return Err(format!(
                "receipt hash mismatch for tx {tx_hash}: got {}",
                receipt.transaction_hash()
            )
            .into());
        }

        if receipt.block_hash() != Some(block_hash) {
            return Err(format!(
                "receipt block hash mismatch for tx {tx_hash}: expected {block_hash}, got {:?}",
                receipt.block_hash()
            )
            .into());
        }

        if tx_ty != receipt_ty {
            return Err(format!(
                "typed envelope mismatch for tx {tx_hash}: tx type=0x{tx_ty:02x}, receipt type=0x{receipt_ty:02x}"
            )
            .into());
        }

        if !matching_tx_and_receipt_envelope_types(tx_envelope, receipt_envelope) {
            return Err(format!(
                "decoded enum variant mismatch for tx {tx_hash}: tx={} receipt={}",
                tx_variant_name(tx_envelope),
                receipt_variant_name(receipt_envelope)
            )
            .into());
        }

        println!(
            "ok tx={} type=0x{tx_ty:02x} variant={}",
            tx_hash,
            tx_variant_name(tx_envelope)
        );
    }
    println!("Receipt decode parity checks passed for {sample_size} transactions.");

    Ok(())
}
