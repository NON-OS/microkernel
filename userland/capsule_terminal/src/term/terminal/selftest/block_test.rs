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

//! Command blocks: the model, durations, absolute lines and timestamps.

use nonos_vt::Term;

use super::mark;
use crate::term::dimensions::{COLS, VISIBLE_ROWS};

pub(super) fn run() {
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"12:34:56");
        st.close_block(false, 0);
        let one = st.blocks.len() == 1;
        let err = st.blocks[0].status == crate::term::block::Status::Err;
        let ts = &st.blocks[0].ts == b"12:34:56";
        let found = st.block_at(st.blocks[0].start_abs).is_some();
        mark(b"block-model", one && err && ts && found);
    }
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"00:00:00");
        st.close_block(true, 1500);
        let ok =
            st.blocks[0].dur_ms == 1500 && st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-dur", ok);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 100);
        let start = t.cursor_pos().line;
        for _ in 0..(VISIBLE_ROWS + 3) {
            t.feed(b"x\r\n");
        }
        let scrolled = t.first_line() + t.history_len() as u64 == 4;
        let row_abs = t.abs_of_row(0) == 4 - t.view_offset() as u64;
        mark(b"block-absline", start == 0 && scrolled && row_abs);
    }
    {
        let f = crate::term::rtc::fmt_hms(9, 5, 42);
        mark(b"block-ts", &f == b"09:05:42");
    }
    {
        let mut st = crate::term::state::State::new();
        st.line.replace(b"echo hi");
        let _ = crate::event::on_enter(&mut st);
        let opened = st.blocks.len() == 1;
        let ok = st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-capture", opened && ok);
    }
    {
        let (a, an) = crate::term::dur::fmt_dur(1800);
        let (b, bn) = crate::term::dur::fmt_dur(250);
        let (c, cn) = crate::term::dur::fmt_dur(125_000);
        let ok = &a[..an] == b"1.8s" && &b[..bn] == b"250ms" && &c[..cn] == b"2m05s";
        mark(b"dur-fmt", ok);
    }
}
