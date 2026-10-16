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

//! The VT driven by escapes: CSI, SGR, the scrollback feed, history, alt screen.

use nonos_vt::Term;

use super::{mark, row_text};
use crate::term::dimensions::{COLS, VISIBLE_ROWS};

pub(super) fn run() {
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"\x1b[5;3HX");
        let pos_ok = t.visible_line(4).cell(2).ch == 'X';
        t.feed(b"\x1b[2J");
        mark(b"vt-csi", pos_ok && row_text(&t, 4).is_empty());
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"\x1b[1;31mZ\x1b[0mY");
        let z = t.visible_line(0).cell(0);
        let y = t.visible_line(0).cell(1);
        let set = z.attr & nonos_vt::cell::attr::BOLD != 0 && z.fg == nonos_vt::Color::Indexed(1);
        mark(b"vt-sgr", set && y.attr == 0 && y.fg == nonos_vt::Color::Default);
    }
    {
        let mut sb = crate::term::scrollback::Scrollback::new();
        sb.feed_raw(b"hi\x1b[32m!\n");
        let bang = sb.vt.visible_line(0).cell(2);
        let ok = row_text(&sb.vt, 0) == "hi!" && bang.fg == nonos_vt::Color::Indexed(2);
        mark(b"vt-feed", ok && sb.vt.cursor().x == 0 && sb.vt.cursor().y == 1);
    }
    {
        let mut sb = crate::term::scrollback::Scrollback::new();
        sb.push_line(b"MIR");
        mark(b"vt-mirror", row_text(&sb.vt, 0) == "MIR");
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 100);
        for i in 0..VISIBLE_ROWS as u8 + 5 {
            t.feed(&[b'A' + i % 26, b'\r', b'\n']);
        }
        let has_hist = t.history_len() >= 5;
        t.scroll_view(1);
        let off1 = t.view_offset() == 1;
        t.scroll_to_bottom();
        mark(b"vt-history", has_hist && off1 && t.view_offset() == 0);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"MAIN\x1b[?1049h");
        let in_alt = t.alt_active() && row_text(&t, 0).is_empty();
        t.feed(b"ALT\x1b[?1049l");
        mark(b"vt-altscreen", in_alt && !t.alt_active() && row_text(&t, 0) == "MAIN");
    }
}
