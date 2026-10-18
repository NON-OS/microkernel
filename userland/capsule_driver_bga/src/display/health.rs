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

//! STATE's look at the adapter (G 12, the fault path): DISPI is still
//! showing this driver's mode, enabled, linear, 32 bits, at its size. Anything
//! else (firmware or a VGA reset turning DISPI off, another writer to its
//! registers) is a lost output: the mode is set again before the answer and
//! the generation moves, so the compositor repaints all of it; a mode that
//! will not come back answers lost without scanning out.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::bga::Bga;
use crate::dispi::{current, set_mode};

impl Bga {
    /// STATE's flags, after putting the mode back when it was lost.
    pub fn health(&mut self) -> u32 {
        let size = |m: Option<crate::dispi::DispiMode>| m.map(|m| (m.width, m.height));
        let want = (self.mode.width, self.mode.height);
        if size(current(self.regs)) == Some(want) {
            return SCANNING_OUT;
        }
        self.generation = self.generation.wrapping_add(1);
        set_mode(self.regs, want.0 as u16, want.1 as u16);
        match current(self.regs) {
            // The virtual width may differ from firmware's: copies follow it.
            Some(m) if (m.width, m.height) == want => {
                self.mode = m;
                SCANNING_OUT | LOST
            }
            _ => LOST,
        }
    }
}
