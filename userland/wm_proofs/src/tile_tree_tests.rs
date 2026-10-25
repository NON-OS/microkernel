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
//! The split tree: a window opens beside the focused one, in its split when
//! that splits the same way and in a new split otherwise; closing one
//! collapses a split left with a single child.

use crate::tile::insert::insert;
use crate::tile::node::Node::{self, Leaf, Split as S};
use crate::tile::remove::remove;
use crate::tile::types::Split::{Across, Down};

const A: (u32, u32) = (10, 1);
const B: (u32, u32) = (11, 1);
const C: (u32, u32) = (12, 1);

fn leaves(t: &Node) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    t.leaves(&mut v);
    v
}

#[test]
fn a_window_opens_beside_the_focused_one() {
    let t = insert(None, None, A, Across);
    assert_eq!(t, Leaf(A));
    let t = insert(Some(t), Some(A), B, Across);
    assert_eq!(t, S(Across, vec![Leaf(A), Leaf(B)]));
    let t = insert(Some(t), Some(A), C, Across);
    assert_eq!(leaves(&t), [A, C, B], "after the focused one, not at the end");
}

#[test]
fn another_direction_splits_the_focused_window() {
    let t = insert(Some(S(Across, vec![Leaf(A), Leaf(B)])), Some(B), C, Down);
    assert_eq!(t, S(Across, vec![Leaf(A), S(Down, vec![Leaf(B), Leaf(C)])]));
    let t = insert(Some(Leaf(A)), Some(A), B, Down);
    assert_eq!(t, S(Down, vec![Leaf(A), Leaf(B)]));
}

#[test]
fn with_no_focused_window_it_goes_at_the_end() {
    let t = insert(Some(S(Across, vec![Leaf(A), Leaf(B)])), None, C, Across);
    assert_eq!(leaves(&t), [A, B, C]);
    let t = insert(Some(S(Across, vec![Leaf(A), Leaf(B)])), Some((99, 9)), C, Down);
    assert_eq!(t, S(Down, vec![S(Across, vec![Leaf(A), Leaf(B)]), Leaf(C)]));
}

#[test]
fn closing_collapses_what_is_left() {
    let t = S(Across, vec![Leaf(A), S(Down, vec![Leaf(B), Leaf(C)])]);
    assert_eq!(remove(t.clone(), C), Some(S(Across, vec![Leaf(A), Leaf(B)])));
    assert_eq!(remove(t.clone(), A), Some(S(Down, vec![Leaf(B), Leaf(C)])));
    assert_eq!(remove(Leaf(A), A), None);
    assert_eq!(remove(t.clone(), (99, 9)), Some(t), "a stranger changes nothing");
}
