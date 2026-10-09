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

//! A bridge behind the VMD: given the next bus number, everything below it
//! walked, then its bus range and memory window narrowed to what was found.

use super::port::ConfigPort;
use super::regs::{CMD_IO, CMD_MASTER, CMD_MEM, REG_BUSES};
use super::walk::{align_up, Walk};
use super::window::MIB;

impl<C: ConfigPort> Walk<'_, C> {
    pub(super) fn bridge(&mut self, bus: u8, device: u8, function: u8, depth: u8) {
        if self.next_bus > self.last_bus as u16 {
            self.out.starved += 1;
            return;
        }
        self.out.bridges += 1;
        let secondary = self.next_bus as u8;
        self.next_bus += 1;
        let command = self.command(bus, device, function);
        self.set_command(bus, device, function, command & !(CMD_IO | CMD_MEM | CMD_MASTER));
        let latency = self.cfg.read32(bus, device, function, REG_BUSES) & 0xFF00_0000;
        let numbers = |sub: u8| latency | (sub as u32) << 16 | (secondary as u32) << 8 | bus as u32;
        // Everything up to the last bus is routed below while the walk is
        // under way, then narrowed to what it found.
        self.cfg.write32(bus, device, function, REG_BUSES, numbers(self.last_bus));

        let start = align_up(self.cursor, MIB).unwrap_or(u64::MAX);
        self.cursor = start.min(self.window.limit.saturating_add(1));
        self.bus(secondary, depth + 1);
        let subordinate = (self.next_bus - 1) as u8;
        self.cfg.write32(bus, device, function, REG_BUSES, numbers(subordinate));

        self.bridge_window(bus, device, function, start);
        self.set_command(bus, device, function, (command & !CMD_IO) | CMD_MEM | CMD_MASTER);
    }
}
