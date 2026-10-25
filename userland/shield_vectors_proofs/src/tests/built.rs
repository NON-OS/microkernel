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

//! The tree a proof from nothing builds, held by the capsule to the shipped
//! cache: the same bytes pass, one bit changed fails, by the vector's name.

use super::json::{failed, lines, passed};
use super::shipped::shipped;
use crate::cache;

#[test]
fn a_built_tree_is_held_to_it_by_name() {
    let Some(top) = shipped("a_built_tree_is_held_to_it_by_name") else { return };
    let line = "transfer-eth: the tree built here is the shipped cache, byte for byte";
    let (_, text) = lines(|out| {
        cache::built_is_shipped(out, "transfer-eth", top);
        true
    });
    assert_eq!((passed(&text), failed(&text)), (vec![line], vec![]));
    let mut built = top.to_vec();
    built[top.len() / 2] ^= 1;
    let (_, text) = lines(|out| {
        cache::built_is_shipped(out, "transfer-eth", &built);
        true
    });
    assert_eq!((passed(&text), failed(&text)), (vec![], vec![line]));
}
