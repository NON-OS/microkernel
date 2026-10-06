//! The wallet marks its own notes spent from the pool's nullifiers.
//!
//! The pool publishes a nullifier it cannot link to a commitment. The wallet holds that link
//! and must build it, or it shows spent notes as money.
//! These pin the derivation's security-relevant properties and the matcher
//! that uses it.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::discovery::spent_commitments;
use nox_shield_core::keys::{note_nullifier_wire, Account};
use nox_shield_core::notes::{NotePlaintext, NoteRecord, NoteStatus};

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn record(acct: &Account, blinding: [u64; 4], leaf_index: u64) -> NoteRecord {
    let plain =
        NotePlaintext { value: 42, asset_id: 0, blinding, spend_pk: acct.address().spend_pk };
    NoteRecord {
        plain,
        leaf_index,
        cm: blinding,
        status: NoteStatus::Unspent,
        found_at: leaf_index,
    }
}

#[test]
fn a_nullifier_is_deterministic() {
    let acct = account();
    let cm = [1, 2, 3, 4];
    assert_eq!(note_nullifier_wire(&acct, &cm, 9), note_nullifier_wire(&acct, &cm, 9));
}

#[test]
fn the_position_moves_the_nullifier() {
    let acct = account();
    let cm = [1, 2, 3, 4];
    assert_ne!(
        note_nullifier_wire(&acct, &cm, 9),
        note_nullifier_wire(&acct, &cm, 10),
        "two identical notes at different leaves must not share a nullifier"
    );
}

#[test]
fn a_different_note_and_a_different_account_both_move_it() {
    let a = account();
    let b = account();
    let cm = [1, 2, 3, 4];
    assert_ne!(note_nullifier_wire(&a, &cm, 0), note_nullifier_wire(&a, &[9, 9, 9, 9], 0));
    assert_ne!(note_nullifier_wire(&a, &cm, 0), note_nullifier_wire(&b, &cm, 0));
}

#[test]
fn the_matcher_marks_only_the_spent_note() {
    let acct = account();
    let mine = record(&acct, [10, 20, 30, 40], 3);
    let other = record(&acct, [50, 60, 70, 80], 4);
    let spent = [note_nullifier_wire(&acct, &mine.cm, mine.leaf_index)];
    let hit = spent_commitments(&acct, &[mine, other], &spent);
    assert_eq!(hit, alloc_vec([10, 20, 30, 40]), "only the note whose nullifier was published");
}

fn alloc_vec(cm: [u64; 4]) -> Vec<[u64; 4]> {
    vec![cm]
}
