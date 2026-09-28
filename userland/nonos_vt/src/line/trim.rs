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

//! What of a line is text, and dropping the rest when it leaves the screen.

use super::Line;

impl Line {
    /// Cells up to the last one a reader would miss.
    pub fn content_len(&self) -> usize {
        self.cells.iter().rposition(|c| !c.is_plain_blank()).map_or(0, |i| i + 1)
    }

    /// Drop trailing blanks, for a line leaving the screen. A wrapped line
    /// keeps them: its spaces run on into the next line and are text.
    pub fn trim(&mut self) {
        if !self.wrapped {
            let n = self.content_len();
            self.cells.truncate(n);
        }
    }
}
