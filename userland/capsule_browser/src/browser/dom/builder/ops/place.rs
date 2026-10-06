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

use super::link::Loc;
use super::state::Builder;

impl Builder {
    /// The appropriate place for inserting a node (13.2.6.1), under `target`
    /// or the current node. With foster parenting on, content aimed at a
    /// table part lands before the table instead. A template's contents are
    /// its ordinary children here.
    pub(in super::super) fn place(&self, target: Option<usize>) -> Loc {
        let target = target.unwrap_or_else(|| self.cur());
        if !self.foster || !self.is_any(target, &["table", "tbody", "tfoot", "thead", "tr"]) {
            return Loc::end(target);
        }
        let last =
            self.open.iter().rev().copied().find(|&id| self.is_any(id, &["template", "table"]));
        let Some(table) = last else {
            /* Fragment case: no table is open, so the root takes it. */
            return Loc::end(self.open[0]);
        };
        if self.is(table, "template") {
            return Loc::end(table);
        }
        /*
         * Unlinking a node zeroes its parent, so a table still in a tree
         * (the document or a detached one) has a nonzero parent.
         */
        let parent = self.dom.nodes[table].parent;
        if parent != 0 {
            return Loc { parent, before: Some(table) };
        }
        /* The table left the tree: the element under it on the stack. */
        let at = self.open.iter().rposition(|&x| x == table).unwrap_or(1);
        Loc::end(self.open[at.saturating_sub(1)])
    }
}
