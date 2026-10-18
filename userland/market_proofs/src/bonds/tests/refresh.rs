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

//! The refresh: READs 30, 31, 33 in turn, one in flight, a failed or late
//! read dropping its bond, and nothing kept from words that do not read.

use alloc::vec::Vec;

use super::words::{bond_of, one, CHAIN, ID, MS, NOW};
use crate::bonds::{Ask, BondCache, STEP_MS};

fn ask(view: u16, args: Vec<[u8; 32]>) -> Option<Ask> {
    Some(Ask { chain: CHAIN, view, args })
}

#[test]
fn a_missing_bond_is_queued_then_read_by_30_31_and_33() {
    let mut c = BondCache::default();
    assert_eq!(c.reading((CHAIN, ID), NOW), None, "a miss answers nothing");
    assert_eq!(c.next(MS), ask(30, Vec::from([ID])));
    assert_eq!(c.next(MS), None, "one read in flight");
    assert_eq!(
        c.on_answer(Some(bond_of([0x5a; 20], [0x11; 32], 5_000)), MS),
        ask(31, Vec::from([ID]))
    );
    assert_eq!(c.on_answer(Some(one(1)), MS), ask(33, Vec::new()));
    assert_eq!(c.on_answer(Some(one(1_000)), MS + 2_000), None);
    let (r, min) = c.reading((CHAIN, ID), NOW + 2).unwrap();
    assert_eq!(
        (r.payee, r.amount, r.active, r.read_at, min),
        ([0x5a; 20], 5_000, true, NOW + 2, 1_000)
    );
    assert!(!c.waiting());
}

#[test]
fn a_failed_or_late_read_drops_its_bond_and_the_next_goes() {
    let mut c = BondCache::default();
    let other = [0xb1; 32];
    c.reading((CHAIN, ID), NOW);
    c.reading((CHAIN, other), NOW);
    c.next(MS);
    assert_eq!(c.on_answer(None, MS), ask(30, Vec::from([other])));
    assert_eq!(c.tick(MS + STEP_MS - 1), None);
    assert!(c.waiting());
    assert_eq!(c.tick(MS + STEP_MS), None);
    assert!(!c.waiting(), "dropped after 150 s");
    assert_eq!(c.on_answer(Some(one(1)), MS + STEP_MS), None, "a late answer has no step");
}
