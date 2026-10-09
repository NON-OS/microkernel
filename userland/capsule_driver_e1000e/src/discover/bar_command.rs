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

//! The PCI Command decode bits the device's BARs call for, from the record,
//! as capsule_driver_rtl8169 derives them: I/O Space for a port BAR, Memory
//! Space for an MMIO one.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO};

pub const CMD_IO_SPACE: u16 = 1 << 0;
pub const CMD_MEMORY_SPACE: u16 = 1 << 1;

pub fn command_bits(r: &DeviceRecord) -> u16 {
    let mut bits = 0u16;
    for bar in &r.bars[..core::cmp::min(r.bar_count as usize, r.bars.len())] {
        if bar.kind == BAR_KIND_PIO {
            bits |= CMD_IO_SPACE;
        } else if bar.kind == BAR_KIND_MMIO {
            bits |= CMD_MEMORY_SPACE;
        }
    }
    bits
}
