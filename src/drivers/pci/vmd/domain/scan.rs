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

//! One bus of the walk: every function on it, and the command register
//! helpers the endpoint and bridge steps share.

use super::port::ConfigPort;
use super::regs::{REG_COMMAND, REG_HEADER, REG_ID};
use super::walk::{present, Walk};

impl<C: ConfigPort> Walk<'_, C> {
    pub(super) fn bus(&mut self, bus: u8, depth: u8) {
        // Bounded by the bus range; the depth check is belt and braces
        // against a device that answers on every bus number.
        if depth > 32 {
            return;
        }
        for device in 0..32u8 {
            if !present(self.cfg.read32(bus, device, 0, REG_ID)) {
                continue;
            }
            let header = (self.cfg.read32(bus, device, 0, REG_HEADER) >> 16) & 0xFF;
            let functions = if header & 0x80 != 0 { 8 } else { 1 };
            for function in 0..functions {
                if !present(self.cfg.read32(bus, device, function, REG_ID)) {
                    continue;
                }
                let kind = (self.cfg.read32(bus, device, function, REG_HEADER) >> 16) & 0x7F;
                match kind {
                    0 => self.endpoint(bus, device, function),
                    1 => self.bridge(bus, device, function, depth),
                    _ => {}
                }
            }
        }
    }

    pub(super) fn command(&mut self, bus: u8, device: u8, function: u8) -> u32 {
        self.cfg.read32(bus, device, function, REG_COMMAND) & 0xFFFF
    }

    /// The upper half of the command dword is status, write-one-to-clear,
    /// so writing it as zero leaves it alone.
    pub(super) fn set_command(&mut self, bus: u8, device: u8, function: u8, value: u32) {
        self.cfg.write32(bus, device, function, REG_COMMAND, value & 0xFFFF);
    }
}
