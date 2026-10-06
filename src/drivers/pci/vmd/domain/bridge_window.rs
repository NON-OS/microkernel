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

//! A bridge's forwarding windows once its children are placed: I/O closed,
//! memory opened over what sits below it through the 32-bit or the 64-bit
//! prefetchable window, or closed when nothing does.

use super::port::ConfigPort;
use super::regs::{
    CLOSED_IO, CLOSED_WINDOW, REG_IO_HI, REG_IO_WINDOW, REG_MEM_WINDOW, REG_PREF_BASE_HI,
    REG_PREF_LIMIT_HI, REG_PREF_WINDOW,
};
use super::walk::{align_up, Walk};
use super::window::MIB;

impl<C: ConfigPort> Walk<'_, C> {
    pub(super) fn bridge_window(&mut self, bus: u8, device: u8, function: u8, start: u64) {
        self.cfg.write32(bus, device, function, REG_IO_WINDOW, CLOSED_IO);
        self.cfg.write32(bus, device, function, REG_IO_HI, 0);
        let used = self.cursor > start;
        let limit = if used { align_up(self.cursor, MIB).map(|end| end - 1) } else { None };
        match limit {
            Some(limit) if !self.window.prefetch64 => {
                let value =
                    ((start >> 16) as u32 & 0xFFF0) | (((limit >> 16) as u32 & 0xFFF0) << 16);
                self.cfg.write32(bus, device, function, REG_MEM_WINDOW, value);
                self.close_prefetch(bus, device, function);
                self.cursor = limit + 1;
            }
            Some(limit) => {
                self.cfg.write32(bus, device, function, REG_MEM_WINDOW, CLOSED_WINDOW);
                let value = ((start >> 16) as u32 & 0xFFF0)
                    | 1
                    | ((((limit >> 16) as u32) & 0xFFF0) << 16)
                    | (1 << 16);
                self.cfg.write32(bus, device, function, REG_PREF_WINDOW, value);
                self.cfg.write32(bus, device, function, REG_PREF_BASE_HI, (start >> 32) as u32);
                self.cfg.write32(bus, device, function, REG_PREF_LIMIT_HI, (limit >> 32) as u32);
                self.cursor = limit + 1;
            }
            None => {
                self.cfg.write32(bus, device, function, REG_MEM_WINDOW, CLOSED_WINDOW);
                self.close_prefetch(bus, device, function);
            }
        }
    }

    fn close_prefetch(&mut self, bus: u8, device: u8, function: u8) {
        self.cfg.write32(bus, device, function, REG_PREF_WINDOW, CLOSED_WINDOW);
        self.cfg.write32(bus, device, function, REG_PREF_BASE_HI, 0);
        self.cfg.write32(bus, device, function, REG_PREF_LIMIT_HI, 0);
    }
}
