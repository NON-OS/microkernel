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

//! Which part of a field's text is shown.

use super::view::View;
use super::Field;
use crate::widgets::text;

const ELLIPSIS: &str = "\u{2026}";

impl Field<'_> {
    /// Whether the field takes typing: focused and not disabled.
    pub(super) fn typing(&self) -> bool {
        self.state.focused && !self.state.disabled
    }

    /// `c` when it is a char boundary of the text, otherwise the text's end.
    pub(super) fn boundary(&self, c: usize) -> usize {
        Some(c).filter(|&c| self.text.is_char_boundary(c)).unwrap_or(self.text.len())
    }

    /// The run shown in a box `w` wide. Empty text shows the placeholder,
    /// truncated. Text that fits shows as is. Text too long shows its tail
    /// behind an ellipsis while typing, so the newest characters stay in
    /// view, unless the caret lies in the part the tail hides, and otherwise
    /// is truncated with an ellipsis.
    pub(super) fn view(&self, w: u32, px: f32) -> View {
        if self.text.is_empty() {
            let shown = text::truncate(self.placeholder, w, px);
            return View { shown, lead: 0, from: 0, keep: 0 };
        }
        if !self.typing() || text::width(self.text, px) <= w {
            let shown = text::truncate(self.text, w, px);
            let cut = shown.char_indices().next_back().map_or(0, |(i, _)| i);
            let keep = if shown == self.text { shown.len() } else { cut };
            return View { shown, lead: 0, from: 0, keep };
        }
        let shown = text::tail(self.text, w, px);
        let lead = shown.chars().next().map_or(0, char::len_utf8);
        let keep = shown.len().saturating_sub(lead);
        let from = self.text.len().saturating_sub(keep);
        match self.caret.map(|c| self.boundary(c)).filter(|&c| c < from) {
            Some(c) => self.window(c, w, px),
            None => View { from, shown, lead, keep },
        }
    }

    /// A text too long for a box `w` wide, shown from the caret at `c` on:
    /// an ellipsis when text is hidden before `c`, then as much of the text
    /// from `c` as fits before a closing ellipsis, so the caret sits at the
    /// left edge of what is shown.
    fn window(&self, c: usize, w: u32, px: f32) -> View {
        let lead = if c == 0 { 0 } else { ELLIPSIS.len() };
        let run = [&ELLIPSIS[..lead], self.text.get(c..).unwrap_or("")].concat();
        let shown = text::truncate(&run, w, px);
        let cut = shown.char_indices().next_back().map_or(0, |(i, _)| i);
        let kept = if shown == run { shown.len() } else { cut };
        View { keep: kept.saturating_sub(lead), shown, lead, from: c }
    }
}
