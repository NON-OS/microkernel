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

use crate::arch::fdt::endian::be_cells;

/// `(address, size)` pairs. Cell counts past two cannot be read into a u64,
/// and an entry of zero cells would never advance; either yields nothing, so
/// a hostile `#address-cells` costs a missing device, not a hang.
pub struct RegIter<'a> {
    data: &'a [u8],
    offset: usize,
    address_cells: u32,
    size_cells: u32,
}

impl<'a> RegIter<'a> {
    pub fn new(data: &'a [u8], address_cells: u32, size_cells: u32) -> Self {
        Self { data, offset: 0, address_cells, size_cells }
    }

    /// True when the cell counts make a readable entry.
    pub fn readable(&self) -> bool {
        self.address_cells <= 2 && self.size_cells <= 2 && self.address_cells != 0
    }
}

impl<'a> Iterator for RegIter<'a> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.readable() {
            return None;
        }
        let size_off = self.offset + self.address_cells as usize * 4;
        let end = size_off + self.size_cells as usize * 4;
        if end > self.data.len() {
            return None;
        }
        let addr = be_cells(self.data, self.offset, self.address_cells).ok()?;
        let size = be_cells(self.data, size_off, self.size_cells).ok()?;
        self.offset = end;
        Some((addr, size))
    }
}
