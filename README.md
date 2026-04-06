# arb-alloy

<img src="./arb-alloy-logo.png" alt="arb-alloy logo" width="160" />

Arbitrum integrations for the Alloy Rust SDK.

## ⚠️ Warning
This project is under active development and not yet stable. API and feature coverage may change at any time. This project is neither affiliated or endorsed by Alloy or OffchainLabs.

## Quick Start

```bash
cargo add arb-alloy
```

```rust
use alloy_provider::{Provider, ProviderBuilder};
use arb_alloy::{network::Arbitrum, provider::ArbProviderExt};

#[tokio::main()]
async fn main() {
    let provider = ProviderBuilder::<_, _, Arbitrum>::default()
        .connect("http://localhost:8547")
        .await.unwrap();

    let latest = provider.get_block_number().await.unwrap();
    println!("latest block: {latest}");

    let _ = provider.arb_maintenance_status().await;
}
```

## Documentation

- [Docs Index](./docs/README.md)
- [Quickstart Guide](./docs/quickstart.md)
- [Connect To A Provider](./docs/guides/connect-provider.md)
- [Provider Extensions](./docs/guides/provider-extensions.md)
- [Use Precompiles](./docs/guides/precompiles.md)
- [Local Dev Chain](./docs/guides/local-dev-chain.md)
- [FAQ](./docs/faq.md)

This workspace contains:
- `arb-alloy-consensus`: Arbitrum consensus transaction and receipt types.
- `arb-alloy-network`: `Network` implementation for Arbitrum.
- `arb-alloy-rpc-types`: Arbitrum RPC request/response types.
- `arb-alloy-provider`: provider extension traits for `arb_*` RPC methods.
- `arb-sequencer-network`: Arbitrum sequencer feed protocol types.
- `arb-alloy`: umbrella crate re-exporting the components above.
