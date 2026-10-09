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

//! Linux e1000_flush_rx_ring: stop the receiver, set the receive descriptor
//! thresholds to prefetch 31 and host 1 counted in descriptors, and run the
//! receiver for a moment so the part takes the change and lets go.

use crate::constants::regs::{REG_RCTL, REG_RXDCTL, REG_STATUS};
use crate::constants::rxtx::{RCTL_EN, RXDCTL_THRESH_UNIT_DESC};
use crate::constants::timeouts::SHORT_MS;
use crate::regs::Regs;
use crate::wait::sleep_ms;

/// The low 14 bits of RXDCTL hold the prefetch and host thresholds.
const THRESH_BITS: u32 = 0x3FFF;
/// Prefetch threshold 31, host threshold 1 (bit 8).
const THRESH_FLUSH: u32 = 0x1F | (1 << 8);

pub fn run(regs: &Regs) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; RCTL, RXDCTL and
    // STATUS are 4-byte registers inside it.
    let rctl = unsafe { regs.r32(REG_RCTL) };
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_RCTL, rctl & !RCTL_EN);
        let _ = regs.r32(REG_STATUS);
    }
    sleep_ms(SHORT_MS);
    // SAFETY: as above.
    unsafe {
        regs.modify(REG_RXDCTL, THRESH_BITS, THRESH_FLUSH | RXDCTL_THRESH_UNIT_DESC);
        regs.w32(REG_RCTL, rctl | RCTL_EN);
        let _ = regs.r32(REG_STATUS);
    }
    sleep_ms(SHORT_MS);
    // SAFETY: as above.
    unsafe { regs.w32(REG_RCTL, rctl & !RCTL_EN) };
}
