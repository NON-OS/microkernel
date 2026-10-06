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

//! The DEC modes that switch screens or save the cursor: 47 and 1047
//! switch, 1047 clearing the alternate screen on the way back, 1048 saves
//! and restores the cursor, and 1049 does both, as full-screen programs
//! use it.

use super::state::Term;

impl Term {
    pub(super) fn set_screen_mode(&mut self, mode: u16, on: bool) {
        match mode {
            47 => {
                if on {
                    self.enter_alt(false)
                } else {
                    self.leave_alt(false)
                }
            }
            1047 => {
                if on {
                    self.enter_alt(false)
                } else {
                    self.leave_alt(true)
                }
            }
            1048 => {
                if on {
                    self.save_cursor()
                } else {
                    self.restore_cursor()
                }
            }
            1049 => {
                if on {
                    if !self.alt_active {
                        self.save_cursor();
                    }
                    self.enter_alt(true);
                } else if self.alt_active {
                    self.leave_alt(false);
                    self.restore_cursor();
                }
            }
            _ => {}
        }
    }
}
