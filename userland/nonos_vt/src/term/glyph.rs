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

//! Putting a character into cells at the cursor, wrapping first when the
//! last column was already written.

use super::state::Term;
use crate::cell::{attr, Cell};

impl Term {
    pub(super) fn put_glyph(&mut self, c: char, w: usize) {
        let cols = self.cols;
        let autowrap = self.modes.autowrap;
        if self.scr_ref().cur.pending_wrap {
            self.scr().cur.pending_wrap = false;
            self.wrap_to_next_line();
        }
        /*
         * A wide character never straddles the edge: it wraps whole, or is
         * dropped when wrapping is off.
         */
        if w == 2 && self.scr_ref().cur.x + 1 >= cols {
            if !autowrap {
                return;
            }
            self.wrap_to_next_line();
        }
        let insert = self.modes.insert;
        let scr = self.scr();
        let (x, y, pen) = (scr.cur.x, scr.cur.y, scr.cur.pen);
        let blank = scr.blank();
        let line = &mut scr.lines[y];
        if insert {
            line.insert_blanks(x, w, blank);
        }
        let base = Cell {
            ch: c,
            fg: pen.fg,
            bg: pen.bg,
            attr: pen.attr & attr::PEN,
            link: pen.link,
            mark: 0,
        };
        if w == 2 {
            let head = Cell { attr: base.attr | attr::WIDE, ..base };
            let tail = Cell { ch: ' ', attr: base.attr | attr::WIDE_TAIL, ..base };
            line.put_pair(x, head, tail);
        } else {
            line.put(x, base);
        }
        if x + w >= cols {
            scr.cur.x = cols - 1;
            scr.cur.pending_wrap = autowrap;
        } else {
            scr.cur.x = x + w;
        }
        self.touch(y);
    }
}
