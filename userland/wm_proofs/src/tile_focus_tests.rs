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
//! Moving focus by direction: the nearest window wholly beyond the focused
//! one's edge, the most in line with it first; none past the screen's edge.

use crate::rect::Rect;
use crate::tile::neighbour::neighbour;
use crate::tile::node::Node::{Leaf, Split as S};
use crate::tile::place::layout;
use crate::tile::types::Dir::{Down, Left, Right, Up};
use crate::tile::types::{Gaps, Split};

const A: (u32, u32) = (1, 1);
const B: (u32, u32) = (2, 1);
const C: (u32, u32) = (3, 1);
const D: (u32, u32) = (4, 1);

/// A above B on the left, C above D on the right.
fn grid() -> Vec<((u32, u32), Rect)> {
    let left = S(Split::Down, vec![Leaf(A), Leaf(B)]);
    let right = S(Split::Down, vec![Leaf(C), Leaf(D)]);
    let work = Rect { x: 0, y: 0, width: 1000, height: 900 };
    layout(&S(Split::Across, vec![left, right]), work, Gaps { inner: 8, outer: 4 })
}

#[test]
fn focus_moves_to_the_nearest_window_that_way() {
    let g = grid();
    assert_eq!(neighbour(&g, A, Right), Some(C), "C is level with A");
    assert_eq!(neighbour(&g, B, Right), Some(D), "D is level with B");
    assert_eq!(neighbour(&g, A, Down), Some(B));
    assert_eq!(neighbour(&g, B, Up), Some(A));
    assert_eq!(neighbour(&g, D, Left), Some(B));
    assert_eq!(neighbour(&g, C, Left), Some(A));
    assert_eq!(neighbour(&g, D, Up), Some(C));
}

#[test]
fn nothing_past_the_edge_or_for_a_stranger() {
    let g = grid();
    assert_eq!(neighbour(&g, A, Left), None);
    assert_eq!(neighbour(&g, A, Up), None);
    assert_eq!(neighbour(&g, D, Down), None);
    assert_eq!(neighbour(&g, C, Right), None);
    assert_eq!(neighbour(&g, (9, 9), Right), None);
}
