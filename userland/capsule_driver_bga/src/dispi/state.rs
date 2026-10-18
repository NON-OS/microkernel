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

//! The DISPI state as the adapter holds it: read whole at claim, so the
//! firmware's scanout (OVMF's GOP mode) can be kept or written back
//! (`restore`). Only what a driver may write is held; VIRT_HEIGHT is the
//! adapter's own sum of video memory over the line length and takes no
//! write.

use super::dispi_off::dispi_off;
use crate::constants::{
    DISPI_INDEX_BANK, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH,
    DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET,
};
use crate::regs::Regs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dispi {
    pub xres: u16,
    pub yres: u16,
    pub bpp: u16,
    pub enable: u16,
    pub bank: u16,
    pub virt_width: u16,
    pub x_offset: u16,
    pub y_offset: u16,
}

pub fn read(regs: Regs) -> Dispi {
    // SAFETY: every index read lies below `regs_len`, which the caller has
    // checked the register mapping covers.
    let r = |index| unsafe { regs.r16(dispi_off(index)) };
    Dispi {
        xres: r(DISPI_INDEX_XRES),
        yres: r(DISPI_INDEX_YRES),
        bpp: r(DISPI_INDEX_BPP),
        enable: r(DISPI_INDEX_ENABLE),
        bank: r(DISPI_INDEX_BANK),
        virt_width: r(DISPI_INDEX_VIRT_WIDTH),
        x_offset: r(DISPI_INDEX_X_OFFSET),
        y_offset: r(DISPI_INDEX_Y_OFFSET),
    }
}
