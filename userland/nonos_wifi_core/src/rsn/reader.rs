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

//! A bounded little-endian cursor over an RSN element's body: every read
//! that would run past the element fails.

use super::suite::selector;

// A bounded little-endian cursor over the element body.
pub(super) struct Reader<'a> {
    pub(super) b: &'a [u8],
    pub(super) off: usize,
}

impl Reader<'_> {
    pub(super) fn remaining(&self) -> usize {
        self.b.len() - self.off
    }
    pub(super) fn done(&self) -> bool {
        self.off >= self.b.len()
    }
    pub(super) fn take(&mut self, n: usize) -> Option<&[u8]> {
        let end = self.off.checked_add(n)?;
        let s = self.b.get(self.off..end)?;
        self.off = end;
        Some(s)
    }
    pub(super) fn skip(&mut self, n: usize) -> Option<()> {
        self.take(n).map(|_| ())
    }
    pub(super) fn u16(&mut self) -> Option<u16> {
        let s = self.take(2)?;
        Some(u16::from_le_bytes([s[0], s[1]]))
    }
    pub(super) fn suite(&mut self) -> Option<u32> {
        let s = self.take(4)?;
        Some(selector([s[0], s[1], s[2], s[3]]))
    }
}
