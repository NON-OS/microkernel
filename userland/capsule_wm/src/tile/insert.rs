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
//! A window added to a tree: beside the window `at` (the focused one), in
//! the split `at` sits in when it splits the same way, or in a new split of
//! the two; at the tree's end when there is no `at`.

use alloc::vec;

use super::node::Node;
use super::types::{Split, Win};

pub fn insert(tree: Option<Node>, at: Option<Win>, new: Win, split: Split) -> Node {
    let Some(mut t) = tree else { return Node::Leaf(new) };
    if at.is_some_and(|a| beside(&mut t, a, new, split)) {
        return t;
    }
    match t {
        Node::Split(s, mut kids) if s == split => {
            kids.push(Node::Leaf(new));
            Node::Split(s, kids)
        }
        other => Node::Split(split, vec![other, Node::Leaf(new)]),
    }
}

fn beside(node: &mut Node, at: Win, new: Win, split: Split) -> bool {
    let pair = |a| Node::Split(split, vec![Node::Leaf(a), Node::Leaf(new)]);
    match node {
        Node::Leaf(w) if *w == at => {
            *node = pair(at);
            true
        }
        Node::Leaf(_) => false,
        Node::Split(s, kids) => {
            let Some(i) = kids.iter().position(|k| *k == Node::Leaf(at)) else {
                return kids.iter_mut().any(|k| beside(k, at, new, split));
            };
            if *s == split {
                kids.insert(i + 1, Node::Leaf(new));
            } else {
                kids[i] = pair(at);
            }
            true
        }
    }
}
