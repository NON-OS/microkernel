//! The scan of the two events: the owned note read, a stranger's and a lone output skipped.

use super::{account, logs_for, note_to};
use nox_shield_core::discovery::{scan_logs, Log};

#[test]
fn the_wallet_reads_its_own_note_from_the_logs() {
    let acct = account();
    let me = acct.address();
    let plain = note_to(&me, 750_000);
    let (committed, output, data) = logs_for(&plain, &me, 41);

    let nc = [Log { topics: &committed, data: &[] }];
    let on = [Log { topics: &output, data: &data }];
    let notes = scan_logs(&acct, &nc, &on);
    assert_eq!(notes.len(), 1, "the wallet's own note is read from the chain");
    assert_eq!(notes[0].plain.value, 750_000);
    assert_eq!(notes[0].leaf_index, 41);
}

#[test]
fn a_note_for_someone_else_is_not_read() {
    let acct = account();
    let other = account().address();
    let plain = note_to(&other, 750_000);
    let (committed, output, data) = logs_for(&plain, &other, 41);

    let nc = [Log { topics: &committed, data: &[] }];
    let on = [Log { topics: &output, data: &data }];
    assert!(scan_logs(&acct, &nc, &on).is_empty(), "a stranger's note is not ours");
}

#[test]
fn an_output_with_no_committed_leaf_is_skipped() {
    let acct = account();
    let me = acct.address();
    let plain = note_to(&me, 750_000);
    let (_committed, output, data) = logs_for(&plain, &me, 41);
    // The OutputNote arrived but its NoteCommitted did not: nothing to join to.
    let on = [Log { topics: &output, data: &data }];
    assert!(scan_logs(&acct, &[], &on).is_empty(), "no commitment, no note");
}

#[test]
fn a_scan_with_a_hole_is_detected() {
    use nox_shield_core::discovery::first_gap;
    use std::collections::BTreeMap;
    let mut m = BTreeMap::new();
    for i in [0u64, 1, 2, 4, 5] {
        m.insert(i, [0u8; 32]);
    }
    assert_eq!(first_gap(&m), Some(3), "leaf 3 was dropped by the RPC");
    m.insert(3, [0u8; 32]);
    assert_eq!(first_gap(&m), None, "0..6 is whole");
}
