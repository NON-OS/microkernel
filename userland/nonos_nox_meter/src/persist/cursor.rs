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

//! Reads that refuse to run past the end rather than panic.

pub(super) struct Cursor<'a> {
    pub(super) b: &'a [u8],
    pub(super) at: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(n)?;
        let s = self.b.get(self.at..end)?;
        self.at = end;
        Some(s)
    }

    pub(super) fn fixed<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.take(N)?.try_into().ok()
    }

    pub(super) fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.fixed()?))
    }

    pub(super) fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.fixed()?))
    }

    pub(super) fn u128(&mut self) -> Option<u128> {
        Some(u128::from_le_bytes(self.fixed()?))
    }

    /// The next `n` bytes, which must all be zero; an unused slot holds nothing.
    pub(super) fn zeros(&mut self, n: usize) -> Option<()> {
        self.take(n)?.iter().all(|&x| x == 0).then_some(())
    }
}
