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

//! C0 control bytes, and the line movements they share with ESC sequences.

use super::state::Term;

impl Term {
    pub(super) fn control(&mut self, b: u8) {
        match b {
            0x07 => self.bell = self.bell.saturating_add(1),
            0x08 => self.backspace(),
            0x09 => self.tab_forward(1),
            0x0A..=0x0C => {
                if self.modes.newline {
                    self.carriage_return();
                }
                self.index();
            }
            0x0D => self.carriage_return(),
            0x0E => self.charsets.gl = 1,
            0x0F => self.charsets.gl = 0,
            _ => {}
        }
    }

    fn backspace(&mut self) {
        let cur = &mut self.scr().cur;
        if cur.pending_wrap {
            cur.pending_wrap = false;
        } else {
            cur.x = cur.x.saturating_sub(1);
        }
    }

    pub(super) fn carriage_return(&mut self) {
        let cur = &mut self.scr().cur;
        cur.x = 0;
        cur.pending_wrap = false;
    }
}
