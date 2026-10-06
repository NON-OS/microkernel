/*
 * A test asserts by panicking, and a measurement prints what it read, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use nox_shield_core::entropy::ProofSeed;
use nox_shield_core::error::ProveError;

fn bytes(tag: u8) -> Vec<u8> {
    (0..96u16).map(|k| (k as u8).wrapping_mul(7).wrapping_add(tag)).collect()
}

#[test]
fn a_seed_is_refused_the_second_time_it_is_offered() {
    let seed = ProofSeed::from_bytes(&bytes(1)).expect("enough entropy");
    seed.consume().expect("the first use stands");
    let again = ProofSeed::from_bytes(&bytes(1)).expect("enough entropy");
    assert_eq!(again.consume().err(), Some(ProveError::SeedReused));
}

#[test]
fn a_different_seed_is_not_blocked_by_another() {
    let first = ProofSeed::from_bytes(&bytes(2)).expect("enough entropy");
    first.consume().expect("the first use stands");
    let second = ProofSeed::from_bytes(&bytes(3)).expect("enough entropy");
    assert!(second.consume().is_ok(), "an unrelated seed must not be refused");
}

#[test]
fn a_short_entropy_buffer_is_refused() {
    assert_eq!(
        ProofSeed::from_bytes(&[0u8; 8]).err(),
        Some(ProveError::SeedEntropy),
        "a seed must not be padded out of too few bytes"
    );
}

#[test]
fn a_drawn_seed_is_accepted_once() {
    let drawn = ProofSeed::draw().expect("the platform random source");
    assert!(drawn.consume().is_ok());
}
