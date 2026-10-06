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

//! DECSC and DECRC: the cursor, its pen and the modes that go with it.

use super::screen::Cursor;
use super::state::Term;
use crate::charset::Charsets;

/// What DECSC saves and DECRC restores.
#[derive(Clone, Copy, Debug)]
pub struct Saved {
    pub cursor: Cursor,
    pub origin: bool,
    pub autowrap: bool,
    pub charsets: Charsets,
}

impl Term {
    pub(super) fn save_cursor(&mut self) {
        let saved = Saved {
            cursor: self.scr_ref().cur,
            origin: self.modes.origin,
            autowrap: self.modes.autowrap,
            charsets: self.charsets,
        };
        self.scr().saved = Some(saved);
    }

    /// Restore what was saved, or the home position with a plain pen when
    /// nothing was, as xterm does.
    pub(super) fn restore_cursor(&mut self) {
        let (cols, rows) = (self.cols, self.rows);
        let saved = self.scr_ref().saved;
        let (mut cur, origin, autowrap, charsets) = match saved {
            Some(s) => (s.cursor, s.origin, s.autowrap, s.charsets),
            None => (Cursor::default(), false, true, Charsets::default()),
        };
        cur.x = cur.x.min(cols - 1);
        cur.y = cur.y.min(rows - 1);
        self.modes.origin = origin;
        self.modes.autowrap = autowrap;
        self.charsets = charsets;
        self.scr().cur = cur;
    }
}
