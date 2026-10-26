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
//! A window taken out of a tree. A split left with one child becomes that
//! child, and one left with none goes, so no empty space is ever laid out.

use alloc::vec::Vec;

use super::node::Node;
use super::types::Win;

/// The tree without `w`; None when nothing is left.
pub fn remove(tree: Node, w: Win) -> Option<Node> {
    match tree {
        Node::Leaf(x) => (x != w).then_some(Node::Leaf(x)),
        Node::Split(s, kids) => {
            let mut left: Vec<Node> = kids.into_iter().filter_map(|k| remove(k, w)).collect();
            match left.len() {
                0 => None,
                1 => left.pop(),
                _ => Some(Node::Split(s, left)),
            }
        }
    }
}
