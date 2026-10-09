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

//! Which lines fill the body. The shell draws its history ending at the
//! last line written, so output sits just above the prompt. A program in
//! the foreground, or on the alternate screen, owns the whole body and is
//! drawn as its screen.

use nonos_vt::{Line, Term};

#[derive(Clone, Copy)]
pub enum Rows {
    /// Screen rows `0..n`, as the program laid them out.
    Screen,
    /// `n` lines of history ending at the last line written, `back` lines
    /// back into history.
    Shell { rows: usize, back: usize },
}

impl Rows {
    /// The absolute line of the first row drawn.
    pub fn first(&self, vt: &Term) -> u64 {
        match *self {
            Rows::Screen => vt.abs_of_row(0),
            Rows::Shell { rows, back } => {
                let cur = vt.cursor_pos();
                let written = vt.line_at(cur.line).is_some_and(|l| l.content_len() > 0);
                let last =
                    if cur.col > 0 || written { cur.line } else { cur.line.saturating_sub(1) };
                let top = (last + 1).saturating_sub(rows as u64).saturating_sub(back as u64);
                top.max(vt.first_line())
            }
        }
    }

    /// Row `i` of the body, given `top` from `first`: its absolute line
    /// and the line itself.
    pub fn get<'a>(&self, vt: &'a Term, top: u64, i: usize) -> Option<(u64, &'a Line)> {
        match self {
            Rows::Screen => (i < vt.rows()).then(|| (vt.abs_of_row(i), vt.visible_line(i))),
            Rows::Shell { .. } => {
                let abs = top + i as u64;
                let cur = vt.cursor_pos().line;
                if abs > cur {
                    return None;
                }
                vt.line_at(abs).map(|l| (abs, l))
            }
        }
    }
}
