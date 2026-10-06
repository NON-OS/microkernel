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

//! `ESC` sequences that are not CSI, OSC or DCS.

use super::state::Term;
use crate::charset::designate;
use crate::parser::Seq;

impl Term {
    pub(super) fn esc_dispatch(&mut self, seq: &Seq) {
        let f = seq.final_byte;
        match seq.inter() {
            [] => match f {
                b'7' => self.save_cursor(),
                b'8' => self.restore_cursor(),
                b'D' => self.index(),
                b'E' => {
                    self.carriage_return();
                    self.index();
                }
                b'H' => self.set_tab(),
                b'M' => self.reverse_index(),
                b'c' => self.full_reset(),
                b'=' => self.modes.keypad = true,
                b'>' => self.modes.keypad = false,
                b'N' => self.charsets.single = Some(2),
                b'O' => self.charsets.single = Some(3),
                b'Z' => self.reply_da1(),
                _ => {}
            },
            [b'('] => self.charsets.g[0] = designate(f),
            [b')'] => self.charsets.g[1] = designate(f),
            [b'*'] => self.charsets.g[2] = designate(f),
            [b'+'] => self.charsets.g[3] = designate(f),
            [b'#'] if f == b'8' => self.align_test(),
            _ => {}
        }
    }

    /// DECALN: fill the screen with `E`, for lining up a display.
    fn align_test(&mut self) {
        let scr = self.scr();
        for line in scr.lines.iter_mut() {
            line.clear(crate::cell::Cell { ch: 'E', ..crate::cell::Cell::BLANK });
        }
        scr.reset_region();
        scr.cur.x = 0;
        scr.cur.y = 0;
        scr.cur.pending_wrap = false;
        self.touch_all();
    }
}
