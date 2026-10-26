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

//! What the headless run's screen showed, copied to the serial log a row at
//! a time as `[AGENTPROOF-OUT] <row>`, history first, so a host reads the
//! program's own output beside the box's line.

use nonos_libc::{mk_debug, mk_exit};

use crate::term::state::State;

/// One line to the serial log, in the pieces the kernel takes (256 bytes).
pub fn say(line: &str) {
    for piece in line.as_bytes().chunks(256) {
        let _ = mk_debug(piece.as_ptr(), piece.len());
    }
}

pub fn dump(state: &mut State) {
    let vt = &mut state.scrollback.vt;
    let back = vt.history_len();
    // Each history row once, oldest first: with the view `back - i` rows back,
    // row 0 on view is history row i.
    for i in 0..back {
        vt.scroll_to_bottom();
        vt.scroll_view((back - i) as isize);
        out(vt.visible_line(0), vt.cols());
    }
    vt.scroll_to_bottom();
    for row in 0..vt.rows() {
        out(vt.visible_line(row), vt.cols());
    }
}

fn out(line: &nonos_vt::line::Line, cols: usize) {
    let text: alloc::string::String =
        (0..cols).map(|x| line.cell(x)).filter(|c| !c.is_tail()).map(|c| c.ch).collect();
    let text = text.trim_end();
    if !text.is_empty() {
        say(&alloc::format!("[AGENTPROOF-OUT] {text}\n"));
    }
}

pub fn lossy(b: &[u8]) -> alloc::string::String {
    alloc::string::String::from_utf8_lossy(b).into_owned()
}

pub fn done(line: &[u8], code: i32) -> ! {
    say(&lossy(line));
    mk_exit(code)
}
