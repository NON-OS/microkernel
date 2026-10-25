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
//! DPLL_CFGCR0 and CFGCR1 encodings (Vol 2c p. 650 to 652).

use nonos_intel_xelp_display::pll::dividers::Dividers;

fn d(integer: u32, fraction: u32, p: u32, q: u32, k: u32) -> Dividers {
    Dividers { integer, fraction, p, q, k }
}

#[test]
fn every_p_and_k() {
    for (p, code) in [(2, 0b0001), (3, 0b0010), (5, 0b0100), (7, 0b1000)] {
        assert_eq!(d(0, 0, p, 1, 1).cfgcr1(), Some((0b001 << 6) | (code << 2)));
    }
    for (k, code) in [(1, 0b001), (2, 0b010), (3, 0b100)] {
        assert_eq!(d(0, 0, 2, 1, k).cfgcr1(), Some((code << 6) | (0b0001 << 2)));
    }
    for (p, k) in [(1, 1), (4, 1), (6, 1), (8, 1), (2, 0), (2, 4)] {
        assert_eq!(d(0, 0, p, 1, k).cfgcr1(), None, "{p} {k}");
    }
}

#[test]
fn q_only_with_k_two() {
    assert_eq!(d(0, 0, 2, 255, 2).cfgcr1(), Some((255 << 10) | (1 << 9) | (2 << 6) | (1 << 2)));
    assert_eq!(d(0, 0, 2, 256, 2).cfgcr1(), None);
    assert_eq!(d(0, 0, 2, 0, 2).cfgcr1(), None);
    // "If K is not 2, Q MUST be 1" (p. 652).
    assert_eq!(d(0, 0, 2, 2, 1).cfgcr1(), None);
    assert_eq!(d(0, 0, 2, 2, 3).cfgcr1(), None);
}

#[test]
fn dco_fields_fit() {
    assert_eq!(d(0x3FF, 0x7FFF, 2, 1, 1).cfgcr0(), Some((0x7FFF << 10) | 0x3FF));
    assert_eq!(d(0x400, 0, 2, 1, 1).cfgcr0(), None);
    assert_eq!(d(0, 0x8000, 2, 1, 1).cfgcr0(), None);
}
