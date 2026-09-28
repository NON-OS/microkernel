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

//! DCS requests: DECRQSS for a setting, and XTGETTCAP for terminfo
//! capabilities. Editors ask these at start-up and wait for an answer, so
//! each is answered, as unknown when it is.

use crate::parser::Seq;
use crate::term::state::Term;
use crate::term::types::CursorShape;

impl Term {
    pub(in crate::term) fn dcs_dispatch(&mut self, seq: &Seq, data: &[u8]) {
        match (seq.private, seq.inter(), seq.final_byte) {
            (0, [b'$'], b'q') => self.decrqss(data),
            (0, [b'+'], b'q') => self.xtgettcap(data),
            _ => {}
        }
    }

    fn decrqss(&mut self, what: &[u8]) {
        let answer = match what {
            b"r" => {
                let s = self.scr_ref();
                Some(alloc::format!("{};{}r", s.top + 1, s.bot + 1))
            }
            b" q" => {
                let n = match (self.cursor_shape, self.modes.cursor_blink) {
                    (CursorShape::Block, true) => 1,
                    (CursorShape::Block, false) => 2,
                    (CursorShape::Underline, true) => 3,
                    (CursorShape::Underline, false) => 4,
                    (CursorShape::Bar, true) => 5,
                    (CursorShape::Bar, false) => 6,
                };
                Some(alloc::format!("{n} q"))
            }
            _ => None,
        };
        match answer {
            Some(a) => self.reply_fmt(format_args!("\x1bP1$r{a}\x1b\\")),
            None => self.reply(b"\x1bP0$r\x1b\\"),
        }
    }
}
