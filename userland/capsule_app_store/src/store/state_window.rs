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
//! The window onto the list: which rows are showing, and which row the cursor
//! is on when a pointer puts it there.

use super::state::State;

impl State {
    /// Keep the window inside the list. Called from the frame, which is
    /// the only place the row count is known.
    pub fn clamp_scroll(&mut self) {
        let n = self.visible().len();
        let most = n.saturating_sub(self.rows);
        if self.scroll > most {
            self.scroll = most;
        }
    }

    /// Move the window without moving the cursor, which is what a wheel does:
    /// the selection stays where the user put it and the list travels under
    /// it.
    pub fn scroll_by(&mut self, delta: isize) -> bool {
        let n = self.visible().len();
        let most = n.saturating_sub(self.rows) as isize;
        let want = (self.scroll as isize + delta).clamp(0, most.max(0)) as usize;
        if want == self.scroll {
            return false;
        }
        self.scroll = want;
        true
    }

    /// Put the cursor on a visible slot, as a click does.
    pub fn select_slot(&mut self, slot: usize) -> bool {
        let want = self.scroll + slot;
        if want >= self.visible().len() || want == self.cursor {
            return false;
        }
        self.cursor = want;
        self.select();
        true
    }
}
