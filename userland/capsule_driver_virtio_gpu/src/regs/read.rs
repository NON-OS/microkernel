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
use super::io::RegIo;
use super::pio;
use super::state::Regs;
impl Regs {
    /// # Safety
    ///
    /// The offset lies inside the mapped region; see the type.
    #[inline]
    pub unsafe fn r8(self, offset: usize) -> u8 {
        match self.common {
            RegIo::Mmio(base) => {
                nonos_libc::mmio_read8(base.add(self.common_offset + offset) as usize)
            }
            RegIo::Pio(grant) => pio::read(grant, self.common_offset + offset, 1) as u8,
        }
    }
    /// # Safety
    ///
    /// The offset lies inside the mapped region; see the type.
    #[inline]
    pub unsafe fn r16(self, offset: usize) -> u16 {
        match self.common {
            RegIo::Mmio(base) => {
                nonos_libc::mmio_read16(base.add(self.common_offset + offset) as usize)
            }
            RegIo::Pio(grant) => pio::read(grant, self.common_offset + offset, 2) as u16,
        }
    }
    /// # Safety
    ///
    /// The offset lies inside the mapped region; see the type.
    #[inline]
    pub unsafe fn r32(self, offset: usize) -> u32 {
        match self.common {
            RegIo::Mmio(base) => {
                nonos_libc::mmio_read32(base.add(self.common_offset + offset) as usize)
            }
            RegIo::Pio(grant) => pio::read(grant, self.common_offset + offset, 4),
        }
    }
    /// # Safety
    ///
    /// The offset lies inside the mapped region; see the type.
    #[inline]
    pub unsafe fn config_r32(self, offset: usize) -> u32 {
        match self.device {
            RegIo::Mmio(base) => {
                nonos_libc::mmio_read32(base.add(self.device_offset + offset) as usize)
            }
            RegIo::Pio(grant) => pio::read(grant, self.device_offset + offset, 4),
        }
    }
}
