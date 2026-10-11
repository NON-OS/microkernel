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

//! The receiver on with IPv6 checksums, the LEDs as the board says, link
//! attention from the MI block, and one frame of low watermark, as
//! tg3_reset_hw does before the PHY. Which frames it keeps is filters.rs.

use crate::constants::literals::LOW_WMARK_57765;
use crate::constants::mac::*;
use crate::constants::mi::{MAC_MI_STAT, MAC_MI_STAT_LNKSTAT_ATTN_ENAB};
use crate::setup::Driver;

use super::blocks::settle;

/*
 * SAFETY: `d.regs` carries the broker MmioMap base for BAR0 and
 * every offset is a 32-bit MAC register inside it.
 */
pub fn enable(d: &mut Driver) {
    let regs = d.regs;
    d.modes.rx_mode = RX_MODE_ENABLE | RX_MODE_IPV6_CSUM_ENABLE;
    let rx_mode = d.modes.rx_mode;
    unsafe {
        settle(|| regs.w32f(MAC_RX_MODE, rx_mode));
        regs.w32(MAC_LED_CTRL, d.chip.hw.led_ctrl);
        regs.w32(MAC_MI_STAT, MAC_MI_STAT_LNKSTAT_ATTN_ENAB);
        settle(|| regs.w32f(MAC_RX_MODE, rx_mode));
        regs.w32f(MAC_LOW_WMARK_MAX_RX_FRAME, LOW_WMARK_57765);
    }
}
