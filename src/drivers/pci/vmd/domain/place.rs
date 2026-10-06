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

//! Placing one sized BAR at the next aligned address the window still has,
//! or leaving it unassigned when it does not fit.

use super::port::ConfigPort;
use super::walk::{align_up, Walk};
use super::window::FOUR_GIB;

impl<C: ConfigPort> Walk<'_, C> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn place(
        &mut self,
        bus: u8,
        device: u8,
        function: u8,
        reg: u16,
        original: u32,
        wide: bool,
        size: u64,
    ) {
        let flags = original & 0xF;
        let addr = align_up(self.cursor, size.max(0x1000));
        let fits = match addr {
            Some(a) => {
                a.checked_add(size - 1).is_some_and(|end| end <= self.window.limit)
                    && (wide || a + size <= FOUR_GIB)
            }
            None => false,
        };
        match (fits, addr) {
            (true, Some(a)) => {
                self.cfg.write32(bus, device, function, reg, (a as u32 & !0xF) | flags);
                if wide {
                    self.cfg.write32(bus, device, function, reg + 4, (a >> 32) as u32);
                }
                self.cursor = a + size;
                self.out.bars += 1;
            }
            _ => {
                // Left at zero with decode off is safe: it claims no address.
                self.cfg.write32(bus, device, function, reg, flags);
                if wide {
                    self.cfg.write32(bus, device, function, reg + 4, 0);
                }
                self.out.starved += 1;
            }
        }
    }
}
