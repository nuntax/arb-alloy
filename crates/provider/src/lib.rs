#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use alloy_network::Network;
use alloy_provider::Provider;
use alloy_transport::TransportResult;

use alloy_rpc_types_eth::BlockNumberOrTag;
use arb_alloy_network::Arbitrum;
use arb_alloy_rpc_types::{ArbMaintenanceStatus, ArbRawBlockMetadata};

/// Provider extension trait for Arbitrum-specific JSON-RPC methods.
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait ArbProviderExt<N: Network = Arbitrum>: Send + Sync {
    /// Nitro reference: `nitro/execution/gethexec/api.go` -> `CheckPublisherHealth`.
    async fn arb_check_publisher_health(&self) -> TransportResult<()>;

    /// Nitro reference: `nitro/execution/gethexec/api.go` -> `MaintenanceStatus`.
    async fn arb_maintenance_status(&self) -> TransportResult<ArbMaintenanceStatus>;

    /// Nitro reference: `nitro/execution/gethexec/api.go` -> `GetRawBlockMetadata`.
    async fn arb_get_raw_block_metadata(
        &self,
        from_block: BlockNumberOrTag,
        to_block: BlockNumberOrTag,
    ) -> TransportResult<Vec<ArbRawBlockMetadata>>;
}

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl<N, P> ArbProviderExt<N> for P
where
    N: Network,
    P: Provider<N>,
{
    async fn arb_check_publisher_health(&self) -> TransportResult<()> {
        self.client().request("arb_checkPublisherHealth", ()).await
    }

    async fn arb_maintenance_status(&self) -> TransportResult<ArbMaintenanceStatus> {
        self.client().request("arb_maintenanceStatus", ()).await
    }

    async fn arb_get_raw_block_metadata(
        &self,
        from_block: BlockNumberOrTag,
        to_block: BlockNumberOrTag,
    ) -> TransportResult<Vec<ArbRawBlockMetadata>> {
        self.client()
            .request("arb_getRawBlockMetadata", (from_block, to_block))
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use alloy_provider::{Provider, ProviderBuilder};
    use alloy_rpc_types_eth::BlockNumberOrTag;
    use arb_alloy_network::Arbitrum;

    use super::ArbProviderExt;
    use alloy_transport::mock::Asserter;
    use arb_alloy_rpc_types::{ArbMaintenanceStatus, ArbRawBlockMetadata};

    #[tokio::test]
    async fn arb_extension_success_paths() {
        let asserter = Asserter::new();
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        asserter.push_success(&());
        asserter.push_success(&ArbMaintenanceStatus { is_running: true });
        asserter.push_success(&vec![ArbRawBlockMetadata {
            block_number: 123,
            raw_metadata: vec![1_u8, 2, 3].into(),
        }]);

        provider.arb_check_publisher_health().await.unwrap();

        let maintenance = provider.arb_maintenance_status().await.unwrap();
        assert!(maintenance.is_running);

        let metadata = provider
            .arb_get_raw_block_metadata(BlockNumberOrTag::Number(10), BlockNumberOrTag::Number(11))
            .await
            .unwrap();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].block_number, 123);
    }

    #[tokio::test]
    async fn arb_extension_uses_expected_rpc_method_names() {
        let asserter = Asserter::new();
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

        let err = provider.arb_check_publisher_health().await.unwrap_err();
        assert!(
            err.to_string().contains("arb_checkPublisherHealth"),
            "{err}"
        );

        let err = provider.arb_maintenance_status().await.unwrap_err();
        assert!(err.to_string().contains("arb_maintenanceStatus"), "{err}");

        let err = provider
            .arb_get_raw_block_metadata(BlockNumberOrTag::Latest, BlockNumberOrTag::Latest)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("arb_getRawBlockMetadata"), "{err}");
    }

    fn arb_rpc_url() -> String {
        env::var("ARB_RPC_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "https://arb1.arbitrum.io/rpc".to_string())
    }

    fn strict_mode() -> bool {
        env::var("ARB_RPC_STRICT")
            .ok()
            .is_some_and(|v| v.trim() == "1" || v.trim().eq_ignore_ascii_case("true"))
    }

    async fn call_or_allow_non_strict<F, T>(name: &str, fut: F) -> Result<(), String>
    where
        F: std::future::Future<Output = alloy_transport::TransportResult<T>>,
    {
        match fut.await {
            Ok(_) => Ok(()),
            Err(err) if err.to_string().contains("error code -32601") => {
                eprintln!("{name} unavailable on this RPC endpoint (method not found): {err}");
                Ok(())
            }
            Err(err) if !strict_mode() => {
                eprintln!("{name} failed (non-fatal in non-strict mode): {err}");
                Ok(())
            }
            Err(err) => Err(format!("{name} failed in strict mode: {err}")),
        }
    }

    #[tokio::test]
    #[ignore = "hits official Arbitrum RPC; run manually"]
    async fn arb_rpc_official_provider_extension_smoke() {
        let provider = ProviderBuilder::<_, _, Arbitrum>::default().connect_http(
            arb_rpc_url()
                .parse()
                .expect("ARB_RPC_URL or fallback URL must be valid"),
        );

        // Public sanity check that transport/network are reachable.
        let latest = provider
            .get_block_number()
            .await
            .expect("failed to read latest block number");

        let _ = latest;
        call_or_allow_non_strict(
            "arb_checkPublisherHealth",
            provider.arb_check_publisher_health(),
        )
        .await
        .expect("arb_checkPublisherHealth check");
        call_or_allow_non_strict("arb_maintenanceStatus", provider.arb_maintenance_status())
            .await
            .expect("arb_maintenanceStatus check");
        call_or_allow_non_strict(
            "arb_getRawBlockMetadata",
            provider.arb_get_raw_block_metadata(BlockNumberOrTag::Latest, BlockNumberOrTag::Latest),
        )
        .await
        .expect("arb_getRawBlockMetadata check");
    }
}
