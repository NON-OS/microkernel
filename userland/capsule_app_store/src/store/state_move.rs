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

//! The cursor and the window onto the list.

use super::state::{State, Tab};

impl State {
    pub fn move_by(&mut self, delta: isize) -> bool {
        let n = self.visible().len();
        if n == 0 {
            return false;
        }
        let want = (self.cursor as isize + delta).clamp(0, n as isize - 1) as usize;
        if want == self.cursor {
            return false;
        }
        self.cursor = want;
        self.follow();
        self.select();
        true
    }

    /// Keep the cursor inside the rows the pane can show.
    fn follow(&mut self) {
        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        } else if self.cursor >= self.scroll + self.rows {
            self.scroll = self.cursor + 1 - self.rows;
        }
    }

    pub fn set_tab(&mut self, tab: Tab) -> bool {
        if self.tab == tab {
            return false;
        }
        self.tab = tab;
        self.cursor = 0;
        self.scroll = 0;
        self.select();
        true
    }
}
