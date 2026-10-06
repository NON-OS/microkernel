//! Scanning a fetched history for one account, or for what a view key holds. Local, so it runs
//! under the session lock once the history is in.

use super::sync_chain::{fetch_history, History};
use crate::discovery::{scan_viewer, ChainScan, Log};
use crate::error::NetError;
use crate::keys::{Account, Viewer};
use crate::net::rpc::RawLog;
use crate::net::tor::Tor;
use crate::notes::{NotePlaintext, NoteRecord};

/// This wallet's notes from a fetched history. Local, so it runs under the session lock.
pub fn scan_history(
    h: &History,
    account: &Account,
    pending: &[NotePlaintext],
    held: &[&NoteRecord],
) -> ChainScan {
    scan_history_as(h, &account.viewer(), pending, held)
}

/// The same with only what a view key holds.
pub fn scan_history_as(
    h: &History,
    viewer: &Viewer,
    pending: &[NotePlaintext],
    held: &[&NoteRecord],
) -> ChainScan {
    let (c, o, n) = (view(&h.committed), view(&h.outputs), view(&h.nullifiers));
    scan_viewer(viewer, &c, &o, &n, pending, held)
}

pub(crate) fn view(logs: &[RawLog]) -> Vec<Log<'_>> {
    logs.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect()
}

/// This wallet's notes, from a whole history fetched now.
pub fn sync_chain(
    tor: &Tor,
    hosts: &[&'static str],
    account: &Account,
    pending: &[NotePlaintext],
    held: &[&NoteRecord],
) -> Result<(u64, ChainScan), NetError> {
    let h = fetch_history(tor, hosts)?;
    Ok((h.head, scan_history(&h, account, pending, held)))
}
