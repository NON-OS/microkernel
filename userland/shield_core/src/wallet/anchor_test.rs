/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::{anchor_where, limbs, Anchor, AnchorRefusal};
use crate::discovery::Log;
use crate::notes::wire_digest;
use crate::prover::launch::tree::pool_root;
use std::collections::BTreeMap;

const EVENT: [u8; 32] = [0xb7; 32];

/// Any committed root, as on a pool with no association registry.
fn anchor(pool: &BTreeMap<u64, [u8; 32]>, logs: &[Log]) -> Result<Anchor, AnchorRefusal> {
    anchor_where(pool, logs, &|_| true)
}

fn leaves(n: u64) -> BTreeMap<u64, [u8; 32]> {
    (0..n).map(|i| (i, wire_digest(&[i + 1, i + 2, i + 3, i + 4]))).collect()
}

fn committed(pool: &BTreeMap<u64, [u8; 32]>, count: u64) -> ([[u8; 32]; 2], [u8; 32]) {
    let under: Vec<[u64; 4]> = (0..count).map(|i| limbs(&pool[&i])).collect();
    let mut data = [0u8; 32];
    data[24..].copy_from_slice(&count.to_be_bytes());
    ([EVENT, wire_digest(&pool_root(&under))], data)
}

#[test]
fn the_newest_root_is_the_anchor_and_covers_its_leaves() {
    let pool = leaves(5);
    let (old_t, old_d) = committed(&pool, 2);
    let (new_t, new_d) = committed(&pool, 4);
    let logs = [Log { topics: &old_t, data: &old_d }, Log { topics: &new_t, data: &new_d }];
    let a = anchor(&pool, &logs).unwrap();
    assert_eq!(a.leaves.len(), 4);
    assert!(a.covers(3) && !a.covers(4), "leaf 4 waits for the next commitRoot");
}

#[test]
fn a_short_or_wrong_history_is_refused() {
    let pool = leaves(4);
    let (t, d) = committed(&pool, 4);
    let mut holed = pool.clone();
    holed.remove(&2);
    assert_eq!(
        anchor(&holed, &[Log { topics: &t, data: &d }]).err(),
        Some(AnchorRefusal::HistoryShort)
    );
    let mut swapped = pool.clone();
    swapped.insert(1, pool[&2]);
    assert_eq!(
        anchor(&swapped, &[Log { topics: &t, data: &d }]).err(),
        Some(AnchorRefusal::TreeMismatch)
    );
    assert_eq!(anchor(&pool, &[]).err(), Some(AnchorRefusal::NoRoot));
}

#[test]
fn a_newer_root_the_registry_lacks_is_passed_over() {
    let pool = leaves(5);
    let (old_t, old_d) = committed(&pool, 2);
    let (new_t, new_d) = committed(&pool, 4);
    let logs = [Log { topics: &old_t, data: &old_d }, Log { topics: &new_t, data: &new_d }];
    let registered = old_t[1];
    let a = anchor_where(&pool, &logs, &|root| *root == registered).unwrap();
    assert_eq!(a.leaves.len(), 2, "the older, registered root");
    assert_eq!(anchor_where(&pool, &logs, &|_| false).err(), Some(AnchorRefusal::NoRoot));
}
