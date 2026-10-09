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

//! DECSTR: a soft reset.

use super::state::Term;
use crate::cell::Pen;
use crate::charset::Charsets;

impl Term {
    /// DECSTR: modes and pen to their defaults, screen contents untouched.
    pub(super) fn soft_reset(&mut self) {
        let m = &mut self.modes;
        m.insert = false;
        m.origin = false;
        m.autowrap = true;
        m.cursor_visible = true;
        m.keypad = false;
        m.cursor_keys = false;
        self.charsets = Charsets::default();
        let scr = self.scr();
        scr.reset_region();
        scr.cur.pen = Pen::default();
        scr.cur.pending_wrap = false;
        scr.saved = None;
    }
}
