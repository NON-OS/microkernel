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
//! Batch, continued.

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
fn consecutive_batches_go_to_different_authorities() {
    let all = entries(5057);
    let first = batch_at(&all, 0, 0).expect("batch zero");
    let second = batch_at(&all, 0, 1).expect("batch one");
    assert_ne!(first.0, second.0, "two batches in a row do not hit one authority");
    assert_eq!(first.1, 9230, "the DirPort the fork publishes");
}
#[test]
fn nothing_to_ask_for_is_no_requests() {
    let none: Vec<Entry> = Vec::new();
    assert_eq!(batch_count(&none), 0);
    assert!(batch_at(&none, 0, 0).is_none());
}
