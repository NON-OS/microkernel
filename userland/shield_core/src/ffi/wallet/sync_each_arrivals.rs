//! What a sync found, as history rows: deposits stored and payments received, never change.

/// What a sync found, into the history: each deposit stored, and each note received that is
/// not the change of one of this wallet's own spends. An account's first scan, as after a
/// restore from the words, finds every past note with no rows to tell change from a payment,
/// so it records nothing but the deposits this store already says it sent: the history starts
/// from what it knows rather than invented.
pub(super) fn arrivals(
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
