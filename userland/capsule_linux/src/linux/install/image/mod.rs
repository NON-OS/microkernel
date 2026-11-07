//! An app installed from its prebuilt image (nonos_app_image): the chunk
//! list fetched and held to the market's pin, each chunk fetched once,
//! held to its digest and kept in a cache named by the pin, then every
//! member decompressed, held to its digest and placed as a package is. No
//! package index is read and nothing is resolved here.

mod assemble;
pub(super) mod bound;
mod cache;
mod evict;
#[cfg(feature = "install-fast-route")]
mod fast;
mod fetch;
pub(super) mod named;
mod open;
pub(super) mod placed;
pub(super) mod prune;
mod reuse;
pub(super) mod room;
pub(super) mod route_pick;
mod run;
pub(super) mod sched;
mod span;
mod window;

pub use run::install;
