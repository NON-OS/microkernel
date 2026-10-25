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
//! DPCLKA_CFGCR0, which maps a DPLL to a combo port and gates its clock,
//! and DSSM, whose strap gives the reference clock.

use super::pll::Dpll;

/// DPCLKA_CFGCR0, 164280h (Vol 2c p. 608).
pub const DPCLKA_CFGCR0: u32 = 0x16_4280;

/// A combo PHY port: DDI A, B or C. Type-C ports are another unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combo {
    A,
    B,
    C,
}

/// The port's Clock Off bit: DDIA 10, DDIB 11, DDIC 24 (p. 608, 609).
pub fn clock_off(p: Combo) -> u32 {
    1 << [10, 11, 24][p as usize]
}

/// The port's two-bit Clock Select field: DDIA 1:0, DDIB 3:2, DDIC 5:4
/// (p. 610), as its mask and its shift.
pub fn select(p: Combo) -> (u32, u32) {
    let shift = [0, 2, 4][p as usize];
    (3 << shift, shift)
}

/// The Clock Select value of a DPLL: 00b DPLL0, 01b DPLL1, 10b DPLL4
/// (p. 610).
pub fn select_value(d: Dpll) -> u32 {
    [0, 1, 2][d as usize]
}

/// DSSM, the display strap state, 51004h (p. 825).
pub const DSSM: u32 = 0x5_1004;

/// DSSM 31:29, Reference Frequency, in kHz: 000b 24 MHz, 001b 19.2 MHz,
/// 010b 38.4 MHz; any other value is not one Vol 2c lists (p. 825).
pub fn reference_khz(dssm: u32) -> Option<u32> {
    match dssm >> 29 {
        0 => Some(24_000),
        1 => Some(19_200),
        2 => Some(38_400),
        _ => None,
    }
}
