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
//! A configured DPLL locked and stopped (Vol 12 p. 177, 178).

use crate::model::clock::Steps;
use crate::model::regs::Regs;
use nonos_intel_xelp_display::pll::lock::{disable, lock};
use nonos_intel_xelp_display::regs::pll::Dpll;

const EN1: u32 = 0x46014;

#[test]
fn lock_and_disable() {
    let mut m = Regs::new(Some(2));
    m.set(EN1, (1 << 27) | (1 << 26));
    lock(&mut m, &Steps::new(1), Dpll::Dpll1).unwrap();
    assert_eq!(m.get(EN1), 0xCC00_0000);
    disable(&mut m, &Steps::new(1), Dpll::Dpll1).unwrap();
    assert_eq!(m.get(EN1), 0);
    // Enable off before power off (p. 178, steps 3 and 6); the status bits
    // a write carries back are read only, so only the request bits count.
    let asked: Vec<u32> = m.writes[1..].iter().map(|w| w.1 & ((1 << 31) | (1 << 27))).collect();
    assert_eq!(asked, [1 << 27, 0]);
}

#[test]
fn lock_that_never_comes_leaves_the_pll_down() {
    let mut m = Regs::new(Some(0));
    m.set(EN1, (1 << 27) | (1 << 26));
    m.stuck.insert((EN1, 30));
    assert!(lock(&mut m, &Steps::new(100), Dpll::Dpll1).is_err());
    assert_eq!(m.get(EN1) & ((1 << 31) | (1 << 27)), 0);
}

#[test]
fn disable_asks_power_off_even_when_the_lock_stays() {
    let mut m = Regs::new(Some(0));
    m.set(EN1, 0xCC00_0000);
    m.stuck.insert((EN1, 30));
    assert!(disable(&mut m, &Steps::new(100), Dpll::Dpll1).is_err());
    assert_eq!(m.get(EN1) & ((1 << 31) | (1 << 27)), 0);
}
