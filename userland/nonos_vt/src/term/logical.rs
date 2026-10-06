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

//! A logical line: the physical lines one run of text wrapped across.

use super::state::Term;
use super::types::Pos;
use alloc::vec::Vec;

impl Term {
    /// The logical line starting at `first`: its characters, each with the
    /// position of its cell, and the absolute line after it.
    pub(super) fn logical(&self, first: u64) -> (Vec<(char, Pos)>, u64) {
        let mut chars = Vec::new();
        let mut n = first;
        while let Some(line) = self.line_at(n) {
            let len = if line.wrapped { line.len() } else { line.content_len() };
            for x in 0..len.min(self.cols) {
                let c = line.cell(x);
                if !c.is_tail() {
                    chars.push((c.ch, Pos { line: n, col: x }));
                }
            }
            n += 1;
            if !line.wrapped {
                break;
            }
        }
        (chars, n)
    }
}
