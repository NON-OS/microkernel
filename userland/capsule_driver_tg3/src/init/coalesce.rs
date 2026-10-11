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

//! Host coalescing stopped, given tg3_init_coal's values for one vector
//! without RSS or TSS, and started again: when the part writes the status
//! block this driver polls (tg3_reset_hw, __tg3_set_coalesce).

use crate::constants::hostcc::*;
use crate::constants::timeouts::BLOCK_MODE_MS;
use crate::regs::Regs;

use super::wait;

/// tp->coalesce_mode on every part but the 5700 AX and BX.
pub const COALESCE_MODE: u32 = HOSTCC_MODE_32BYTE;

/// The other vectors' registers, each zeroed: (tick, frames, frames-irq)
/// for transmit, then for receive.
const VEC_REGS: [usize; 6] = [
    HOSTCC_TXCOL_TICKS_VEC1,
    HOSTCC_TXMAX_FRAMES_VEC1,
    HOSTCC_TXCOAL_MAXF_INT_VEC1,
    HOSTCC_RXCOL_TICKS_VEC1,
    HOSTCC_RXMAX_FRAMES_VEC1,
    HOSTCC_RXCOAL_MAXF_INT_VEC1,
];

/*
 * SAFETY for every access: `regs` carries the broker MmioMap base for BAR0
 * and each offset is a 32-bit host coalescing register.
 */
pub fn program(regs: &Regs) {
    unsafe { regs.w32(HOSTCC_MODE, 0) };
    // Linux goes on whether or not the block stopped in time.
    let _ =
        wait::until(BLOCK_MODE_MS, || unsafe { regs.r32(HOSTCC_MODE) & HOSTCC_MODE_ENABLE == 0 });
    unsafe {
        regs.w32(HOSTCC_TXCOL_TICKS, LOW_TXCOL_TICKS);
        regs.w32(HOSTCC_TXMAX_FRAMES, LOW_TXMAX_FRAMES);
        regs.w32(HOSTCC_TXCOAL_MAXF_INT, DEFAULT_TXCOAL_MAXF_INT);
        regs.w32(HOSTCC_RXCOL_TICKS, LOW_RXCOL_TICKS);
        regs.w32(HOSTCC_RXMAX_FRAMES, LOW_RXMAX_FRAMES);
        regs.w32(HOSTCC_RXCOAL_MAXF_INT, DEFAULT_RXCOAL_MAXF_INT);
        for n in 0..IRQ_MAX_VECS - 1 {
            for reg in VEC_REGS {
                regs.w32(reg + n * VEC_STRIDE, 0);
            }
        }
        regs.w32(HOSTCC_MODE, HOSTCC_MODE_ENABLE | COALESCE_MODE);
    }
}

/// tg3_timer without tagged status: ask for a status block now, so a frame
/// that arrived under the coalescing threshold is seen within a tick.
pub fn now(regs: &Regs) {
    // SAFETY: as above.
    unsafe { regs.w32(HOSTCC_MODE, COALESCE_MODE | HOSTCC_MODE_ENABLE | HOSTCC_MODE_NOW) };
}
