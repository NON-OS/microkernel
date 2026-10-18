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

//! The HDMI DCO search and the divider split.

use crate::engine::dpll_cfgcr::cfgcr;
use crate::engine::dpll_hdmi::{dco, Dco, CENTRAL_MHZ};
use crate::engine::dpll_split::split;

#[test]
fn the_common_hdmi_clocks_land_where_the_search_says() {
    assert_eq!(dco(148_500), Some(Dco { divider: 12, central_mhz: 9000, dco_hz: 8_910_000_000 }));
    assert_eq!(dco(25_175).map(|d| (d.divider, d.central_mhz)), Some((76, 9600)));
    assert_eq!(dco(594_000).map(|d| (d.divider, d.central_mhz)), Some((3, 9000)), "only odd fits");
    assert_eq!(dco(0), None);
}

#[test]
fn the_1080p_clock_programs_its_registers() {
    let (c1, c2) = cfgcr(&dco(148_500).unwrap()).unwrap();
    // 8910 MHz / 24 MHz = 371.25: integer 371, fraction 0.25 x 2^15.
    assert_eq!(c1, (1 << 31) | (8192 << 9) | 371);
    // P0 2, P1 3, P2 2; qdiv mode on; centre 9000.
    assert_eq!(c2, (3 << 8) | (1 << 7) | (1 << 5) | (1 << 2) | 1);
}

#[test]
fn every_clock_the_search_takes_splits_back_to_its_divider() {
    for khz in (25_000..=594_000).step_by(250) {
        let Some(d) = dco(khz) else { continue };
        let centre = d.central_mhz as u64 * 1_000_000;
        assert!(d.dco_hz <= centre + centre / 100 && d.dco_hz >= centre - centre * 6 / 100);
        assert!(CENTRAL_MHZ.contains(&(d.central_mhz as u64)));
        let (p0, p1, p2) = split(d.divider).unwrap_or_else(|| panic!("{khz} kHz: {}", d.divider));
        assert_eq!(p0 * p1 * p2, d.divider);
        assert!(matches!(p0, 1 | 2 | 3 | 7) && matches!(p2, 1 | 2 | 3 | 5) && p1 <= 255);
        assert!(cfgcr(&d).is_some());
    }
}
