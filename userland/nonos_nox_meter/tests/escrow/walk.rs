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

//! The walk in slices (v1.4.34): a small budget a step finds the same
//! answer the one-shot request does, over many steps, each one bounded.

use nonos_equix::{Solver, INDEX_SPACE};
use nonos_nox_meter::escrow::{escrow_request, Walk, Walked};

use super::request::want;
use super::x::D;

const SLICE: u32 = 4096;

#[test]
fn a_sliced_walk_finds_what_the_one_shot_finds() {
    let mut solver = Solver::new().unwrap();
    let mut n = 0u8;
    let mut draw = || {
        n += 1;
        Some([n; 32])
    };
    let whole = escrow_request(&D, &mut draw, &mut solver, &want()).unwrap();

    let mut walk = Walk::new(&D, [1; 32], [2; 16], want());
    let mut steps = 0u32;
    let sliced = loop {
        steps += 1;
        match walk.step(&mut solver, SLICE) {
            Walked::Going => {}
            Walked::Found(a) => break a,
            Walked::GaveUp => panic!("gave up"),
        }
    };
    assert!(steps > INDEX_SPACE / SLICE, "each slice hashed at most {SLICE}");
    assert_eq!(sliced.buyer_sk, whole.buyer_sk);
    assert_eq!(sliced.request.encode(), whole.request.encode());
}
