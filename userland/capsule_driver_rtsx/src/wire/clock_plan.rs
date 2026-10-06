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

//! The SSC settings for a card clock, computed as rtsx_pci_switch_clock
//! does for the chips without their own conversion (no conv_clk_and_div_n):
//! the RTS5227 family among them.

use crate::regs::clk::{CLK_DIV_1, CLK_DIV_8, MAX_DIV_N_PCR, MIN_DIV_N_PCR, SSC_DEPTH_4M};
use crate::regs::sd::{SD_CLK_DIVIDE_0, SD_CLK_DIVIDE_128};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockPlan {
    /// SD_CFG1's divider: 128 in the initial (identification) mode.
    pub divider: u8,
    /// The internal SSC clock in MHz.
    pub clk_mhz: u32,
    pub n: u8,
    pub div: u8,
    pub mcu_cnt: u8,
    pub ssc_depth: u8,
}

/// `ssc_depth` is the register value (SSC_DEPTH_500K and so on). `None`
/// when the clock is outside what the SSC can make, as Linux's -EINVAL.
pub fn plan(card_hz: u32, ssc_depth: u8, initial: bool, double: bool) -> Option<ClockPlan> {
    // Around 234 kHz in identification: 30 MHz through the 128 divider.
    let (divider, card_hz) =
        if initial { (SD_CLK_DIVIDE_128, 30_000_000) } else { (SD_CLK_DIVIDE_0, card_hz) };
    let card_mhz = card_hz / 1_000_000;
    let clk = if !initial && double { card_mhz * 2 } else { card_mhz };
    if clk <= 2 || clk - 2 > MAX_DIV_N_PCR as u32 {
        return None;
    }
    let mut n = clk - 2;
    let mcu_cnt = (125 / clk + 3).min(15) as u8;
    let mut div = CLK_DIV_1;
    while n < MIN_DIV_N_PCR as u32 && div < CLK_DIV_8 {
        n = (n + 2) * 2 - 2;
        div += 1;
    }
    let mut depth = ssc_depth;
    if double && depth > 1 {
        depth -= 1;
    }
    if div > CLK_DIV_1 {
        depth = if depth > div - 1 { depth - (div - 1) } else { SSC_DEPTH_4M };
    }
    Some(ClockPlan { divider, clk_mhz: clk, n: n as u8, div, mcu_cnt, ssc_depth: depth })
}
