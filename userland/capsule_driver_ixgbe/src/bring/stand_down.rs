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

//! What a failed attempt leaves the part doing before its grants go back:
//! nothing. A step that fails after a queue was enabled would otherwise leave
//! it able to DMA into ring memory the broker is about to hand to someone
//! else. Both queues are disabled, both engines stopped, every cause masked,
//! and bus mastering asked to stop.

use crate::bring::quiet::mask_all;
use crate::facts::mac::{CTRL_GIO_DIS, DMATXCTL_TE, RXCTRL_RXEN};
use crate::facts::regs::REG_TXDCTL;
use crate::facts::regs::{REG_CTRL, REG_DMATXCTL, REG_RXCTRL, REG_RXDCTL, REG_STATUS};
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (facts::regs).
    unsafe {
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_TXDCTL, 0);
        let rxctrl = regs.r32(REG_RXCTRL);
        regs.w32(REG_RXCTRL, rxctrl & !RXCTRL_RXEN);
        let dmatxctl = regs.r32(REG_DMATXCTL);
        regs.w32(REG_DMATXCTL, dmatxctl & !DMATXCTL_TE);
        mask_all(regs);
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_DIS);
        let _ = regs.r32(REG_STATUS);
    }
}
