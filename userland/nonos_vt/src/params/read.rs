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

//! Reading parameters back, by position or grouped with their
//! sub-parameters.

use super::Params;

impl Params {
    /// Value `i`, top-level and sub-parameters counted alike.
    pub fn raw(&self, i: usize) -> Option<u16> {
        (i < self.len).then(|| self.vals[i])
    }

    /// Parameter `i` counting only top-level values, 0 when absent.
    pub fn get(&self, i: usize) -> u16 {
        self.groups().nth(i).and_then(|g| g.first().copied()).unwrap_or(0)
    }

    /// Parameter `i`, with 0 and absent both meaning `d`, as most sequences
    /// read a count or a position.
    pub fn count(&self, i: usize, d: u16) -> u16 {
        match self.get(i) {
            0 => d,
            v => v,
        }
    }

    /// Each top-level value with its sub-parameters.
    pub fn groups(&self) -> Groups<'_> {
        Groups { p: self, i: 0 }
    }
}

pub struct Groups<'a> {
    p: &'a Params,
    i: usize,
}

impl<'a> Iterator for Groups<'a> {
    type Item = &'a [u16];

    fn next(&mut self) -> Option<&'a [u16]> {
        let start = self.i;
        if start >= self.p.len {
            return None;
        }
        let mut end = start + 1;
        while end < self.p.len && self.p.sub & (1 << end) != 0 {
            end += 1;
        }
        self.i = end;
        Some(&self.p.vals[start..end])
    }
}
