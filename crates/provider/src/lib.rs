use alloy_network::Network;
use alloy_provider::Provider;
use alloy_transport::TransportResult;

use alloy_rpc_types_eth::BlockNumberOrTag;
use arb_alloy_network::Arbitrum;
use arb_alloy_rpc_types::{ArbMaintenanceStatus, ArbMinRequiredNitroVersion, ArbRawBlockMetadata};

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait ArbProviderExt<N: Network = Arbitrum>: Send + Sync {
    /// Nitro reference: `nitro/arbnode/api.go` -> `GetL1Confirmations`.
    async fn arb_get_l1_confirmations(&self, block_num: u64) -> TransportResult<u64>;

    /// Nitro reference: `nitro/arbnode/api.go` -> `FindBatchContainingBlock`.
    async fn arb_find_batch_containing_block(&self, block_num: u64) -> TransportResult<u64>;

    /// Nitro reference: `nitro/arbnode/nitro-version-alerter/server.go` -> `GetMinRequiredNitroVersion`.
    async fn arb_get_min_required_nitro_version(
        &self,
    ) -> TransportResult<ArbMinRequiredNitroVersion>;

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
    async fn arb_get_l1_confirmations(&self, block_num: u64) -> TransportResult<u64> {
        self.client()
            .request("arb_getL1Confirmations", (block_num,))
            .await
    }

    async fn arb_find_batch_containing_block(&self, block_num: u64) -> TransportResult<u64> {
        self.client()
            .request("arb_findBatchContainingBlock", (block_num,))
            .await
    }

    async fn arb_get_min_required_nitro_version(
        &self,
    ) -> TransportResult<ArbMinRequiredNitroVersion> {
        self.client()
            .request("arb_getMinRequiredNitroVersion", ())
            .await
    }

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
