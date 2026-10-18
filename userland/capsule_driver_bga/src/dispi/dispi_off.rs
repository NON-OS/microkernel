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

use crate::constants::{DISPI_IOPORT_OFFSET, VGA_MMIO_BASE, VGA_PORT_BASE};

pub fn dispi_off(index: u32) -> u32 {
    DISPI_IOPORT_OFFSET + index * 2
}

/// Where VGA port `port` (0x3C0 and up) sits in the register BAR.
pub fn vga_off(port: u16) -> u32 {
    VGA_MMIO_BASE + (port - VGA_PORT_BASE) as u32
}
