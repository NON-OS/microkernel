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
//! The Combo PHY Initialization Sequence (Vol 12 pp. 391, 392) on the
//! model: values, order, what is skipped and what is refused untouched.

use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::init::init;
use nonos_intel_xelp_display::regs::clka::Combo;

/// Ports A's and B's PHY blocks, and A's PHY_MISC (Vol 2c p. 664).
pub(super) const A: u32 = 0x162000;
pub(super) const B: u32 = 0x6C000;
const MISC_A: u32 = 0x64C00;

/// A PHY that reports 1.05 V, dot-1 in PORT_COMP_DW3 (p. 899).
pub(super) fn fresh() -> Regs {
    let mut m = Regs::new(Some(0));
    m.set(A + 0x10C, (1 << 26) | (2 << 24));
    m.set(B + 0x10C, (1 << 26) | (2 << 24));
    m.set(MISC_A, 1 << 23);
    m.set(A + 0x800 + 0xA0, 0x4000_0000); // TX_DW8 lane 0: div4, as at reset
    m
}

#[test]
fn port_a_from_cold() {
    let mut m = fresh();
    init(&mut m, Combo::A).unwrap();
    for n in 0..4 {
        // odcc_clksel 1, odcc_clk_div_sel 00 (p. 953); DCC continuous (p. 906).
        assert_eq!(m.get(A + 0x800 + 0x100 * n + 0xA0) >> 29, 0b100);
        assert_eq!((m.get(A + 0x800 + 0x100 * n + 0x004) >> 20) & 3, 3);
    }
    assert_eq!(m.get(MISC_A) & (1 << 23), 0);
    assert_eq!(m.get(A + 0x104), 0x0044_0000);
    assert_eq!((m.get(A + 0x124), m.get(A + 0x128)), (0x9A00AB25, 0x8AE38FF1));
    // irefgen (p. 901), Comp Init last but one, CL Power Down last (p. 392).
    assert_eq!(m.get(A + 0x120) & (1 << 24), 1 << 24);
    let tail: Vec<u32> = m.writes.iter().rev().take(2).map(|w| w.0).collect();
    assert_eq!(tail, [A + 0x014, A + 0x100]);
    assert_eq!(m.get(A + 0x014) & (1 << 4), 1 << 4);
    assert!(!m.reads.iter().any(|r| (A + 0x600..A + 0x700).contains(r)));
}

#[test]
fn already_initialised_is_left() {
    let mut m = fresh();
    m.set(A + 0x100, 1 << 31);
    init(&mut m, Combo::A).unwrap();
    assert!(m.writes.is_empty());
}
