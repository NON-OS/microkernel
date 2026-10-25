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
//! A port's clock: the select written before the clock is let on, as
//! two writes (Vol 12 p. 178), and every other bit kept.

use crate::model::regs::Regs;
use nonos_intel_xelp_display::pll::map::{map, unmap};
use nonos_intel_xelp_display::regs::clka::Combo;
use nonos_intel_xelp_display::regs::pll::Dpll;

const CLKA: u32 = 0x164280;

#[test]
fn ddic_from_dpll4() {
    let mut m = Regs::new(Some(0));
    // Every clock off, as at reset, and DDIA already on DPLL1.
    m.set(CLKA, (1 << 24) | (1 << 11) | (1 << 10) | 0x1);
    map(&mut m, Combo::C, Dpll::Dpll4);
    let after_select = (1 << 24) | (1 << 11) | (1 << 10) | (0b10 << 4) | 0x1;
    assert_eq!(m.writes, [(CLKA, after_select), (CLKA, after_select & !(1 << 24))]);
}

#[test]
fn ddia_from_dpll0_and_back() {
    let mut m = Regs::new(Some(0));
    m.set(CLKA, 0x0100_0C33);
    map(&mut m, Combo::A, Dpll::Dpll0);
    assert_eq!(m.get(CLKA), 0x0100_0830);
    unmap(&mut m, Combo::A);
    assert_eq!(m.get(CLKA), 0x0100_0C30);
}

#[test]
fn ddib_from_dpll1() {
    let mut m = Regs::new(Some(0));
    // DDIA already on DPLL1, its clock running.
    m.set(CLKA, (1 << 11) | 0b01);
    map(&mut m, Combo::B, Dpll::Dpll1);
    assert_eq!(m.get(CLKA), (0b01 << 2) | 0b01);
    // Off again, its select and DDIA's kept.
    unmap(&mut m, Combo::B);
    assert_eq!(m.get(CLKA), (1 << 11) | (0b01 << 2) | 0b01);
}
