/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::arithmetic_side_effects)]

//! A note waits for 20 more leaves and six hours of blocks, and both must hold.

use super::ripe::{ripe, WAIT_BLOCKS, WAIT_LEAVES};
use crate::net::rpc::RawLog;

/// A pool of `n` leaves, leaf i landing in block `100 + i`.
fn pool(n: u64) -> Vec<RawLog> {
    (0..n)
        .map(|i| {
            let mut index = [0u8; 32];
            index[24..32].copy_from_slice(&i.to_be_bytes());
            RawLog { topics: vec![[0; 32], [1; 32], index], data: vec![], block: 100 + i, tx: None }
        })
        .collect()
}

#[test]
fn a_note_waits_for_both_the_leaves_and_the_hours() {
    let late = 100 + WAIT_BLOCKS + 100;
    assert!(!ripe(&pool(WAIT_LEAVES), late)(0), "the pool has not grown past it");
    assert!(ripe(&pool(WAIT_LEAVES + 1), late)(0), "grown and aged");
    assert!(!ripe(&pool(WAIT_LEAVES + 1), 100 + WAIT_BLOCKS - 1)(0), "grown but too recent");
    assert!(!ripe(&pool(WAIT_LEAVES + 1), late)(1), "the second leaf has not seen 20 after it");
}

#[test]
fn the_wait_counts_down_to_zero_when_a_note_is_ripe() {
    use super::ripe::waiting;
    let late = 100 + WAIT_BLOCKS + 100;
    assert_eq!(waiting(&pool(WAIT_LEAVES + 1), late, &[0]), (0, 0), "ripe, nothing to wait for");
    assert_eq!(waiting(&pool(5), late, &[0]).0, WAIT_LEAVES - 4, "four have landed after it");
    assert_eq!(waiting(&pool(WAIT_LEAVES + 1), 100, &[0]).1, WAIT_BLOCKS, "no time has passed");
}
