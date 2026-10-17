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
//! A write to the device configuration: the one a driver makes is the
//! events-clear word, after reading which events the device raised.

use super::io::RegIo;
use super::pio;
use super::state::Regs;
use core::ptr::write_volatile;

impl Regs {
    /// # Safety
    ///
    /// The offset lies inside the mapped region; see the type.
    #[inline]
    pub unsafe fn config_w32(self, offset: usize, value: u32) {
        match self.device {
            RegIo::Mmio(base) => {
                write_volatile(base.add(self.device_offset + offset).cast(), value)
            }
            RegIo::Pio(grant) => pio::write(grant, self.device_offset + offset, 4, value),
        }
    }
}
