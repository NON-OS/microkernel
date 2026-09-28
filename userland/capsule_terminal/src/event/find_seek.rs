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

//! Where a search starts, the next match from a place, and bringing a
//! match into view.

use nonos_vt::Pos;

use crate::term::select::Find;
use crate::term::state::State;

/// Just past the newest cell, so a backward search starts at the bottom.
pub(super) fn end(state: &State) -> Pos {
    Pos { line: state.scrollback.vt.last_line() + 1, col: 0 }
}

pub(super) fn seek(state: &State, f: &Find, from: Pos, older: bool) -> Option<(Pos, Pos)> {
    if f.query.is_empty() {
        return None;
    }
    state.scrollback.vt.find(&f.query, from, older, f.case)
}

/// Scroll history so `line` sits mid-screen.
pub(super) fn reveal(state: &mut State, line: u64) {
    let vt = &mut state.scrollback.vt;
    let rows = vt.rows() as u64;
    let bottom = vt.cursor_pos().line;
    let back = (bottom + rows / 2).saturating_sub(line + rows);
    vt.scroll_to_bottom();
    vt.scroll_view(back.min(isize::MAX as u64) as isize);
}
