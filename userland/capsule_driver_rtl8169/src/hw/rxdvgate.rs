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

use super::regs::{MISC_RXDV_GATED_EN, REG_MISC};
use super::{hold_ms, wait_txrx_fifo_empty};
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux rtl_enable_rxdvgate: gate RX data off, give the MAC 2 ms, then
/// wait for both FIFOs. Only called on VER_40 and later, which have MISC.
pub fn enable_rxdvgate(regs: &Regs, ver: MacVersion) -> Result<(), &'static str> {
    // SAFETY: MISC (0xF0) lies inside every mapped window.
    unsafe { regs.w32(REG_MISC, regs.r32(REG_MISC) | MISC_RXDV_GATED_EN) };
    hold_ms(2);
    wait_txrx_fifo_empty(regs, ver)
}

/// Linux rtl_disable_rxdvgate. Until this runs on an 8168g or later, or an
/// 8125, no received frame reaches the ring.
pub fn disable_rxdvgate(regs: &Regs) {
    // SAFETY: MISC (0xF0) lies inside every mapped window.
    unsafe { regs.w32(REG_MISC, regs.r32(REG_MISC) & !MISC_RXDV_GATED_EN) };
}
