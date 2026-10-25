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
//! A workspace opens a window beside the one that has focus, wherever that
//! is in the tree; and a split with nothing left in it is never laid out.

use crate::tile::node::Node::{Leaf, Split as S};
use crate::tile::remove::remove;
use crate::tile::types::Split::Across;
use crate::tile::workspace::Workspace;

const A: (u32, u32) = (1, 1);
const B: (u32, u32) = (2, 1);
const C: (u32, u32) = (3, 1);

#[test]
fn a_window_opens_beside_the_focused_one_not_the_last() {
    let mut w = Workspace::new();
    w.add(A);
    w.add(B);
    w.focus = Some(A);
    w.add(C);
    assert_eq!(w.windows(), [A, C, B]);
    assert_eq!(w.focus, Some(C));
}

#[test]
fn a_floating_focus_opens_at_the_end() {
    let mut w = Workspace::new();
    w.add(A);
    w.add(B);
    assert!(w.toggle_float(A));
    w.add(C);
    assert_eq!(w.windows(), [B, C, A], "tiled B then C, A floating");
}

#[test]
fn an_emptied_split_is_gone() {
    assert_eq!(remove(S(Across, vec![Leaf(A)]), A), None);
    assert_eq!(remove(S(Across, vec![S(Across, vec![Leaf(A)]), Leaf(B)]), A), Some(Leaf(B)));
}
