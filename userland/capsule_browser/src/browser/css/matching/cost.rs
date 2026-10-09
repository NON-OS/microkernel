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

use crate::browser::dom::node::Node;

use super::cx::{Cx, CALL_STEPS};

/* Bytes one step pays for. A compound test costs one step, about as long
 * as reading 16 bytes byte by byte, so a test that reads an attribute, a
 * class list or a text pays for what it reads at that rate. */
const BYTES_PER_STEP: usize = 16;

/* What one item walked costs in bytes: an attribute entry, a class token. */
pub(super) const ITEM: usize = 8;

/* What one parent hop of an ancestor walk costs in bytes: one step. */
pub(super) const HOP: usize = BYTES_PER_STEP;

/* The cost of looking one attribute up among the element's attributes. */
pub(super) fn lookup(node: &Node) -> usize {
    node.attrs.len().saturating_mul(ITEM)
}

impl Cx<'_> {
    /* Spend one step; false once the budget is gone. */
    pub fn tick(&self) -> bool {
        self.steps.set(self.steps.get().saturating_add(1));
        !self.exhausted()
    }

    /* Pay for reading `bytes` before reading them; false once the budget
     * is gone, and the caller then answers no match without reading. */
    pub fn charge(&self, bytes: usize) -> bool {
        self.read.set(self.read.get().saturating_add(bytes));
        !self.exhausted()
    }

    /* Steps spent, at most one past the budget: a read refused for its
     * size was never made, so it is not counted in full. */
    pub fn spent(&self) -> u32 {
        let read = u32::try_from(self.read.get() / BYTES_PER_STEP).unwrap_or(u32::MAX);
        self.steps.get().saturating_add(read).min(CALL_STEPS + 1)
    }

    pub fn exhausted(&self) -> bool {
        self.spent() > CALL_STEPS
    }
}
