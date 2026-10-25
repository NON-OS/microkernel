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
//! HDMI dividers: Vol 12's worked example, the symbol clock's range and
//! the references.

use nonos_intel_xelp_display::pll::hdmi::dividers;

#[test]
fn vol_12_example_dvi_113_309_mhz() {
    // Vol 12 p. 183: P 2, Q 4, K 2; DCO Integer 179h, Fraction 592Ch.
    let d = dividers(113_309, 24_000).unwrap();
    assert_eq!((d.p, d.q, d.k, d.integer, d.fraction), (2, 4, 2, 0x179, 0x592C));
    assert_eq!(d.cfgcr1(), Some((4 << 10) | (1 << 9) | (0b010 << 6) | (0b0001 << 2)));
}

#[test]
fn symbol_clock_range() {
    // 20 to 600 MHz, and not strictly between 500 and 533.2 (p. 170).
    for ok in [20_000, 148_500, 500_000, 533_200, 594_000, 600_000] {
        assert!(dividers(ok, 24_000).is_ok(), "{ok}");
    }
    for bad in [0, 19_999, 500_001, 533_199, 600_001, u32::MAX] {
        assert!(dividers(bad, 24_000).is_err(), "{bad}");
    }
}

#[test]
fn references() {
    // 38.4 MHz is used as 19.2 (p. 180); other references refused.
    assert_eq!(dividers(148_500, 38_400), dividers(148_500, 19_200));
    assert!(dividers(148_500, 25_000).is_err());
}
