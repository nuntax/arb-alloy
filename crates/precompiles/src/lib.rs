#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Canonical addresses for all Arbitrum precompile contracts.
pub mod addresses;

mod interfaces;

pub use interfaces::ArbAddressTable;
pub use interfaces::ArbAggregator;
pub use interfaces::ArbDebug;
pub use interfaces::ArbFunctionTable;
pub use interfaces::ArbGasInfo;
pub use interfaces::ArbInfo;
pub use interfaces::ArbOwner;
pub use interfaces::ArbOwnerPublic;
pub use interfaces::ArbRetryableTx;
pub use interfaces::ArbStatistics;
pub use interfaces::ArbSys;
pub use interfaces::ArbWasm;
pub use interfaces::ArbWasmCache;
pub use interfaces::ArbosActs;
pub use interfaces::NodeInterface;
