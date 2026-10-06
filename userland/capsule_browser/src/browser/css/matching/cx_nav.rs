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

use super::cx::Cx;

/* Moving through the tree as the matcher does: up to the parent element,
 * back to the previous element sibling. */
impl Cx<'_> {
    /* The parent when it is an element; the document root ends the chain. */
    pub fn parent_el(&self, id: usize) -> Option<usize> {
        let p = self.dom.nodes.get(id)?.parent;
        (p != id && p != 0 && self.element(p).is_some()).then_some(p)
    }

    pub fn prev_el(&self, id: usize) -> Option<usize> {
        if let Some(t) = self.sib.tab() {
            return t.prev(id);
        }
        let kids = &self.dom.nodes.get(self.dom.nodes.get(id)?.parent)?.children;
        let (last, at) = self.walked.get();
        let at = if last == id && kids.get(at) == Some(&id) {
            at
        } else {
            kids.iter().position(|&k| k == id)?
        };
        let i = kids[..at].iter().rposition(|&k| self.element(k).is_some())?;
        self.walked.set((kids[i], i));
        Some(kids[i])
    }
}
