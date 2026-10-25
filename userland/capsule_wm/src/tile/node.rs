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
//! A workspace's tiled windows: a window, or a split of two or more.

use alloc::vec::Vec;

use super::types::{Split, Win};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Leaf(Win),
    Split(Split, Vec<Node>),
}

impl Node {
    /// Every window in the tree, in order.
    pub fn leaves(&self, out: &mut Vec<Win>) {
        match self {
            Node::Leaf(w) => out.push(*w),
            Node::Split(_, kids) => kids.iter().for_each(|k| k.leaves(out)),
        }
    }

    pub fn holds(&self, w: Win) -> bool {
        match self {
            Node::Leaf(x) => *x == w,
            Node::Split(_, kids) => kids.iter().any(|k| k.holds(w)),
        }
    }
}
