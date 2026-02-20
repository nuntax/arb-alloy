# arb-alloy-rpc-smoke

Small example project that uses `arb-alloy` against a live Arbitrum RPC.

## Run

```bash
cd /home/nun/arbitrum-alloy-workspace/arb-alloy/examples/rpc-smoke
ARB_RPC_URL=https://arb1.arbitrum.io/rpc cargo run
```

The example calls:

- `eth_blockNumber`
- `arb_checkPublisherHealth`
- `arb_maintenanceStatus`
- `arb_getL1Confirmations`

It also:

- checks these known Arbitrum tx hashes explicitly:
  - `ba468eff535d02c61cc4dba52987287e5412c23fea8dd5ea63ba91f3a18b24b4` (`SubmitRetryable`, `0x69`)
  - `982d30564efe4ceec675a09c72637d1f9490558131f31d4c37c9c4c8e08d7724` (`Deposit`, `0x64`)
  - `51f2698bcf39d55c0d2a9c49d9192980c5b2c0cc1a2f935d0a8f79df3885a43b` (`Internal`, `0x6a`)
- fetches the latest block with full transactions
- prints key header fields (`number`, `hash`, `parent_hash`, `timestamp`)
- fetches receipts for up to 5 transactions in the block
- checks tx/receipt parity by typed envelope id and enum variant
