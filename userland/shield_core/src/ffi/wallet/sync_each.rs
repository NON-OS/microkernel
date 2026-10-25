//! Recording what one fetched history holds for each account, own or view-only. Every account
//! reads the same history, so the network learns nothing of how many there are.

#[path = "sync_each_arrivals.rs"]
mod arrivals;
#[path = "sync_each_watched.rs"]
mod watched;

use arrivals::arrivals;
pub(super) use watched::sync_watched;

use crate::discovery::{note_commitments, Log};
use crate::error::WalletError;
use crate::notes::NoteStatus;
use crate::store::Row;
use crate::wallet::{scan_history, History, Session};

/// Record what the history holds for account `index`: received, deposited, spent.
pub(super) fn sync_account(
    s: &mut Session,
    index: u32,
    history: &History,
) -> Result<[u32; 3], WalletError> {
    let Some(slot) = s.at(index) else { return Ok([0; 3]) };
    let held = slot.state().held();
    let scan = scan_history(history, slot.account(), &slot.state().pending_deposits(), &held);
    let known: Vec<[u64; 4]> = held.iter().map(|n| n.cm).collect();
    let fresh = |cm: &[u64; 4]| !known.contains(cm);
    let received: Vec<_> = scan.received.into_iter().filter(|n| fresh(&n.cm)).collect();
    let deposited: Vec<_> = scan.deposited.into_iter().filter(|n| fresh(&n.cm)).collect();
    let found = [count(received.len()), count(deposited.len()), count(scan.spent.len())];
    // The scan this account had reached before: none on its first, as after a restore.
    let reached = slot.state().cursor();
    let events = arrivals(slot.state().activity(), &received, &deposited, reached == 0);
    for note in received.into_iter().chain(deposited) {
        s.record_to(index, Row::Found(note))?;
    }
    for a in events {
        s.record_to(index, Row::Activity(a))?;
    }
    for cm in scan.spent {
        s.record_to(index, Row::Status { cm, status: NoteStatus::Spent })?;
    }
    // How far the scan reached, kept only when it moved, so the next sync knows it is not the
    // account's first. The scan itself always reads the whole history.
    let now_at = u64::try_from(history.committed.len()).unwrap_or(u64::MAX);
    if now_at > reached {
        s.record_to(index, Row::Cursor(now_at))?;
    }
    s.mark_pool_at(index, &leaves(history));
    Ok(found)
}
/// The active pool's leaves by position, which a held note must match to count.
fn leaves(history: &History) -> std::collections::BTreeMap<u64, [u8; 32]> {
    let logs: Vec<Log> =
        history.committed.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect();
    note_commitments(&logs)
}

fn count(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

#[cfg(test)]
#[path = "sync_each_test.rs"]
mod sync_each_test;
