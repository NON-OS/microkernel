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
//! The Voltage Swing Programming Sequence (Vol 12 pp. 392, 393) on the
//! model, for one row of each table: every lane's fields, per-lane loadgen,
//! and training turned off before the change and on after it.

use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::signal::Signal;
use nonos_intel_xelp_display::phy::swing::program;
use nonos_intel_xelp_display::regs::clka::Combo;

const B: u32 = 0x6C000;
fn ln(n: u32, off: u32) -> u32 {
    B + 0x800 + 0x100 * n + off
}

#[test]
fn dp_hbr_level_1_1_on_four_lanes() {
    let mut m = Regs::new(Some(0));
    let sig = Signal::Dp { mbps: 2700, lanes: 4, vs: 1, pe: 1 };
    program(&mut m, Combo::B, sig).unwrap();
    // DP up to HBR, (1, 1): Swing Sel 1100b, N Scalar 73h, Cursor 34h, Post
    // Cursor 1 0Bh (p. 393).
    for n in 0..4 {
        let dw2 = m.get(ln(n, 0x88));
        assert_eq!((dw2 >> 15 & 1, dw2 >> 11 & 7, dw2 & 0xFF), (1, 0b100, 0x98));
        let dw4 = m.get(ln(n, 0x90));
        let loadgen = [0, 1, 1, 1][n as usize];
        assert_eq!(
            (dw4 >> 31, dw4 >> 12 & 0x3F, dw4 >> 6 & 0x3F, dw4 & 0x3F),
            (loadgen, 0x0B, 0, 0x34)
        );
        let dw5 = m.get(ln(n, 0x94));
        // Training on, 2tap 0, 3tap 1, cursor program 0, polarity 0, scaling
        // 010b, rterm 110b (p. 393).
        assert_eq!(dw5, (1 << 31) | (1 << 29) | (0b010 << 18) | (0b110 << 3));
        assert_eq!(m.get(ln(n, 0x9C)) >> 24 & 0x7F, 0x73);
        assert_eq!(m.get(ln(n, 0x04)) >> 26 & 1, 1);
    }
    assert_eq!(m.get(B + 0x014) & 3, 3);
}

#[test]
fn training_off_first_on_last() {
    let mut m = Regs::new(Some(0));
    let sig = Signal::Edp { mbps: 5400, lanes: 2, vs: 0, pe: 0 };
    program(&mut m, Combo::B, sig).unwrap();
    let dw5: Vec<u32> = m.writes.iter().filter(|w| w.0 == B + 0x694).map(|w| w.1 >> 31).collect();
    assert_eq!(dw5.first(), Some(&0));
    assert_eq!(dw5.last(), Some(&1));
    let last = m.writes.last().unwrap();
    assert_eq!(*last, (B + 0x694, last.1));
    // Two lanes at or under 6 Gbit/s: loadgen 0, 1, 1, 0 (p. 392).
    assert_eq!([0, 1, 2, 3].map(|n| m.get(ln(n, 0x90)) >> 31), [0, 1, 1, 0]);
    // eDP up to HBR2 (0, 0): Swing Sel 0000b, N Scalar 7Fh (p. 394).
    assert_eq!(m.get(ln(0, 0x88)) & 0xB800, 0);
}
