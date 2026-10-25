//! One RPC's read of the pool's histories, accepted only whole.

use super::super::history_join::{joined, read_from};
use super::{History, HISTORIES, HISTORY_NAMES, NEXT_LEAF_INDEX, WIDEST, WINDOW};
use crate::discovery::{first_gap, note_commitments, Log};
use crate::error::NetError;
use crate::net::pool::{ACTIVE, NOTE_COMMITTED, NULLIFIER_SPENT, OUTPUT_NOTE, ROOT_COMMITTED};
use crate::net::rpc::progress::{self, ScanProgress, Stop};
use crate::net::rpc::{calls_at_over_tor, head_over_tor, pages_over_tor, RawLog};
use crate::net::tor::{Purpose, Tor};

pub(super) fn fetch_from(
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
    let registered = super::super::registry::registered(tor, host, &roots, head)?;
    let outputs = pages(3, OUTPUT_NOTE, kept.map(|k| &k.outputs))?;
    let nullifiers = pages(4, NULLIFIER_SPENT, kept.map(|k| &k.nullifiers))?;
    progress::set(None);
    Ok(History { head, committed, outputs, nullifiers, roots, registered })
}
