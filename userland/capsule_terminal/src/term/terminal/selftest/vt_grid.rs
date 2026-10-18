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

//! The VT on its own: palette, grid, scrolling, and the parser.

use nonos_vt::Term;

use super::{mark, row_text};
use crate::term::dimensions::{COLS, VISIBLE_ROWS};

pub(super) fn run() {
    mark(b"vt-skeleton", true);
    {
        let p = nonos_vt::Palette::xterm();
        mark(b"vt-color", p.colors[1] == 0xCD_0000 && p.colors[15] == 0xFF_FFFF);
    }
    {
        let t = Term::new(COLS, VISIBLE_ROWS, 10);
        let ok =
            t.cols() == COLS && t.rows() == VISIBLE_ROWS && t.visible_line(0).cell(0).ch == ' ';
        mark(b"vt-grid", ok && t.cursor().x == 0 && t.cursor().y == 0);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"AB\rC");
        let row0_ok = row_text(&t, 0) == "CB";
        for _ in 0..VISIBLE_ROWS {
            t.feed(b"\n");
        }
        let scrolled_ok = t.history_len() >= 1;
        t.feed(b"Z\x1b[2J");
        mark(b"vt-grid-ops", row0_ok && scrolled_ok && row_text(&t, 0).is_empty());
    }
    {
        // The parser on its own: prints, controls and CSI finals in order.
        struct Rec(alloc::vec::Vec<u8>);
        impl nonos_vt::parser::Handler for Rec {
            fn print(&mut self, c: char) {
                self.0.push(c as u8);
            }
            fn execute(&mut self, b: u8) {
                self.0.push(b);
            }
            fn csi(&mut self, s: &nonos_vt::parser::Seq) {
                self.0.push(s.final_byte);
            }
            fn esc(&mut self, _s: &nonos_vt::parser::Seq) {}
            fn osc(&mut self, _d: &[u8]) {}
            fn dcs(&mut self, _s: &nonos_vt::parser::Seq, _d: &[u8]) {}
        }
        let mut rec = Rec(alloc::vec::Vec::new());
        nonos_vt::parser::Parser::new().feed(&mut rec, b"A\x1b[31mB\x1b[0m\n\x1b[2J");
        mark(b"vt-parser", rec.0 == b"AmBm\nJ");
    }
}
