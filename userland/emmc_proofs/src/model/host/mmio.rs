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

//! The register-level view the driver's `Mmio` sees.

use super::bus::SimBus;

impl crate::emmc::env::Mmio for SimBus {
    fn r8(&self, off: u32) -> u8 {
        self.0.borrow_mut().read(off, 1) as u8
    }
    fn r16(&self, off: u32) -> u16 {
        self.0.borrow_mut().read(off, 2) as u16
    }
    fn r32(&self, off: u32) -> u32 {
        self.0.borrow_mut().read(off, 4)
    }
    fn w8(&self, off: u32, v: u8) {
        self.0.borrow_mut().write(off, 1, v as u32)
    }
    fn w16(&self, off: u32, v: u16) {
        self.0.borrow_mut().write(off, 2, v as u32)
    }
    fn w32(&self, off: u32, v: u32) {
        self.0.borrow_mut().write(off, 4, v)
    }
}
