//! Reading the chain over JSON-RPC: the calls the wallet makes and the replies
//! it parses. The transport is one connection per call through the proxy, so
//! the RPC endpoint sees a scan of the whole pool and no address of the wallet.
mod fetch;
mod http;
mod paging;
#[cfg(test)]
#[path = "paging_test.rs"]
mod paging_test;
pub mod progress;
#[cfg(test)]
#[path = "http_test.rs"]
mod http_test;
mod request;
mod response;

pub use fetch::{
    call_over_tor, calls_at_over_tor, calls_over_tor, head_over_tor, page_over_tor,
    pages_over_tor,
};
pub(crate) use http::{exchange, post};
pub use request::{block_number, eth_call, eth_call_batch, eth_call_batch_at, get_logs};
pub(crate) use response::{after, hex_bytes, objects, quoted};
pub use response::{
    block_number as parse_block_number, call_result, call_results, logs as parse_logs, RawLog,
};
