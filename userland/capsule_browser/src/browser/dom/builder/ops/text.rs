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

use alloc::borrow::Cow;

use super::super::super::node::NodeKind;
use super::link::Loc;
use super::state::Builder;

impl Builder {
    /// Insert characters (13.2.6.1) at the appropriate place, appending to
    /// the text node already there when there is one, so a run split by
    /// references or ignored tags stays one node. A comment in between
    /// keeps them apart, as the comment node the tree does not hold would.
    pub(in super::super) fn insert_text(&mut self, text: Cow<'_, str>) {
        if text.is_empty() {
            return;
        }
        let loc = self.place(None);
        if loc.parent == 0 {
            return;
        }
        if let Some((i, p)) = self.slot_before(loc) {
            if self.dom.nodes[p].kind == NodeKind::Text && self.comment_at != (loc.parent, i + 1) {
                self.dom.nodes[p].text.push_str(&text);
                return;
            }
        }
        if let Some(t) = self.create_text(text.into_owned()) {
            self.link(loc, t);
        }
    }

    /// Note where a comment would be inserted now, for `insert_text`.
    pub(in super::super) fn mark_comment(&mut self) {
        let loc = self.place(None);
        let at = match self.slot_before(loc) {
            Some((i, _)) => i + 1,
            None => 0,
        };
        self.comment_at = (loc.parent, at);
    }

    /// The child just before `loc`, with its index.
    fn slot_before(&self, loc: Loc) -> Option<(usize, usize)> {
        let kids = &self.dom.nodes[loc.parent].children;
        let i = match loc.before {
            Some(b) => kids.iter().rposition(|&c| c == b)?.checked_sub(1)?,
            None => kids.len().checked_sub(1)?,
        };
        Some((i, kids[i]))
    }
}
