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

//! A span of the screen and its history picked with the pointer, and the
//! match a scrollback search is on.

use nonos_vt::Pos;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Selection {
    /// Where the press landed, and where the pointer is now.
    pub anchor: Pos,
    pub head: Pos,
    /// Alt held: a rectangle of columns rather than a run of text.
    pub block: bool,
}

impl Selection {
    pub fn ordered(&self) -> (Pos, Pos) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    pub fn contains(&self, line: u64, col: usize) -> bool {
        let (a, b) = self.ordered();
        if line < a.line || line > b.line {
            return false;
        }
        if self.block {
            let (l, r) = (a.col.min(b.col), a.col.max(b.col));
            return col >= l && col <= r;
        }
        let from = if line == a.line { a.col } else { 0 };
        let to = if line == b.line { b.col } else { usize::MAX };
        col >= from && col <= to
    }
}

/// A search through the screen and its history.
#[derive(Clone, Default, Debug)]
pub struct Find {
    pub query: alloc::string::String,
    /// The match on view, first and last cells.
    pub hit: Option<(Pos, Pos)>,
    /// Upper and lower case count as different.
    pub case: bool,
}

/// A span to shade: a search match.
pub fn in_span(span: Option<(Pos, Pos)>, line: u64, col: usize) -> bool {
    match span {
        Some((a, b)) => {
            let p = Pos { line, col };
            p >= a && p <= b
        }
        None => false,
    }
}
