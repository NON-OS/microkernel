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

use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/* The elements under a node, in tree order. A child is followed only when
 * it names the node listing it as its parent, and the walk stops after as
 * many visits as the tree has nodes, so a script-made loop cannot hold it. */
pub(super) struct Under<'a> {
    dom: &'a Dom,
    stack: Vec<usize>,
    left: usize,
}

impl<'a> Under<'a> {
    pub fn new(dom: &'a Dom, root: usize) -> Self {
        let mut walk = Under { dom, stack: Vec::new(), left: dom.nodes.len() };
        walk.push_children(root);
        walk
    }

    fn push_children(&mut self, p: usize) {
        if let Some(n) = self.dom.nodes.get(p) {
            let dom = self.dom;
            let kids = n.children.iter().rev().copied();
            self.stack.extend(kids.filter(|&k| dom.nodes.get(k).is_some_and(|c| c.parent == p)));
        }
    }
}

impl Iterator for Under<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while let Some(id) = self.stack.pop() {
            if self.left == 0 {
                return None;
            }
            self.left -= 1;
            self.push_children(id);
            if self.dom.nodes[id].kind == NodeKind::Element {
                return Some(id);
            }
        }
        None
    }
}
