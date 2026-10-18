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

//! Reads of the legacy window. A memory window's read is followed by libc's
//! read barrier, as libc's own accessors are: nothing the register vouches
//! for in DMA memory is read before it.

use super::io::RegIo;
use super::pio;
use super::state::Regs;

impl Regs {
    pub fn r8(self, offset: usize) -> u8 {
        match self.io {
            RegIo::Mmio(m) => {
                let value = m.r8(offset);
                nonos_libc::dma_rmb();
                value
            }
            RegIo::Pio(grant) => pio::read(grant, offset, 1) as u8,
        }
    }

    pub fn r16(self, offset: usize) -> u16 {
        match self.io {
            RegIo::Mmio(m) => {
                let value = m.r16(offset);
                nonos_libc::dma_rmb();
                value
            }
            RegIo::Pio(grant) => pio::read(grant, offset, 2) as u16,
        }
    }

    pub fn r32(self, offset: usize) -> u32 {
        match self.io {
            RegIo::Mmio(m) => {
                let value = m.r32(offset);
                nonos_libc::dma_rmb();
                value
            }
            RegIo::Pio(grant) => pio::read(grant, offset, 4),
        }
    }
}
