// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Batch.

use crate::batch::{batch_at, batch_count, batch_entries};
use crate::directory::consensus::Entry;
use alloc::vec::Vec;

fn entries(n: usize) -> Vec<Entry> {
    (0..n)
        .map(|i| {
            let mut e = Entry { address: [10, 0, 0, 1], or_port: 9001, ..Entry::default() };
            // Distinct per entry so a batch cannot be confused with its neighbour.
            e.microdesc_digest[0] = (i >> 8) as u8;
            e.microdesc_digest[1] = i as u8;
            e
        })
        .collect()
}

#[test]
fn the_batches_cover_every_entry_exactly_once() {
    let all = entries(5057);
    let count = batch_count(&all);
    let mut seen = 0usize;
    for index in 0..count {
        seen += batch_entries(&all, index).len();
    }
    assert_eq!(seen, all.len(), "5057 entries across {count} batches, none lost or repeated");
    assert_eq!(count, 55, "ninety two per request");
}
#[test]
fn the_count_matches_what_can_be_addressed() {
    let all = entries(5057);
    let count = batch_count(&all);
    assert!(batch_at(&all, 0, count - 1).is_some(), "the last batch is a real request");
    assert!(batch_at(&all, 0, count).is_none(), "and one past it is not");
    assert!(batch_entries(&all, count).is_empty(), "past the end covers nothing");
}
/// A short final batch is normal: 5057 is not a multiple of 92. Rounding the count
#[test]
fn the_final_batch_is_the_remainder() {
    let all = entries(5057);
    let count = batch_count(&all);
    let last = batch_entries(&all, count - 1);
    assert_eq!(last.len(), 5057 % 92, "the remainder, not a full request");
    assert!(!last.is_empty(), "and not an empty request nobody needed to send");
}
