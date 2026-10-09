// Tests assert by panicking and read memory through a pointer, so those lints are off here.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use core::mem::MaybeUninit;
use core::ptr;
use nox_shield_core::custody::Seed;
use nox_shield_core::keys::ViewKey;
use nox_shield_core::notes::NotePlaintext;

// Secrets are gone from memory after drop, observed and not merely declared, since a derived
// wipe can survive a refactor that moved the fields. Each value sits in storage the test
// owns, is dropped in place, and is read back, so no freed memory is read. The pattern is 0xA5
// because all zeros would pass against a wipe that never happened.
const PATTERN: u8 = 0xA5;

#[test]
fn the_seed_is_zero_after_it_drops() {
    let mut slot = MaybeUninit::new(Seed::new([PATTERN; 64]));
    let seed = slot.as_mut_ptr();

    // SAFETY: the storage is this test's, and it holds an initialised Seed.
    unsafe { ptr::drop_in_place(seed) };

    // SAFETY: the storage is still this test's, and a Seed is 64 plain bytes.
    let seen = unsafe { ptr::read(seed.cast::<[u8; 64]>()) };
    assert_eq!(seen, [0u8; 64], "the seed was still in memory after it dropped");
}

#[test]
fn a_note_plaintext_is_zero_after_it_drops() {
    let plaintext = NotePlaintext {
        value: u64::from(PATTERN),
        asset_id: u64::from(PATTERN),
        blinding: [u64::from(PATTERN); 4],
        spend_pk: [u64::from(PATTERN); 4],
    };
    let mut slot = MaybeUninit::new(plaintext);
    let note = slot.as_mut_ptr();

    // SAFETY: the storage is this test's, and it holds an initialised note.
    unsafe { ptr::drop_in_place(note) };

    // SAFETY: still this test's storage, and a note plaintext is ten 64 bit words.
    let seen = unsafe { ptr::read(note.cast::<[u64; 10]>()) };
    assert_eq!(seen, [0u64; 10], "a note's value or blinding survived its drop");
}

#[test]
fn the_viewing_secret_is_zero_after_it_drops() {
    let mut slot = MaybeUninit::new(ViewKey::from_seed(&[PATTERN; 64]));
    let view = slot.as_mut_ptr();

    // SAFETY: the storage is this test's, and it holds an initialised key.
    unsafe { ptr::drop_in_place(view) };

    // SAFETY: still this test's storage, and a viewing key is one 32 byte secret.
    let seen = unsafe { ptr::read(view.cast::<[u8; 32]>()) };
    assert_eq!(seen, [0u8; 32], "the viewing secret survived its drop");
}

// The control: a seed shaped type with no wipe, put through the same steps, must still hold
// the pattern. If it reads zeros, the three tests above prove nothing.
struct Unwiped(#[allow(dead_code)] [u8; 64]);

#[test]
fn the_technique_would_notice_a_missing_wipe() {
    let mut slot = MaybeUninit::new(Unwiped([PATTERN; 64]));
    let plain = slot.as_mut_ptr();

    // SAFETY: the storage is this test's, and it holds an initialised value.
    unsafe { ptr::drop_in_place(plain) };

    // SAFETY: still this test's storage, holding 64 plain bytes.
    let seen = unsafe { ptr::read(plain.cast::<[u8; 64]>()) };
    assert_eq!(
        seen, [PATTERN; 64],
        "reading storage after a drop returns zeros by itself, so the wipe tests prove nothing"
    );
}
