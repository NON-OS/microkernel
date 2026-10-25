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
//! The DPLL enable and disable sequences (Vol 12 p. 177, 178) on the
//! model: the order of the writes, the waits, and what failure leaves.

use crate::model::clock::Steps;
use crate::model::regs::Regs;
use nonos_intel_xelp_display::pll::dp::dividers;
use nonos_intel_xelp_display::pll::sequence::configure;
use nonos_intel_xelp_display::regs::pll::Dpll;

const EN1: u32 = 0x46014;
const SSC1: u32 = 0x164C10;
const CR0_1: u32 = 0x16428C;
const CR1_1: u32 = 0x164290;

#[test]
fn configure_in_vol_12_order() {
    let mut m = Regs::new(Some(1));
    m.set(SSC1, 0x8000_0000);
    let d = dividers(2700, 24_000).unwrap();
    configure(&mut m, &Steps::new(1), Dpll::Dpll1, &d, true).unwrap();
    let regs: Vec<u32> = m.writes.iter().map(|w| w.0).collect();
    // Power, then SSC, CFGCR0, CFGCR1; then CFGCR0 read back last.
    assert_eq!(regs, [EN1, SSC1, CR0_1, CR1_1]);
    assert_eq!(m.get(EN1), (1 << 27) | (1 << 26));
    assert_eq!(m.get(SSC1), 0x8000_0200);
    assert_eq!(m.get(CR0_1), (0x4000 << 10) | 0x151);
    assert_eq!(*m.reads.last().unwrap(), CR0_1);
}

#[test]
fn hdmi_turns_spread_off() {
    let mut m = Regs::new(Some(0));
    m.set(SSC1, 0x0000_0201);
    let d = dividers(1620, 19_200).unwrap();
    configure(&mut m, &Steps::new(1), Dpll::Dpll1, &d, false).unwrap();
    assert_eq!(m.get(SSC1), 0x0000_0001);
}

#[test]
fn enabled_pll_not_touched() {
    let mut m = Regs::new(Some(0));
    m.set(EN1, (1 << 31) | (1 << 30) | (1 << 27) | (1 << 26));
    let d = dividers(2700, 24_000).unwrap();
    assert!(configure(&mut m, &Steps::new(1), Dpll::Dpll1, &d, true).is_err());
    assert!(m.writes.is_empty());
}

#[test]
fn power_that_never_comes_is_asked_off() {
    let mut m = Regs::new(Some(0));
    m.stuck.insert((EN1, 26));
    let d = dividers(2700, 24_000).unwrap();
    assert!(configure(&mut m, &Steps::new(10), Dpll::Dpll1, &d, true).is_err());
    assert_eq!(m.get(EN1) & (1 << 27), 0);
    assert!(!m.writes.iter().any(|w| w.0 == CR0_1 || w.0 == CR1_1));
}
