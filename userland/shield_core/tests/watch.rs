//! A deposit becomes money only if the held note recomputes to the leaf the pool stored.
//!
//! The pool cannot check that, since the owner digest is opaque to it, so the client must: a
//! matching deposit is accepted, a mismatched leaf is refused, and the escrowed value must be
//! the value the note was sealed to.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::discovery::{accept_incoming, confirm_deposit, Leaf, WatchReject};
use nox_shield_core::keys::{Account, Address};
use nox_shield_core::notes::{commitment, commitment_bytes, seal_note, wire_digest, NotePlaintext};
use nox_shield_core::prover::pool_hasher;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn note_to(addr: &Address, value: u64) -> NotePlaintext {
    NotePlaintext { value, asset_id: 1, blinding: [11, 22, 33, 44], spend_pk: addr.spend_pk }
}

/// The leaf the pool would emit for a note, as the wallet computes it.
fn emitted_leaf(plain: &NotePlaintext) -> [u8; 32] {
    let cm = commitment(&pool_hasher(), &plain.note());
    wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()])
}

#[test]
fn a_matching_deposit_is_accepted() {
    let me = account().address();
    let plain = note_to(&me, 1_000_000);
    let leaf = emitted_leaf(&plain);
    let rec = confirm_deposit(&plain, 7, &leaf, 1_000_000).expect("a matching deposit is money");
    assert_eq!(rec.leaf_index, 7);
    assert_eq!(rec.plain.value, 1_000_000);
}

#[test]
fn a_leaf_that_does_not_match_is_refused() {
    // The 162-notes case: the pool stored some other leaf. The note recomputes
    // to a different one, so it can never be spent and must not be counted.
    let me = account().address();
    let plain = note_to(&me, 1_000_000);
    let mut wrong = emitted_leaf(&plain);
    wrong[0] ^= 0x01;
    assert_eq!(confirm_deposit(&plain, 7, &wrong, 1_000_000).unwrap_err(), WatchReject::Leaf);
}

#[test]
fn a_value_that_is_not_the_sealed_one_is_refused() {
    let me = account().address();
    let plain = note_to(&me, 1_000_000);
    let leaf = emitted_leaf(&plain);
    // The pool escrowed a different amount than the note was sealed to.
    assert_eq!(confirm_deposit(&plain, 7, &leaf, 999_999).unwrap_err(), WatchReject::Value);
}

fn leaf_for(plain: &NotePlaintext, to: &Address, leaf_index: u64) -> Leaf {
    let cm = commitment(&pool_hasher(), &plain.note());
    let cm_bytes = commitment_bytes(&cm);
    let cipher = seal_note(plain, to, &cm_bytes).expect("seal");
    let mut encrypted_note = [0u8; 128];
    encrypted_note[..32].copy_from_slice(&cipher.eph_pk);
    encrypted_note[32..].copy_from_slice(&cipher.sealed);
    Leaf { commitment: emitted_leaf(plain), leaf_index, view_tag: cipher.view_tag, encrypted_note }
}

#[test]
fn a_note_created_for_us_is_accepted() {
    let acct = account();
    let me = acct.address();
    let plain = note_to(&me, 500_000);
    let leaf = leaf_for(&plain, &me, 12);
    let rec = accept_incoming(&acct, &leaf).expect("our incoming note opens");
    assert_eq!(rec.plain.value, 500_000);
    assert_eq!(rec.leaf_index, 12);
}

#[test]
fn a_note_created_for_someone_else_is_not_ours() {
    let acct = account();
    let other = account().address();
    let plain = note_to(&other, 500_000);
    let leaf = leaf_for(&plain, &other, 12);
    assert_eq!(accept_incoming(&acct, &leaf).unwrap_err(), WatchReject::NotOurs);
}
