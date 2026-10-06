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

//! The alternate screen full-screen programs draw on, which leaves the
//! normal screen and its history untouched underneath.

use super::state::Term;

impl Term {
    /// Switch to the alternate screen. The cursor carries over, as the two
    /// screens share one in xterm.
    pub(super) fn enter_alt(&mut self, clear: bool) {
        if !self.alt_active {
            self.alt.cur = self.primary.cur;
            self.alt.reset_region();
            self.alt_active = true;
            self.view = 0;
        }
        if clear {
            let blank = self.alt.blank();
            for line in self.alt.lines.iter_mut() {
                line.clear(blank);
            }
        }
        self.touch_all();
    }

    pub(super) fn leave_alt(&mut self, clear_first: bool) {
        if !self.alt_active {
            return;
        }
        if clear_first {
            let blank = self.alt.blank();
            for line in self.alt.lines.iter_mut() {
                line.clear(blank);
            }
        }
        self.primary.cur = self.alt.cur;
        self.alt_active = false;
        self.touch_all();
    }

    pub fn alt_active(&self) -> bool {
        self.alt_active
    }
}
