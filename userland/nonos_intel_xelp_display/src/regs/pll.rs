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
//! The combo PHY's PLLs: DPLL0, DPLL1 and DPLL4. The Thunderbolt PLL and
//! the Type-C PLLs are other units. Vol 12 says DPLL4 is absent on the Y
//! and U parts (p. 176), so the caller must not choose it there.

/// A combo PHY PLL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpll {
    Dpll0,
    Dpll1,
    Dpll4,
}

/// DPLL0_ENABLE, DPLL1_ENABLE and DPLL4_ENABLE: 46010h, 46014h and 46018h
/// (Vol 2c p. 655).
pub fn enable(d: Dpll) -> u32 {
    [0x4_6010, 0x4_6014, 0x4_6018][d as usize]
}

/// DPLLn_CFGCR0: 164284h, 16428Ch and 164294h (p. 650).
pub fn cfgcr0(d: Dpll) -> u32 {
    [0x16_4284, 0x16_428C, 0x16_4294][d as usize]
}

/// DPLLn_CFGCR1: 164288h, 164290h and 164298h (p. 651).
pub fn cfgcr1(d: Dpll) -> u32 {
    [0x16_4288, 0x16_4290, 0x16_4298][d as usize]
}

/// DPLLn_SSC: 164B10h, 164C10h and 164E10h (p. 658).
pub fn ssc(d: Dpll) -> u32 {
    [0x16_4B10, 0x16_4C10, 0x16_4E10][d as usize]
}

/// DPLL_ENABLE bits (p. 656): 31 PLL Enable, 30 PLL Lock, 27 Power Enable,
/// 26 Power State.
pub const PLL_ENABLE: u32 = 1 << 31;
pub const PLL_LOCK: u32 = 1 << 30;
pub const POWER_ENABLE: u32 = 1 << 27;
pub const POWER_STATE: u32 = 1 << 26;

/// DPLL_SSC bit 9, sscen, the spread spectrum modulator (p. 659).
pub const SSC_EN: u32 = 1 << 9;
