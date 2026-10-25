//! One pass over the pool's history over Tor, in pages as wide as the server allows (up to
//! 100,000 blocks, halved on a refused range down to 10,000), accepted only whole: leaves
//! `0..nextLeafIndex` at one block, since a short history would show a balance missing notes.
//! A pass after the first reads on from the history it kept, and is checked whole the same way.

#[path = "sync_chain_history.rs"]
mod history;
#[path = "sync_chain_read.rs"]
mod read;

use crate::error::NetError;
use crate::net::rpc::progress::{self, Stop};
use crate::net::tor::Tor;
pub use history::History;
use read::fetch_from;

/// `nextLeafIndex()`, from the compiled pool's method table.
const NEXT_LEAF_INDEX: [u8; 4] = [0x0b, 0xe4, 0xf4, 0x22];
/// The narrowest page: what every free tier serves, and the window a scan always used.
const WINDOW: u64 = 10_000;
/// The widest page tried first. Tenderly's Sepolia gateway answers a 100,000-block page of
/// NoteCommitted whole (some 270 KB); ethpandaops and publicnode refuse past 50,000, and are
/// read in halves.
const WIDEST: u64 = 100_000;
/// The histories a sync reads, for its progress.
const HISTORIES: u32 = 4;
/// What each history is, as a failed scan names the step it stopped at.
const HISTORY_NAMES: [&str; 4] =
    ["the deposits' history", "the roots' history", "the outputs' history", "the spends' history"];

/// Fetch the history, refusing any RPC whose leaves are not all of `0..n`.
pub fn fetch_history(tor: &Tor, hosts: &[&'static str]) -> Result<History, NetError> {
    fetch_history_until(tor, hosts, Stop::never(), None)
}

/// The same, stopped at its next request once told to `stop`, as when the wallet locks. With
/// a history `kept` from an earlier read, only the blocks from a little under its head are
/// asked for, and the whole is checked as a first read is. A server that disagrees with what
/// was kept is read whole instead.
pub fn fetch_history_until(
    tor: &Tor,
    hosts: &[&'static str],
    stop: Stop<'_>,
    kept: Option<&History>,
) -> Result<History, NetError> {
    let mut last = NetError::Transport;
    for host in hosts {
        let read = match fetch_from(tor, host, stop, kept) {
            Err(NetError::ReplyShape) if kept.is_some() => fetch_from(tor, host, stop, None),
            read => read,
        };
        match read {
            Ok(h) => return Ok(h),
            Err(NetError::Stopped) => {
                progress::set(None);
                return Err(NetError::Stopped);
            }
            Err(e) => last = e,
        }
    }
    Err(last)
}
