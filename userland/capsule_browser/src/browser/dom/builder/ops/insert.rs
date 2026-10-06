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

use crate::browser::html::tokenizer::Tag;

use super::super::super::limits::{MAX_DEPTH, TRUNC_DEPTH};
use super::super::super::node::Ns;
use super::state::Builder;

impl Builder {
    /// Insert an element for `tag` at the appropriate place and, when `push`,
    /// open it (13.2.6.1, "insert a foreign element"). Every element the
    /// parser creates goes through here or through `link`.
    ///
    /// The stack holds at most `MAX_DEPTH` open elements: one more closes
    /// the current node first, so the new element becomes its sibling. That
    /// bounds every walk of the stack, and with it the work per token.
    pub(in super::super) fn insert_element(
        &mut self,
        tag: Tag,
        ns: Ns,
        push: bool,
    ) -> Option<usize> {
        if push && self.open.len() >= MAX_DEPTH {
            self.make_room();
        }
        let loc = self.place(None);
        let id = self.create(tag, ns)?;
        self.link(loc, id);
        if push {
            self.push_open(id);
        }
        Some(id)
    }

    /// Insert and open an HTML element.
    pub(in super::super) fn insert_html(&mut self, tag: Tag) -> Option<usize> {
        self.insert_element(tag, Ns::Html, true)
    }

    /// Insert an HTML element that is closed at once: void elements, and the
    /// few the rules pop straight after inserting.
    pub(in super::super) fn insert_void(&mut self, tag: Tag) -> Option<usize> {
        self.insert_element(tag, Ns::Html, false)
    }

    /// Close the current node to keep the stack within its cap, undoing what
    /// opening it did to the list of active formatting elements so that it
    /// is not reopened by the next character.
    fn make_room(&mut self) {
        let Some(id) = self.pop() else {
            return;
        };
        self.dom.truncated |= TRUNC_DEPTH;
        self.remove_fmt(id);
        let markers = ["applet", "caption", "marquee", "object", "td", "template", "th"];
        if self.is_any(id, &markers) {
            self.clear_to_marker();
        }
    }
}
