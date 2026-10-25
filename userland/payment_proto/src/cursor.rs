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

//! Reads that refuse to run past the end rather than panic, and the u16
//! length prefix every string on this wire carries.

pub(crate) struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(b: &'a [u8]) -> Self {
        Cursor { b, at: 0 }
    }

    pub(crate) fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(n)?;
        let s = self.b.get(self.at..end)?;
        self.at = end;
        Some(s)
    }

    pub(crate) fn fixed<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.take(N)?.try_into().ok()
    }

    pub(crate) fn u8(&mut self) -> Option<u8> {
        Some(self.fixed::<1>()?[0])
    }

    pub(crate) fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.fixed()?))
    }

    pub(crate) fn u128(&mut self) -> Option<u128> {
        Some(u128::from_le_bytes(self.fixed()?))
    }

    /// A u16 length, then that many bytes.
    pub(crate) fn lp(&mut self) -> Option<&'a [u8]> {
        let n = u16::from_le_bytes(self.fixed()?) as usize;
        self.take(n)
    }

    /// Some(v) only when nothing follows: a body has one reading.
    pub(crate) fn end<T>(&self, v: T) -> Option<T> {
        (self.at == self.b.len()).then_some(v)
    }
}
