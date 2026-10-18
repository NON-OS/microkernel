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

//! Running one line through the shell and grading what it left on screen.

use nonos_libc::mk_debug;
use nonos_vt::Term;

use crate::term::state::State;
use crate::term::util::copy_into;

pub(super) fn run_cmd(state: &mut State, cmd: &[u8]) {
    state.scrollback.clear();
    state.line.replace(cmd);
    let _ = crate::event::on_enter(state);
}

pub(super) fn ok_cmd(state: &mut State, cmd: &[u8]) -> bool {
    run_cmd(state, cmd);
    state.last_status == 0
}

pub(super) fn visible_has(state: &State, needle: &[u8]) -> bool {
    let vt = &state.scrollback.vt;
    (0..vt.rows()).any(|row| row_text(vt, row).as_bytes() == needle)
}

/// Visible row `row` without trailing blanks.
pub(super) fn row_text(vt: &Term, row: usize) -> alloc::string::String {
    let line = vt.visible_line(row);
    let s: alloc::string::String =
        (0..vt.cols()).map(|x| line.cell(x)).filter(|c| !c.is_tail()).map(|c| c.ch).collect();
    alloc::string::String::from(s.trim_end())
}

pub(super) fn mark(step: &[u8], ok: bool) {
    let mut buf = [0u8; 64];
    let mut n = 0;
    n += copy_into(&mut buf[n..], b"[TERMINAL-TEST] ");
    n += copy_into(&mut buf[n..], step);
    n += copy_into(&mut buf[n..], if ok { b" ok\n" } else { b" FAIL\n" });
    let _ = mk_debug(buf.as_ptr(), n);
}
