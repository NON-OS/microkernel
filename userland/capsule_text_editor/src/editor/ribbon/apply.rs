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

//! Run-style edits over the selection. They change the marks beside the text
//! (`style_marks`), not the model alone, so the next keystroke's rebuild and
//! Export keep them.

use crate::doc::align::Align;
use crate::editor::state::State;
use crate::editor::style_marks::Mark;

impl State {
    pub(in crate::editor) fn restyle_sel(&mut self, f: &dyn Fn(&mut Mark)) -> bool {
        let Some((s, e)) = self.sel_range() else {
            return false;
        };
        self.mark_range(s, e, f)
    }

    pub(in crate::editor) fn align_sel(&mut self, a: Align) {
        let (s, e) = self.sel_range().unwrap_or((self.caret, self.caret));
        self.align_lines(s, e, a);
    }
}
