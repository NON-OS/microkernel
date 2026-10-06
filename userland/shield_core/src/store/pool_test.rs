// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing)]

use super::on_pool;
use crate::net::pool::ACTIVE;
use crate::notes::{NotePlaintext, NoteRecord, NoteStatus};
use crate::store::row::Row;
use crate::store::state::StoreState;
use std::collections::BTreeMap;

fn note(value: u64, leaf: u64, cm: [u64; 4]) -> NoteRecord {
    let asset_id = ACTIVE.assets[1].id;
    let plain = NotePlaintext { value, asset_id, blinding: [leaf, 1, 2, 3], spend_pk: [0; 4] };
    NoteRecord { plain, leaf_index: leaf, cm, status: NoteStatus::Unspent, found_at: leaf }
}

/// The pool's word for a commitment: limb 3 first, big endian, as `RootCommitted` carries it.
fn wire(cm: [u64; 4]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (chunk, limb) in out.chunks_exact_mut(8).zip(cm.iter().rev()) {
        chunk.copy_from_slice(&limb.to_be_bytes());
    }
    out
}

/// A note from the launch pool sits at a leaf the v2 pool holds for another note. It keeps its
/// row, but the balance and the spendable list leave it out.
#[test]
fn a_note_from_another_pool_is_neither_counted_nor_spent() {
    let here = note(5_000, 0, [11, 12, 13, 14]);
    let there = note(7_000, 1, [21, 22, 23, 24]);
    let committed = BTreeMap::from([(0, wire(here.cm)), (1, wire([91, 92, 93, 94]))]);
    assert!(on_pool(&committed, &here));
    assert!(!on_pool(&committed, &there));

    let mut state = StoreState::empty();
    state.apply(Row::Found(here));
    state.apply(Row::Found(there));
    assert_eq!(state.balance(&ACTIVE.assets[1]).spendable, 12_000, "before a sync marks them");
    state.mark_pool(&committed);
    assert_eq!(state.balance(&ACTIVE.assets[1]).spendable, 5_000);
    assert_eq!(state.spendable().len(), 1);
    assert_eq!(state.held().len(), 2, "the launch note keeps its row");
}
