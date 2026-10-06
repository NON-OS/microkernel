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

//! `ESC [` sequences, routed by private marker, intermediates and final.

use super::state::Term;
use crate::parser::Seq;

impl Term {
    pub(super) fn csi_dispatch(&mut self, seq: &Seq) {
        let p = &seq.params;
        let n = |d: u16| p.count(0, d) as usize;
        match (seq.private, seq.inter(), seq.final_byte) {
            (0, [], b'@') => self.insert_chars(n(1)),
            (0, [], b'A') => self.cursor_up(n(1)),
            (0, [], b'B') | (0, [], b'e') => self.cursor_down(n(1)),
            (0, [], b'C') | (0, [], b'a') => self.cursor_forward(n(1)),
            (0, [], b'D') => self.cursor_back(n(1)),
            (0, [], b'E') => self.cursor_line(n(1), true),
            (0, [], b'F') => self.cursor_line(n(1), false),
            (0, [], b'G') | (0, [], b'`') => self.cursor_column(n(1)),
            (0, [], b'H') | (0, [], b'f') => {
                self.cursor_to(p.count(0, 1) as usize, p.count(1, 1) as usize)
            }
            (0, [], b'I') => self.tab_forward(n(1)),
            (0 | b'?', [], b'J') => self.erase_display(p.get(0)),
            (0 | b'?', [], b'K') => self.erase_in_line(p.get(0)),
            (0, [], b'L') => self.insert_lines(n(1)),
            (0, [], b'M') => self.delete_lines(n(1)),
            (0, [], b'P') => self.delete_chars(n(1)),
            (0, [], b'S') => self.scroll_up(n(1)),
            // Five parameters make it xterm's mouse highlight, not a scroll.
            (0, [], b'T') if p.len() <= 1 => self.scroll_down(n(1)),
            (0, [], b'X') => self.erase_chars(n(1)),
            (0, [], b'Z') => self.tab_back(n(1)),
            (0, [], b'b') => self.repeat_last(n(1)),
            (0, [], b'c') if p.get(0) == 0 => self.reply_da1(),
            (b'>', [], b'c') if p.get(0) == 0 => self.reply_da2(),
            (0, [], b'd') => self.cursor_row(n(1)),
            (0, [], b'g') => self.clear_tabs(p.get(0)),
            (0, [], b'h') => self.set_ansi_modes(p, true),
            (0, [], b'l') => self.set_ansi_modes(p, false),
            (b'?', [], b'h') => self.set_dec_modes(p, true),
            (b'?', [], b'l') => self.set_dec_modes(p, false),
            (0, [], b'm') => self.sgr(p),
            (0, [], b'n') => self.reply_status(p.get(0), false),
            (b'?', [], b'n') => self.reply_status(p.get(0), true),
            (0, [], b'r') => self.set_region(p.count(0, 1) as usize, p.count(1, 0) as usize),
            (0, [], b's') => self.save_cursor(),
            (0, [], b't') => self.window_op(p),
            (0, [], b'u') => self.restore_cursor(),
            (b'>', [], b'q') if p.get(0) == 0 => self.reply_version(),
            (0 | b'?', [b'$'], b'p') => self.reply_mode(p.get(0), seq.private == b'?'),
            (0, [b' '], b'q') => self.set_cursor_style(p.get(0)),
            (0, [b'!'], b'p') => self.soft_reset(),
            _ => {}
        }
    }
}
