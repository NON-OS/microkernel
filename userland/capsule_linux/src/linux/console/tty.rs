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

//! What the family's terminal means for its output, and how large it is.

use nonos_libc::mk_tty_query;

use super::state::{attached, with};
use super::winsize::shown;

/// True when what the family prints goes only to its launcher: it holds a
/// model, or it is on a terminal.
pub fn private() -> bool {
    crate::linux::file::models::held() || attached()
}

/// The terminal's own size as `(rows, cols)`, or 24 by 80 when it gives none.
fn terminal() -> (u16, u16) {
    let got = [0, 1, 2].into_iter().find_map(mk_tty_query);
    match got {
        Some((cols, rows)) if cols > 0 && rows > 0 => (rows, cols),
        _ => (24, 80),
    }
}

/// The size a guest is shown as `(rows, cols)`: the one a guest set, while
/// the terminal has kept the size it had then, else the terminal's own.
pub fn size() -> (u16, u16) {
    let now = terminal();
    with(|c| shown(c.winsize, now))
}

/// TIOCSWINSZ: the size a guest is shown from now, and whether that changed
/// it, which is when Linux raises SIGWINCH.
pub fn set_size(rows: u16, cols: u16) -> bool {
    let now = terminal();
    with(|c| {
        let before = shown(c.winsize, now);
        c.winsize = Some(((rows, cols), now));
        before != (rows, cols)
    })
}
