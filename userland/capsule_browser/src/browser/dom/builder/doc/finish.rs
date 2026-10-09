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

use core::mem;

use super::super::super::tree::Dom;
use super::super::ops::state::Builder;
use super::compact::compact;

impl Builder {
    /// The finished tree. A fragment's top-level nodes become the root's
    /// children, so a caller copies node 0's children as it would a
    /// document's. Nodes the parse left outside the tree are dropped and the
    /// rest renumbered in document order.
    pub fn finish(mut self) -> Dom {
        if self.context.is_some() {
            if let Some(&root) = self.dom.nodes[0].children.first() {
                let kids = mem::take(&mut self.dom.nodes[root].children);
                for &k in &kids {
                    self.dom.nodes[k].parent = 0;
                }
                self.dom.nodes[0].children = kids;
            }
        }
        compact(&mut self.dom);
        self.dom
    }
}
