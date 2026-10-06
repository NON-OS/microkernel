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

//! Answers to what a program asks: identity, cursor position, modes and
//! size. Many programs wait for these, so an unanswered question can hang
//! them. Answers queue for the host to write back to the program.

use crate::term::state::Term;

impl Term {
    /// DA1: a VT220 with ANSI colour.
    pub(in crate::term) fn reply_da1(&mut self) {
        self.reply(b"\x1b[?62;22c");
    }

    /// DA2: terminal type 1, version 10.
    pub(in crate::term) fn reply_da2(&mut self) {
        self.reply(b"\x1b[>1;10;0c");
    }

    pub(in crate::term) fn reply_version(&mut self) {
        self.reply(b"\x1bP>|NONOS vt 0.1\x1b\\");
    }

    /// DSR 5 is status, DSR 6 the cursor position, 1-based and counted from
    /// the region in origin mode.
    pub(in crate::term) fn reply_status(&mut self, what: u16, dec: bool) {
        match what {
            5 if !dec => self.reply(b"\x1b[0n"),
            6 => {
                let s = self.scr_ref();
                let top = if self.modes.origin { s.top } else { 0 };
                let (row, col) = (s.cur.y - top.min(s.cur.y) + 1, s.cur.x + 1);
                let q = if dec { "?" } else { "" };
                self.reply_fmt(format_args!("\x1b[{q}{row};{col}R"));
            }
            _ => {}
        }
    }

    /// DECRQM: 1 set, 2 reset, 0 not recognised.
    pub(in crate::term) fn reply_mode(&mut self, mode: u16, dec: bool) {
        let v = match self.mode_state(mode, dec) {
            Some(true) => 1,
            Some(false) => 2,
            None => 0,
        };
        let q = if dec { "?" } else { "" };
        self.reply_fmt(format_args!("\x1b[{q}{mode};{v}$y"));
    }
}
