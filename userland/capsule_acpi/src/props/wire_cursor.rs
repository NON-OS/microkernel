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

//! The reader's cursor over a PROPERTIES reply: each take is bounds
//! checked, and a count past the wire's `MAX` stops the read.

use super::wire::MAX;
use alloc::vec::Vec;

pub(super) struct Cursor<'a> {
    pub b: &'a [u8],
    pub at: usize,
}

impl Cursor<'_> {
    pub fn take(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.b.get(self.at..self.at.checked_add(n)?)?;
        self.at += n;
        Some(s)
    }

    pub fn byte(&mut self) -> Option<u8> {
        self.take(1).map(|b| b[0])
    }

    pub fn name(&mut self) -> Option<Vec<u8>> {
        let n = self.byte()? as usize;
        self.take(n).map(<[u8]>::to_vec)
    }

    pub fn count(&mut self) -> Option<usize> {
        Some(self.byte()? as usize).filter(|n| *n <= MAX)
    }
}
