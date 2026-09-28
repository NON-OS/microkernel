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

//! The word or the whole line a double or triple click picks.

use super::state::Term;
use super::types::Pos;

/// Characters a double click keeps together, so a path or a URL is picked
/// whole.
fn is_word(c: char) -> bool {
    c.is_alphanumeric() || "_-./~:@%+#?&=".contains(c)
}

impl Term {
    /// The word under `p`, as the positions of its first and last cells.
    pub fn word_at(&self, p: Pos) -> (Pos, Pos) {
        let Some(line) = self.line_at(p.line) else { return (p, p) };
        let class = |x: usize| {
            let c = line.cell(x);
            if c.is_tail() {
                None
            } else {
                Some(is_word(c.ch))
            }
        };
        let here = class(p.col).unwrap_or(false);
        let same = |x: usize| class(x).is_none_or(|w| w == here);
        let mut l = p.col;
        while l > 0 && same(l - 1) {
            l -= 1;
        }
        let mut r = p.col;
        while r + 1 < self.cols && same(r + 1) {
            r += 1;
        }
        (Pos { line: p.line, col: l }, Pos { line: p.line, col: r })
    }

    /// The whole line `p` is on, following wraps both ways, for a triple
    /// click.
    pub fn line_bounds(&self, p: Pos) -> (Pos, Pos) {
        let mut first = p.line;
        while first > self.first_line() && self.line_at(first - 1).is_some_and(|l| l.wrapped) {
            first -= 1;
        }
        let mut last = p.line;
        while self.line_at(last).is_some_and(|l| l.wrapped) && last < self.last_line() {
            last += 1;
        }
        (Pos { line: first, col: 0 }, Pos { line: last, col: self.cols - 1 })
    }
}
