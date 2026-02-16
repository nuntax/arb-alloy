#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod ext;

pub use ext::arb::ArbProviderExt;
pub use ext::arbdebug::ArbDebugProviderExt;
pub use ext::arbtrace::ArbTraceProviderExt;
pub use ext::auctioneer::AuctioneerProviderExt;
pub use ext::timeboost::TimeboostProviderExt;
