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

use super::super::super::node::Ns;
use super::super::ops::link::Loc;
use super::super::ops::mode::Entry;
use super::super::ops::state::Builder;

impl Builder {
    /// The adoption agency's last steps: the furthest block's children move
    /// under a clone of the formatting element, which takes the formatting
    /// element's place in the list at `bookmark` and on the stack below the
    /// furthest block. False when the node cap stopped it.
    pub(super) fn adopt_finish(&mut self, fe: usize, fb: usize, mut bookmark: usize) -> bool {
        let Some(new) = self.create(self.token_of(fe), Ns::Html) else {
            return false;
        };
        let kids = mem::take(&mut self.dom.nodes[fb].children);
        for &k in &kids {
            self.dom.nodes[k].parent = new;
        }
        self.dom.nodes[new].children = kids;
        self.link(Loc::end(fb), new);
        if let Some(j) = self.fmt_index(fe) {
            bookmark -= usize::from(j < bookmark);
        }
        self.remove_fmt(fe);
        self.fmt.insert(bookmark.min(self.fmt.len()), Entry::Elem(new));
        self.fmt_mark(new);
        self.remove_open(fe);
        self.insert_open_after(fb, new);
        true
    }
}
