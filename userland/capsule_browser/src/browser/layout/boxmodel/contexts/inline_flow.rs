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

use super::super::inline_items::Lead;

/* White-space state carried across element boundaries through one inline
 * formatting context, so "a <b>b</b>" keeps one space and "a<b>b</b>" none,
 * wherever the source put it. */
pub(in super::super) struct Flow {
    /* A collapsible space waiting for the next item: its width in the text
     * it came from, whether a line may wrap at it, and whether it is
     * underlined. */
    space: Option<(i32, bool, bool)>,
    /* Nothing is on the line yet: a space here collapses away. */
    pub(in super::super) at_start: bool,
    /* The last item was an atom, which a line may break after. */
    after_atom: bool,
    /* A break is allowed before the next item though no space precedes it
     * (after a hyphen, or after preserved spaces). */
    soft: bool,
}

impl Flow {
    pub(in super::super) fn new() -> Self {
        Flow { space: None, at_start: true, after_atom: false, soft: false }
    }

    /* Whitespace in the source: the first of a run is the one kept. */
    pub(in super::super) fn gap(&mut self, w: i32, wrap: bool, ul: bool) {
        if self.space.is_none() && !self.at_start {
            self.space = Some((w, wrap, ul));
        }
    }

    /* Allow a line break before the next item, with no space there. */
    pub(in super::super) fn allow_break(&mut self) {
        self.soft = true;
    }

    /* The next item begins a word: at a line start, after a space or atom. */
    pub(in super::super) fn word_start(&self) -> bool {
        self.at_start || self.space.is_some() || self.after_atom
    }

    /* The lead of the next item. An atom may be broken before and after
     * where its own white-space allows wrapping. */
    pub(in super::super) fn lead(&mut self, atom: bool, wraps: bool) -> Lead {
        let (space, wrap, ul) = self.space.take().unwrap_or((0, false, false));
        let near_atom = (atom || self.after_atom) && wraps;
        let brk = (wrap || near_atom || self.soft) && !self.at_start;
        (self.at_start, self.after_atom, self.soft) = (false, atom, false);
        Lead { space, brk, ul, join: false }
    }

    /* A forced line break: the next line starts clean. */
    pub(in super::super) fn hard_break(&mut self) {
        (self.space, self.at_start, self.after_atom, self.soft) = (None, true, false, false);
    }
}
