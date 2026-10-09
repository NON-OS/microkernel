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

//! The voltages, bus width, DMA and slot type the Capabilities offer.

use super::super::regs::{
    CAP_180, CAP_300, CAP_330, CAP_64BIT_V3, CAP_8BIT, CAP_ADMA2, CAP_HIGH_SPEED,
    CAP_SLOT_TYPE_SHIFT,
};
use super::Caps;

impl Caps {
    pub const fn v330(&self) -> bool {
        self.caps & CAP_330 != 0
    }

    pub const fn v300(&self) -> bool {
        self.caps & CAP_300 != 0
    }

    pub const fn v180(&self) -> bool {
        self.caps & CAP_180 != 0
    }

    pub const fn bus8(&self) -> bool {
        self.caps & CAP_8BIT != 0
    }

    pub const fn adma2(&self) -> bool {
        self.caps & CAP_ADMA2 != 0
    }

    pub const fn high_speed(&self) -> bool {
        self.caps & CAP_HIGH_SPEED != 0
    }

    /// 64-bit system addressing in version 3 mode (ADMA2 with 12-byte
    /// descriptors, DMA select 11b).
    pub const fn dma64(&self) -> bool {
        self.caps & CAP_64BIT_V3 != 0
    }

    /// Slot type, bits 31:30: 0 removable, 1 embedded, 2 shared bus.
    pub const fn slot_type(&self) -> u8 {
        (self.caps >> CAP_SLOT_TYPE_SHIFT) as u8 & 0x3
    }
}
