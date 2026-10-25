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

//! One structure (DSP0134 6.1.2): a four-byte header (type, length,
//! handle), the formatted area of `length` bytes counting the header, and
//! its strings, each NUL-terminated, the set ended by a second NUL. A byte
//! in the formatted area that names a string holds its index from 1; 0
//! names none.

use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Structure<'a> {
    pub kind: u8,
    pub handle: u16,
    /// The formatted area, header included.
    pub formatted: &'a [u8],
    pub strings: Vec<&'a [u8]>,
}

impl<'a> Structure<'a> {
    /// The byte at `offset` in the formatted area; None past its end, as
    /// for a field an older version of the structure does not have.
    pub fn byte(&self, offset: usize) -> Option<u8> {
        self.formatted.get(offset).copied()
    }

    /// The string the byte at `offset` names.
    pub fn string(&self, offset: usize) -> Option<&'a [u8]> {
        let index = usize::from(self.byte(offset)?);
        index.checked_sub(1).and_then(|i| self.strings.get(i).copied())
    }
}
