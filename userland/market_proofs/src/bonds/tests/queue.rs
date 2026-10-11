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

//! What the refresh refuses: words that do not read keep nothing, and the
//! queue holds each bond once, QUEUE_MAX at most.

use super::words::{bond_of, one, CHAIN, ID, MS, NOW};
use crate::bonds::{BondCache, QUEUE_MAX};

#[test]
fn words_that_do_not_read_keep_nothing() {
    for (active, min) in [(one(2), one(1)), (one(1), [[1u8; 32]].to_vec())] {
        let mut c = BondCache::default();
        c.reading((CHAIN, ID), NOW);
        c.next(MS);
        c.on_answer(Some(bond_of([0x5a; 20], [0x11; 32], 5_000)), MS);
        c.on_answer(Some(active), MS);
        c.on_answer(Some(min), MS);
        assert_eq!(c.reading((CHAIN, ID), NOW), None);
    }
}

#[test]
fn the_queue_holds_each_bond_once_and_at_most_queue_max() {
    let mut c = BondCache::default();
    (0..QUEUE_MAX as u8 + 5).for_each(|i| _ = c.reading((CHAIN, [i; 32]), NOW));
    c.reading((CHAIN, [0; 32]), NOW);
    let mut asks = usize::from(c.next(MS).is_some());
    while c.on_answer(None, MS).is_some() {
        asks += 1;
    }
    assert_eq!(asks, QUEUE_MAX);
}
