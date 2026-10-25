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
//! Vol 12's DisplayPort PLL table (p. 178, 179): every row gives back its
//! rate by Vol 12's own formula (p. 182), and encodes as the table's bits.

use nonos_intel_xelp_display::pll::dividers::Dividers;
use nonos_intel_xelp_display::pll::dp::dividers;

const RATES: [u32; 8] = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];

/// Symbol clock = DCO * reference / (5 P Q K), and the link rate is ten
/// symbols; so, exactly, rate * 1000 * P Q K * 2^15 = 2 * DCO * reference.
fn rate_holds(d: &Dividers, ref_khz: u32, mbps: u32) -> bool {
    let dco = ((d.integer as u64) << 15) + d.fraction as u64;
    let lhs = mbps as u64 * 1000 * (d.p * d.q * d.k) as u64 * (1 << 15);
    lhs == 2 * dco * ref_khz as u64
}

#[test]
fn every_row_gives_its_rate() {
    for r in RATES {
        for (look, real) in [(24_000, 24_000), (19_200, 19_200), (38_400, 19_200)] {
            let d = dividers(r, look).unwrap();
            assert!(rate_holds(&d, real, r), "{r} at {look}: {d:?}");
            assert!(d.cfgcr0().is_some() && d.cfgcr1().is_some());
        }
    }
}

#[test]
fn rows_as_the_table_writes_them() {
    // 2.16 at 24 MHz: DCO 168h, 0000h; Pdiv 0001b, Kdiv 010b, Qdiv on, 02h.
    let d = dividers(2160, 24_000).unwrap();
    assert_eq!(d.cfgcr0(), Some(0x168));
    assert_eq!(d.cfgcr1(), Some((2 << 10) | (1 << 9) | (0b010 << 6) | (0b0001 << 2)));
    // 5.4 at 19.2 MHz: DCO 1A5h, 7000h; Pdiv 0010b, Kdiv 001b, Qdiv off.
    let d = dividers(5400, 19_200).unwrap();
    assert_eq!(d.cfgcr0(), Some((0x7000 << 10) | 0x1A5));
    assert_eq!(d.cfgcr1(), Some((0b001 << 6) | (0b0010 << 2)));
    // 6.48 at 38.4 MHz: the 19.2 row, 1FAh, 2000h.
    assert_eq!(dividers(6480, 38_400).unwrap().cfgcr0(), Some((0x2000 << 10) | 0x1FA));
}
