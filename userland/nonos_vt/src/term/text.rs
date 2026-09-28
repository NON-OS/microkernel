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

//! Reading text back out, for copying a selection.

use alloc::string::String;

use super::state::Term;
use super::types::Pos;
use crate::line::Line;

fn push_cell(out: &mut String, line: &Line, x: usize) {
    let c = line.cell(x);
    if c.is_tail() {
        return;
    }
    out.push(c.ch);
    if let Some(m) = line.marks_of(&c) {
        out.push_str(m);
    }
}

impl Term {
    /// The text from `a` to `b`, both inclusive, in either order. Lines that
    /// wrapped join without a break; other line ends lose trailing blanks
    /// and become newlines. `block` takes the same columns from each line.
    pub fn text_between(&self, a: Pos, b: Pos, block: bool) -> String {
        let (s, e) = if a <= b { (a, b) } else { (b, a) };
        let (left, right) = (s.col.min(e.col), s.col.max(e.col));
        let mut out = String::new();
        let mut line_no = s.line;
        while line_no <= e.line {
            let Some(line) = self.line_at(line_no) else {
                line_no += 1;
                continue;
            };
            let (from, to) = if block {
                (left, right)
            } else {
                let from = if line_no == s.line { s.col } else { 0 };
                let to = if line_no == e.line { e.col } else { self.cols - 1 };
                (from, to)
            };
            let mut piece = String::new();
            for x in from..=to.min(self.cols - 1) {
                push_cell(&mut piece, line, x);
            }
            let runs_on = line.wrapped && !block && line_no != e.line;
            if !runs_on {
                let trimmed = piece.trim_end_matches(' ').len();
                piece.truncate(trimmed);
            }
            out.push_str(&piece);
            if line_no != e.line && !runs_on {
                out.push('\n');
            }
            line_no += 1;
        }
        out
    }
}
