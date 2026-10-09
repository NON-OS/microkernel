//! Note discovery: every output is tried locally, the only option that leaks nothing.
//! The view tag cuts the cost to a key agreement and a byte compare per output, with decryption on
//! about one in 256. The options weighed are in docs/04-discovery.md.

mod chain;
mod chain_read;
mod chain_viewer;
mod logs;
mod output;
mod scan;
mod spent;
mod stats;
mod watch;
pub(crate) mod words;

pub use chain::{scan_chain, ChainScan};
pub use chain_viewer::scan_viewer;
pub use logs::{first_gap, note_commitments, output_leaves, scan_logs, Log};
pub use output::PoolOutput;
pub use scan::scan;
pub use spent::spent_commitments;
pub use stats::ScanStats;
pub use watch::{accept_incoming, confirm_deposit, Leaf, WatchReject};
