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

//! Skylake's shared DPLLs 1 to 3 (DPLL 0 drives CDCLK and eDP from boot
//! and is left to firmware): DPLL_CTRL1 picks a DisplayPort link rate or
//! HDMI mode per DPLL, DPLL_CTRL2 routes a DDI's clock to a DPLL, and
//! DPLL_STATUS reports lock (SKL PRM Vol 2c, DPLL_CTRL1, DPLL_CTRL2,
//! DPLL_STATUS, LCPLL2_CTL, WRPLL_CTL1, WRPLL_CTL2). Field layouts below
//! are unverified against the volume where marked.

pub const DPLL_CTRL1: u32 = 0x6_C058;
pub const DPLL_CTRL2: u32 = 0x6_C05C;
pub const DPLL_STATUS: u32 = 0x6_C060;

/// Each DPLL's enable register, DPLL 1 to 3; bit 31 enables.
pub fn enable_reg(dpll: u32) -> u32 {
    match dpll {
        1 => 0x4_6014,
        2 => 0x4_6040,
        _ => 0x4_6060,
    }
}

/// DPLL_CFGCR1 and DPLL_CFGCR2 of DPLL 1 to 3, used in HDMI mode.
pub fn cfgcr1(dpll: u32) -> u32 {
    0x6_C040 + 8 * (dpll - 1)
}
pub fn cfgcr2(dpll: u32) -> u32 {
    cfgcr1(dpll) + 4
}
