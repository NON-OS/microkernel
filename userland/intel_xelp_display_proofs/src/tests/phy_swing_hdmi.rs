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
//! Swing programming for HDMI (Vol 12 p. 395).

use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::signal::Signal;
use nonos_intel_xelp_display::phy::swing::program;
use nonos_intel_xelp_display::regs::clka::Combo;

const B: u32 = 0x6C000;
fn ln(n: u32, off: u32) -> u32 {
    B + 0x800 + 0x100 * n + off
}

#[test]
fn hdmi_row_four_and_cmnkeeper_off() {
    let mut m = Regs::new(Some(0));
    m.set(ln(0, 0x04), 1 << 26);
    program(&mut m, Combo::B, Signal::Hdmi { row: 3, symbol_khz: 594_000 }).unwrap();
    // HDMI ALS row: 1011b, 73h, 3Fh, 00h (p. 395); not DP, so cmnkeeper 0.
    let dw2 = m.get(ln(2, 0x88));
    assert_eq!((dw2 >> 15 & 1, dw2 >> 11 & 7), (1, 0b011));
    assert_eq!(m.get(ln(3, 0x04)) >> 26 & 1, 0);
    assert_eq!([0, 1, 2, 3].map(|n| m.get(ln(n, 0x90)) >> 31), [0, 1, 1, 1]);
}
