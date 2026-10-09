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

//! An endpoint behind the VMD: decode off, every memory BAR sized and
//! placed, then memory decode back on with mastering left to its driver.

use super::port::ConfigPort;
use super::regs::{CMD_IO, CMD_MASTER, CMD_MEM, REG_BAR0};
use super::walk::Walk;

impl<C: ConfigPort> Walk<'_, C> {
    pub(super) fn endpoint(&mut self, bus: u8, device: u8, function: u8) {
        self.out.endpoints += 1;
        let command = self.command(bus, device, function);
        self.set_command(bus, device, function, command & !(CMD_IO | CMD_MEM | CMD_MASTER));
        let mut index = 0u16;
        while index < 6 {
            let reg = REG_BAR0 + index * 4;
            let original = self.cfg.read32(bus, device, function, reg);
            if original & 1 != 0 {
                // No I/O space behind a VMD.
                index += 1;
                continue;
            }
            let wide = (original >> 1) & 0x3 == 2 && index < 5;
            self.cfg.write32(bus, device, function, reg, 0xFFFF_FFFF);
            let low = self.cfg.read32(bus, device, function, reg) & !0xF;
            let high = if wide {
                self.cfg.write32(bus, device, function, reg + 4, 0xFFFF_FFFF);
                self.cfg.read32(bus, device, function, reg + 4)
            } else {
                0xFFFF_FFFF
            };
            if low == 0 {
                self.cfg.write32(bus, device, function, reg, original);
                index += if wide { 2 } else { 1 };
                continue;
            }
            let mask = ((high as u64) << 32) | low as u64;
            let size = (!mask).wrapping_add(1);
            self.place(bus, device, function, reg, original, wide, size);
            index += if wide { 2 } else { 1 };
        }
        self.set_command(bus, device, function, (command & !(CMD_IO | CMD_MASTER)) | CMD_MEM);
    }
}
