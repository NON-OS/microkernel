//! Reading the chain over a proxy, one connection per call. The endpoint sees a scan of the
//! pool and never an address of the wallet. Paging is the caller's loop.

#[path = "fetch_calls.rs"]
mod calls;

pub use calls::{call_over_tor, calls_at_over_tor, calls_over_tor};

use super::http::post;
use super::progress::Stop;
use super::request;
use super::response::{self, RawLog};
use crate::error::NetError;
use crate::net::tor::{tls, Purpose, Tor};

fn text(bytes: &[u8]) -> Result<&str, NetError> {
    core::str::from_utf8(bytes).map_err(|_| NetError::ReplyShape)
}

/// One page of an event's logs from an HTTPS RPC, over the wallet's own Tor on
/// circuits reserved for scanning, TLS end to end so the exit sees ciphertext.
pub fn page_over_tor(
    tor: &Tor,
    host: &str,
    address: &str,
    topic0: &str,
    from_block: u64,
    to_block: u64,
) -> Result<Vec<RawLog>, NetError> {
    let stream = tls(tor.connect(Purpose::Scan, host, 443)?, host)?;
    let body = request::get_logs(address, topic0, from_block, to_block);
    let reply = post(stream, host, "/", &body)?;
    response::logs(text(&reply)?)
}

/// The chain head from an HTTPS RPC over the wallet's own Tor.
pub fn head_over_tor(tor: &Tor, host: &str) -> Result<u64, NetError> {
    let stream = tls(tor.connect(Purpose::Scan, host, 443)?, host)?;
    let reply = post(stream, host, "/", &request::block_number())?;
    response::block_number(text(&reply)?)
}

/// Every log of one event over `blocks`, first to last, in pages as wide as the server allows:
/// `windows` is the narrowest and the widest page, a page starting at the widest and halved on
/// each refused range, since free RPC tiers cap a call. A page refused at the narrowest fails
/// the whole scan, so no silent gap is left. A read told to `stop` stops before its next page.
pub fn pages_over_tor(
    tor: &Tor,
    host: &str,
    address: &str,
    topic0: &str,
    blocks: (u64, u64),
    windows: (u64, u64),
    stop: Stop<'_>,
) -> Result<Vec<RawLog>, NetError> {
    let ((from_block, to_block), (narrowest, widest)) = (blocks, windows);
    super::paging::paged(
        from_block,
        to_block,
        widest.max(narrowest),
        narrowest,
        |from, to| {
            stop.check()?;
            page_over_tor(tor, host, address, topic0, from, to)
        },
        super::progress::read_to,
    )
}
