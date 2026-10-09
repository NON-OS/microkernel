//! Recording what one fetched history holds for each account, own or view-only. Every account
//! reads the same history, so the network learns nothing of how many there are.

use crate::discovery::{note_commitments, Log};
use crate::error::WalletError;
use crate::notes::NoteStatus;
use crate::store::Row;
use crate::wallet::watch::Watched;
use crate::wallet::{scan_history, scan_history_as, History, Session};

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

/// Record what the history holds for one view-only account. It has no deposits of its own.
pub(super) fn sync_watched(w: &mut Watched, history: &History) -> Result<(), WalletError> {
    let held = w.state.held();
    let scan = scan_history_as(history, &w.viewer(), &[], &held);
    let known: Vec<[u64; 4]> = held.iter().map(|n| n.cm).collect();
    let rows: Vec<Row> = scan
        .received
        .into_iter()
        .filter(|n| !known.contains(&n.cm))
        .map(Row::Found)
        .chain(scan.spent.into_iter().map(|cm| Row::Status { cm, status: NoteStatus::Spent }))
        .collect();
    for row in rows {
        w.log.append(&row)?;
        w.state.apply(row);
    }
    w.state.mark_pool(&leaves(history));
    Ok(())
}

/// The active pool's leaves by position, which a held note must match to count.
fn leaves(history: &History) -> std::collections::BTreeMap<u64, [u8; 32]> {
    let logs: Vec<Log> =
        history.committed.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect();
    note_commitments(&logs)
}

/// What a sync found, into the history: each deposit stored, and each note received that is
/// not the change of one of this wallet's own spends. An account's first scan, as after a
/// restore from the words, finds every past note with no rows to tell change from a payment,
/// so it records nothing but the deposits this store already says it sent: the history starts
/// from what it knows rather than invented.
fn arrivals(
    done: &[crate::store::activity::Activity],
    received: &[crate::notes::NoteRecord],
    deposited: &[crate::notes::NoteRecord],
    first_scan: bool,
) -> Vec<crate::store::activity::Activity> {
    use crate::store::activity::{is_change, Activity, DEPOSIT_IN, DEPOSIT_SENT, RECEIVED};
    let at = crate::wallet::history::clock();
    let sent_here = |cm: &[u64; 4]| done.iter().any(|a| a.kind == DEPOSIT_SENT && a.cm == *cm);
    let stored = deposited
        .iter()
        .filter(|n| !first_scan || sent_here(&n.cm))
        .map(|n| Activity { cm: n.cm, ..Activity::new(DEPOSIT_IN, at) });
    if first_scan {
        return stored.collect();
    }
    // A note of no value is a spend's filler, never a payment.
    let paid = received.iter().filter(|n| n.plain.value > 0 && !is_change(done, &n.cm)).map(|n| {
        Activity {
            asset_id: n.plain.asset_id,
            value: n.plain.value,
            cm: n.cm,
            ..Activity::new(RECEIVED, at)
        }
    });
    stored.chain(paid).collect()
}

fn count(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

#[cfg(test)]
#[path = "sync_each_test.rs"]
mod sync_each_test;
