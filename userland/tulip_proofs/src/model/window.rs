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

//! The longword accesses the driver makes, decoded: CSR `n` at `n * 8`.
//! Anything else is a driver reaching outside the CSR file.

use super::part::Part;

impl Part {
    pub fn read32(&mut self, offset: usize) -> u32 {
        assert!(offset & 7 == 0 && offset < 0x80, "read at window offset {offset:#x}");
        match offset / 8 {
            5 => self.csr5(),
            n => self.csr[n],
        }
    }

    pub fn write32(&mut self, offset: usize, v: u32) {
        assert!(offset & 7 == 0 && offset < 0x80, "write at window offset {offset:#x}");
        self.write_csr(offset / 8, v);
    }
}
