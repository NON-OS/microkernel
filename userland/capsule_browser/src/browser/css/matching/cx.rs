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

use core::cell::Cell;

use crate::browser::dom::node::{Node, NodeKind};
use crate::browser::dom::Dom;

use super::sibling::Siblings;

/* Steps one match may spend before it answers no match: one per compound
 * test or node visited, and one per 16 bytes read (cost.rs). A :has()
 * scan costs about two per element it visits, so this admits one over a
 * whole document of MAX_NODES (60,000) elements, and no more. */
pub(super) const CALL_STEPS: u32 = 131_072;

/* One match in progress: the tree, its sibling table, the :scope element
 * (0 for the document element), the :has() anchor, and what it has spent:
 * compound tests and nodes visited, and bytes read (see cost.rs). */
pub(super) struct Cx<'a> {
    pub dom: &'a Dom,
    pub sib: &'a Siblings,
    pub scope: usize,
    pub anchor: Cell<usize>,
    pub(super) steps: Cell<u32>,
    pub(super) read: Cell<usize>,
    /* Where the last walk back through siblings stopped, as (node, index in
     * its parent's children), so a ~ scan without a table stays linear. */
    pub walked: Cell<(usize, usize)>,
}

impl<'a> Cx<'a> {
    pub fn new(dom: &'a Dom, sib: &'a Siblings, scope: usize) -> Self {
        let none = Cell::new(usize::MAX);
        let (steps, read, walked) = (Cell::new(0), Cell::new(0), Cell::new((0, 0)));
        Cx { dom, sib, scope, anchor: none, steps, read, walked }
    }

    pub fn element(&self, id: usize) -> Option<&'a Node> {
        self.dom.nodes.get(id).filter(|n| n.kind == NodeKind::Element)
    }
}
