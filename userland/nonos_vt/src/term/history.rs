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

//! Scrollback: lines that left the top of the normal screen.

use super::state::Term;
use crate::line::Line;

impl Term {
    pub(super) fn push_history(&mut self, mut line: Line) {
        self.scrolled = self.scrolled.saturating_add(1);
        if self.scrollback_limit == 0 {
            return;
        }
        line.trim();
        if self.scrollback.len() >= self.scrollback_limit {
            self.scrollback.pop_front();
        }
        self.scrollback.push_back(line);
        if self.view > 0 {
            /*
             * Someone reading history keeps reading the same lines while
             * output arrives below them.
             */
            self.view = (self.view + 1).min(self.scrollback.len());
        }
    }

    /// CSI 3 J: forget the scrollback.
    pub(super) fn clear_history(&mut self) {
        self.scrollback.clear();
        self.view = 0;
        self.prompts.clear();
        self.touch_all();
    }
}
