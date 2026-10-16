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

//! A reply read in order, every field bounds-checked.

pub(super) struct At<'a>(pub &'a [u8], pub usize);

impl<'a> At<'a> {
    pub(super) fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let b = self.0.get(self.1..self.1.checked_add(n)?)?;
        self.1 += n;
        Some(b)
    }
    pub(super) fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    pub(super) fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    pub(super) fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    /// All ones on the wire is a field the firmware left unknown.
    pub(super) fn u64(&mut self) -> Option<Option<u64>> {
        let v = u64::from_le_bytes(self.take(8)?.try_into().ok()?);
        Some((v != u64::MAX).then_some(v))
    }
}
