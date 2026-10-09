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

//! Whether a mode is set, for DECRQM.

use super::mouse_mode::MouseMode;
use super::state::Term;

impl Term {
    /// Whether a mode is set: `Some(true)` set, `Some(false)` reset, `None`
    /// not a mode this terminal knows.
    pub(super) fn mode_state(&self, mode: u16, dec: bool) -> Option<bool> {
        let m = &self.modes;
        if !dec {
            return match mode {
                4 => Some(m.insert),
                20 => Some(m.newline),
                _ => None,
            };
        }
        Some(match mode {
            1 => m.cursor_keys,
            5 => m.reverse_video,
            6 => m.origin,
            7 => m.autowrap,
            12 => m.cursor_blink,
            25 => m.cursor_visible,
            66 => m.keypad,
            9 => m.mouse == MouseMode::Press,
            1000 => m.mouse == MouseMode::Click,
            1002 => m.mouse == MouseMode::Drag,
            1003 => m.mouse == MouseMode::Motion,
            1004 => m.focus,
            1006 => m.mouse_sgr,
            1007 => m.alt_scroll,
            2004 => m.bracketed_paste,
            2026 => m.sync,
            7727 => m.raw_input,
            47 | 1047 | 1049 => self.alt_active,
            _ => return None,
        })
    }
}
