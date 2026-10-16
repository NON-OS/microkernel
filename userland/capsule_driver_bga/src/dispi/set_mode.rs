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

use super::dispi_off::dispi_off;
use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_XRES,
    DISPI_INDEX_YRES, DISPI_LFB_ENABLED,
};
use crate::regs::Regs;

/// How far into the register mapping the mode set writes: through ENABLE,
/// the highest DISPI index it touches.
pub fn regs_len() -> u64 {
    dispi_off(DISPI_INDEX_ENABLE) as u64 + 2
}

/// Program the mode with the adapter disabled, ENABLE written last.
pub fn set_mode(regs: Regs, width: u16, height: u16) {
    // SAFETY: every index written lies below `regs_len`, which the caller
    // has checked the register mapping covers.
    unsafe {
        regs.w16(dispi_off(DISPI_INDEX_ENABLE), 0);
        regs.w16(dispi_off(DISPI_INDEX_XRES), width);
        regs.w16(dispi_off(DISPI_INDEX_YRES), height);
        regs.w16(dispi_off(DISPI_INDEX_BPP), DISPI_BPP_32);
        regs.w16(dispi_off(DISPI_INDEX_ENABLE), DISPI_ENABLED | DISPI_LFB_ENABLED);
    }
}
