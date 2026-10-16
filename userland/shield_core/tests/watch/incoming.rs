//! A note that arrives as a ciphertext: ours opens, another's is not ours.

use super::{account, emitted_leaf, note_to};
use nox_shield_core::discovery::{accept_incoming, Leaf, WatchReject};
use nox_shield_core::keys::Address;
use nox_shield_core::notes::{commitment, commitment_bytes, seal_note, NotePlaintext};
use nox_shield_core::prover::pool_hasher;

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
