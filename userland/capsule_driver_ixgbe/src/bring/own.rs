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

//! A driver owns the port: no-snoop disabled, so every DMA is snooped and
//! no cache flush is ever needed, and CTRL_EXT.DRV_LOAD set, which tells
//! manageability firmware to leave the port alone.

use crate::facts::mac::{CTRL_EXT_DRV_LOAD, CTRL_EXT_NS_DIS};
use crate::facts::regs::REG_CTRL_EXT;
use crate::regs::Regs;

pub fn take(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and
    // CTRL_EXT is a 32-bit register inside it (facts::regs).
    unsafe {
        let ext = regs.r32(REG_CTRL_EXT);
        regs.w32(REG_CTRL_EXT, ext | CTRL_EXT_NS_DIS | CTRL_EXT_DRV_LOAD);
    }
}
