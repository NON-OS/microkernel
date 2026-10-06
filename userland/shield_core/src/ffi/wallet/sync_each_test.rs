use super::arrivals;
use crate::notes::{NotePlaintext, NoteRecord, NoteStatus};
use crate::store::activity::{Activity, DEPOSIT_IN, RECEIVED, SENT};

fn note(value: u64, cm: [u64; 4]) -> NoteRecord {
    NoteRecord {
        plain: NotePlaintext { value, asset_id: 1, blinding: [0; 4], spend_pk: [0; 4] },
        leaf_index: 0,
        cm,
        status: NoteStatus::Unspent,
        found_at: 0,
    }
}

#[test]
fn a_first_scan_records_nothing_however_much_it_finds() {
    let past_change = note(4, [1, 1, 1, 1]);
    let past_payment = note(9, [2, 2, 2, 2]);
    let past_deposit = note(5, [3, 3, 3, 3]);
    let got = arrivals(&[], &[past_change, past_payment], &[past_deposit], true);
    assert!(got.is_empty(), "a restore starts with an empty history, not an invented one");
}

#[test]
fn a_first_scan_still_stores_a_deposit_this_store_sent() {
    let mut sent = Activity::new(crate::store::activity::DEPOSIT_SENT, 1);
    sent.cm = [3, 3, 3, 3];
    let ours = note(5, [3, 3, 3, 3]);
    let other = note(7, [8, 8, 8, 8]);
    let got = arrivals(&[sent], &[note(9, [2, 2, 2, 2])], &[ours, other], true);
    let read: Vec<(u8, [u64; 4])> = got.iter().map(|a| (a.kind, a.cm)).collect();
    assert_eq!(read, [(DEPOSIT_IN, [3, 3, 3, 3])], "only the deposit it sent, nothing received");
}

#[test]
fn a_later_scan_records_a_payment_and_a_stored_deposit_and_skips_change() {
    let mut spent = Activity::new(SENT, 1);
    spent.cm = [1, 1, 1, 1];
    let change = note(4, [1, 1, 1, 1]);
    let payment = note(9, [2, 2, 2, 2]);
    let filler = note(0, [6, 6, 6, 6]);
    let deposit = note(5, [3, 3, 3, 3]);
    let got = arrivals(&[spent], &[change, payment, filler], &[deposit], false);
    let read: Vec<(u8, u64, [u64; 4])> = got.iter().map(|a| (a.kind, a.value, a.cm)).collect();
    assert_eq!(read, [(DEPOSIT_IN, 0, [3, 3, 3, 3]), (RECEIVED, 9, [2, 2, 2, 2])]);
}
