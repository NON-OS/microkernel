//! One pass over the pool's history over Tor, in pages as wide as the server allows (up to
//! 100,000 blocks, halved on a refused range down to 10,000), accepted only whole: leaves
//! `0..nextLeafIndex` at one block, since a short history would show a balance missing notes.
//! A pass after the first reads on from the history it kept, and is checked whole the same way.

use super::history_join::{joined, read_from};
use crate::discovery::{first_gap, note_commitments, Log};
use crate::error::NetError;
use crate::net::pool::{ACTIVE, NOTE_COMMITTED, NULLIFIER_SPENT, OUTPUT_NOTE, ROOT_COMMITTED};
use crate::net::rpc::progress::{self, ScanProgress, Stop};
use crate::net::rpc::{calls_at_over_tor, head_over_tor, pages_over_tor, RawLog};
use crate::net::tor::{Purpose, Tor};

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

/// The pool's three event histories, whole, from the first RPC that serves them so.
#[derive(Clone)]
pub struct History {
    pub head: u64,
    pub committed: Vec<RawLog>,
    pub outputs: Vec<RawLog>,
    pub nullifiers: Vec<RawLog>,
    /// Every `RootCommitted`, for anchoring a spend to the newest root.
    pub roots: Vec<RawLog>,
    /// The newest roots the association registry holds, or none when the pool has no registry.
    pub registered: Option<Vec<[u8; 32]>>,
}

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

fn fetch_from(
    tor: &Tor,
    host: &'static str,
    stop: Stop<'_>,
    kept: Option<&History>,
) -> Result<History, NetError> {
    stop.check()?;
    progress::at(host, "the chain head");
    let head = head_over_tor(tor, host)?;
    stop.check()?;
    progress::at(host, "the pool's leaf count");
    let next = calls_at_over_tor(
        tor,
        Purpose::Scan,
        host,
        ACTIVE.address,
        &[NEXT_LEAF_INDEX.to_vec()],
        head,
    )?;
    let next_leaf = next.first().and_then(|w| w.get(24..32)).ok_or(NetError::ReplyShape)?;
    let mut be = [0u8; 8];
    be.copy_from_slice(next_leaf);
    let expected = u64::from_be_bytes(be);
    let from = kept.map_or(ACTIVE.deploy_block, |k| read_from(k.head, ACTIVE.deploy_block));
    let pages = |history: u32, topic, kept: Option<&Vec<RawLog>>| {
        progress::at(host, HISTORY_NAMES[(history as usize).saturating_sub(1).min(3)]);
        progress::set(Some(ScanProgress {
            history,
            histories: HISTORIES,
            block: from,
            head,
            from,
        }));
        let read =
            pages_over_tor(tor, host, ACTIVE.address, topic, (from, head), (WINDOW, WIDEST), stop);
        if read.is_err() {
            progress::set(None);
        }
        read.map(|fresh| joined(kept.map_or(&[][..], Vec::as_slice), from, fresh))
    };
    let committed = pages(1, NOTE_COMMITTED, kept.map(|k| &k.committed))?;
    let logs: Vec<Log> =
        committed.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect();
    let leaves = note_commitments(&logs);
    // Whole, or refused: no hole, and every leaf the pool says it holds.
    if first_gap(&leaves).is_some() || u64::try_from(leaves.len()).ok() != Some(expected) {
        progress::set(None);
        return Err(NetError::ReplyShape);
    }
    let roots = pages(2, ROOT_COMMITTED, kept.map(|k| &k.roots))?;
    stop.check()?;
    progress::at(host, "the association registry");
    let registered = super::registry::registered(tor, host, &roots, head)?;
    let outputs = pages(3, OUTPUT_NOTE, kept.map(|k| &k.outputs))?;
    let nullifiers = pages(4, NULLIFIER_SPENT, kept.map(|k| &k.nullifiers))?;
    progress::set(None);
    Ok(History { head, committed, outputs, nullifiers, roots, registered })
}
