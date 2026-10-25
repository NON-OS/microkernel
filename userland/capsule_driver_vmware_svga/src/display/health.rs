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

//! STATE's look at the adapter: still enabled, 32 bits, at this driver's
//! size. Anything else (a guest VGA reset, another writer) is a lost
//! output: the mode is set again before the answer and the generation
//! moves, so the compositor repaints all of it; a mode that will not come
//! back answers lost without scanning out.

use super::svga::Svga;
use crate::constants::mode::BITS_PER_PIXEL as BPP;
use crate::constants::regs::{BITS_PER_PIXEL, ENABLE, HEIGHT, WIDTH};
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

impl Svga {
    /// STATE's flags, after putting the mode back when it was lost.
    pub fn health(&mut self) -> u32 {
        let want = (self.mode.width, self.mode.height);
        let r = |reg| self.regs.read(reg);
        if r(ENABLE) != 0 && r(BITS_PER_PIXEL) == BPP && (r(WIDTH), r(HEIGHT)) == want {
            return SCANNING_OUT;
        }
        self.generation = self.generation.wrapping_add(1);
        match self.program(want.0, want.1) {
            Ok(()) => SCANNING_OUT | LOST,
            Err(_) => LOST,
        }
    }
}
