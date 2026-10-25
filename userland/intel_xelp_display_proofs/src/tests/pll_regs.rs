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
//! The PLL and port clock registers at the addresses Vol 2c lists, and
//! their bits.

use nonos_intel_xelp_display::regs::clka::{self, clock_off, select, select_value, Combo};
use nonos_intel_xelp_display::regs::pll::{self, Dpll};

const D: [Dpll; 3] = [Dpll::Dpll0, Dpll::Dpll1, Dpll::Dpll4];
const P: [Combo; 3] = [Combo::A, Combo::B, Combo::C];

#[test]
fn dpll_registers() {
    for (i, d) in D.iter().enumerate() {
        // Vol 2c p. 655, 650, 651, 658.
        assert_eq!(pll::enable(*d), [0x46010, 0x46014, 0x46018][i]);
        assert_eq!(pll::cfgcr0(*d), [0x164284, 0x16428C, 0x164294][i]);
        assert_eq!(pll::cfgcr1(*d), [0x164288, 0x164290, 0x164298][i]);
        assert_eq!(pll::ssc(*d), [0x164B10, 0x164C10, 0x164E10][i]);
    }
    // DPLL_ENABLE 31, 30, 27, 26 (p. 656); DPLL_SSC sscen 9 (p. 659).
    let bits = [pll::PLL_ENABLE, pll::PLL_LOCK, pll::POWER_ENABLE, pll::POWER_STATE];
    assert_eq!(bits, [1 << 31, 1 << 30, 1 << 27, 1 << 26]);
    assert_eq!(pll::SSC_EN, 1 << 9);
}

#[test]
fn port_clock_fields() {
    // DPCLKA_CFGCR0 at 164280h: Clock Off DDIA 10, DDIB 11, DDIC 24 (p. 608,
    // 609); Clock Select 1:0, 3:2, 5:4 (p. 610).
    assert_eq!(clka::DPCLKA_CFGCR0, 0x164280);
    for (i, p) in P.iter().enumerate() {
        assert_eq!(clock_off(*p), [1 << 10, 1 << 11, 1 << 24][i]);
        assert_eq!(select(*p), ([0x3, 0xC, 0x30][i], [0, 2, 4][i]));
    }
    // 00b DPLL0, 01b DPLL1, 10b DPLL4 (p. 610).
    assert_eq!(D.map(select_value), [0, 1, 2]);
}

#[test]
fn reference_from_the_strap() {
    // DSSM at 51004h, 31:29: 000b 24, 001b 19.2, 010b 38.4 MHz (p. 825).
    assert_eq!(clka::DSSM, 0x51004);
    assert_eq!(clka::reference_khz(0x0000_0040), Some(24_000));
    assert_eq!(clka::reference_khz(0x2000_0000), Some(19_200));
    assert_eq!(clka::reference_khz(0x5FFF_FFFF), Some(38_400));
    for v in 3..8u32 {
        assert_eq!(clka::reference_khz(v << 29), None);
    }
}
