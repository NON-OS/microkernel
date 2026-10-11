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

//! DPLL_CTRL1, DPLL_CTRL2 and DPLL_STATUS fields.

use crate::engine::dpll::{cfgcr1, cfgcr2, enable_reg};
use crate::engine::dpll_ctrl::{ctrl1_bits, ctrl2_bits, link_rate_code, locked};

#[test]
fn link_rate_codes_cover_the_dp_rates_skylake_runs() {
    assert_eq!(link_rate_code(1620), Some(0b010));
    assert_eq!(link_rate_code(2700), Some(0b001));
    assert_eq!(link_rate_code(5400), Some(0b000));
    assert_eq!(link_rate_code(8100), None, "HBR3 is not a Skylake DDI rate");
}

#[test]
fn each_dpll_owns_its_six_bits_of_ctrl1() {
    assert_eq!(ctrl1_bits(1, Some(2700)), Some((0x3F << 6, 0b011 << 6)));
    assert_eq!(ctrl1_bits(3, None), Some((0x3F << 18, 0b10_0001 << 18)), "HDMI mode");
    assert_eq!(ctrl1_bits(2, Some(1234)), None);
}

#[test]
fn a_ddi_is_routed_to_a_dpll_with_its_clock_on() {
    let (mask, bits) = ctrl2_bits(1, 2);
    assert_eq!(mask, (7 << 3) | (1 << 16));
    assert_eq!(bits, 0b101 << 3, "override set, DPLL 2, clock-off clear");
}

#[test]
fn registers_and_lock_bits_per_dpll() {
    assert_eq!((enable_reg(1), enable_reg(2), enable_reg(3)), (0x4_6014, 0x4_6040, 0x4_6060));
    assert_eq!(
        (cfgcr1(1), cfgcr2(1), cfgcr1(3), cfgcr2(3)),
        (0x6_C040, 0x6_C044, 0x6_C050, 0x6_C054)
    );
    assert!(locked(1 << 16, 2) && !locked(1 << 16, 1) && locked(1, 0));
}
