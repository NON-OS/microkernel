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

/* Compound tests one match may spend before it answers no match. A :has()
 * scan costs about two per element it visits, so this admits one over a
 * whole document of MAX_NODES (60,000) elements, and no more. */
pub(super) const CALL_STEPS: u32 = 131_072;

/* One match in progress: the tree, its sibling table, the :scope element
 * (0 for the document element), the :has() anchor, and the steps spent. */
pub(super) struct Cx<'a> {
    pub dom: &'a Dom,
    pub sib: &'a Siblings,
    pub scope: usize,
    pub anchor: Cell<usize>,
    steps: Cell<u32>,
    /* Where the last walk back through siblings stopped, as (node, index in
     * its parent's children), so a ~ scan without a table stays linear. */
    pub walked: Cell<(usize, usize)>,
}

impl<'a> Cx<'a> {
    pub fn new(dom: &'a Dom, sib: &'a Siblings, scope: usize) -> Self {
        let none = Cell::new(usize::MAX);
        Cx { dom, sib, scope, anchor: none, steps: Cell::new(0), walked: Cell::new((0, 0)) }
    }

    /* Spend one step; false once the budget is gone. */
    pub fn tick(&self) -> bool {
        let s = self.steps.get().saturating_add(1);
        self.steps.set(s);
        s <= CALL_STEPS
    }

    pub fn spent(&self) -> u32 {
        self.steps.get()
    }

    pub fn exhausted(&self) -> bool {
        self.steps.get() > CALL_STEPS
    }

    pub fn element(&self, id: usize) -> Option<&'a Node> {
        self.dom.nodes.get(id).filter(|n| n.kind == NodeKind::Element)
    }
}
