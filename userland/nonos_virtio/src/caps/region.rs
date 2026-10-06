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

//! One register structure's place: a BAR and a byte range inside it.

/// The broker maps whole pages.
pub const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    pub bar: u8,
    pub offset: u32,
    pub length: u32,
}

impl Region {
    /// One past the last byte. Two 32-bit values cannot overflow 64 bits.
    pub const fn end(self) -> u64 {
        self.offset as u64 + self.length as u64
    }

    /// The whole pages a mapping of the full region covers, as BAR offsets.
    pub const fn page_span(self) -> (u64, u64) {
        let start = self.offset as u64 & !PAGE_MASK;
        let end = (self.end() + PAGE_MASK) & !PAGE_MASK;
        (start, end)
    }
}
