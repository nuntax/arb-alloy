use alloc::{boxed::Box, vec::Vec};
use alloy_network::Network;
use alloy_provider::Provider;
use alloy_rpc_types_eth::BlockNumberOrTag;
use alloy_transport::TransportResult;
use arb_alloy_network::Arbitrum;
use arb_alloy_rpc_types::{ArbMaintenanceStatus, ArbRawBlockMetadata};

/// Provider extension trait for the `arb_*` JSON-RPC namespace.
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait ArbProviderExt<N: Network = Arbitrum>: Send + Sync {
    /// Checks whether the transaction publisher is healthy.
    ///
    /// Nitro reference: `execution/gethexec/api.go` -> `CheckPublisherHealth`.
    async fn arb_check_publisher_health(&self) -> TransportResult<()>;

    /// Returns current maintenance status of the execution layer.
    ///
    /// Nitro reference: `execution/gethexec/api.go` -> `MaintenanceStatus`.
    async fn arb_maintenance_status(&self) -> TransportResult<ArbMaintenanceStatus>;

    /// Returns raw block metadata for a range of blocks.
    ///
    /// Nitro reference: `execution/gethexec/api.go` -> `GetRawBlockMetadata`.
    async fn arb_get_raw_block_metadata(
        &self,
        from_block: BlockNumberOrTag,
        to_block: BlockNumberOrTag,
    ) -> TransportResult<Vec<ArbRawBlockMetadata>>;

    /// Returns the number of L1 confirmations for the given L2 block.
    ///
    /// Nitro reference: `arbnode/api.go` -> `GetL1Confirmations`.
    async fn arb_get_l1_confirmations(&self, block_num: u64) -> TransportResult<u64>;

    /// Returns the batch number that contains the given L2 block.
    ///
    /// Nitro reference: `arbnode/api.go` -> `FindBatchContainingBlock`.
    async fn arb_find_batch_containing_block(&self, block_num: u64) -> TransportResult<u64>;
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
    /// Note: not yet available on Nitro as of 19th feb 2026, will error with "method not found" until then.
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
}

#[cfg(test)]
mod tests {
    use alloy_provider::ProviderBuilder;
    use alloy_rpc_types_eth::BlockNumberOrTag;
    use alloy_transport::mock::Asserter;
    use arb_alloy_rpc_types::{ArbMaintenanceStatus, ArbRawBlockMetadata};

    use super::ArbProviderExt;

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
        asserter.push_success(&42_u64);
        asserter.push_success(&99_u64);

        provider.arb_check_publisher_health().await.unwrap();

        let maintenance = provider.arb_maintenance_status().await.unwrap();
        assert!(maintenance.is_running);

        let metadata = provider
            .arb_get_raw_block_metadata(BlockNumberOrTag::Number(10), BlockNumberOrTag::Number(11))
            .await
            .unwrap();
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].block_number, 123);

        let l1_confs = provider.arb_get_l1_confirmations(100).await.unwrap();
        assert_eq!(l1_confs, 42);

        let batch = provider.arb_find_batch_containing_block(200).await.unwrap();
        assert_eq!(batch, 99);
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

        let err = provider.arb_get_l1_confirmations(0).await.unwrap_err();
        assert!(err.to_string().contains("arb_getL1Confirmations"), "{err}");

        let err = provider
            .arb_find_batch_containing_block(0)
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("arb_findBatchContainingBlock"),
            "{err}"
        );
    }
}
