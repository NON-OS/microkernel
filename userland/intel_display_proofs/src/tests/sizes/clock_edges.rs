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

//! Clocks at every edge: the HDMI DCO and DisplayPort's link M/N.

use super::edges::EDGES;
use super::Pick;
use crate::engine::dpll_hdmi::dco;
use crate::engine::mn::{compute, N};

#[test]
fn the_dco_is_found_in_range_or_not_at_all_at_every_edge() {
    let clocks = EDGES.iter().copied().chain((0..4096u32).map(|i| 25_000 + i * 150));
    let mut found = 0;
    for khz in clocks {
        let Some(d) = dco(khz) else { continue };
        let afe = khz as u128 * 5000;
        assert_eq!(d.dco_hz as u128, d.divider as u128 * afe, "{khz}");
        let centre = d.central_mhz as u128 * 1_000_000;
        let dev = (d.dco_hz as u128).abs_diff(centre) * 10_000 / centre;
        assert!(dev <= if d.dco_hz as u128 >= centre { 100 } else { 600 }, "{khz}: {dev}");
        found += 1;
    }
    assert!(found > 50, "{found}");
}

#[test]
fn link_m_n_agree_with_u128_at_every_edge() {
    let mut p = Pick(0x7000_3A11);
    let mut ok = 0;
    for _ in 0..300_000 {
        let (khz, bpp, lanes, mbps) = (p.edge(), p.edge() % 64, p.edge() % 8, p.edge());
        let (sym, n) = (mbps as u128 * 100, N as u128);
        let (num, den) = (khz as u128 * bpp as u128, sym * lanes as u128 * 8);
        let link = if sym == 0 { 0 } else { (khz as u128 * n + sym / 2) / sym };
        let good = khz > 0 && bpp > 0 && lanes > 0 && sym > 0 && num <= den && link < 1 << 24;
        match compute(khz, bpp, lanes, mbps) {
            Ok(m) => {
                assert!(good);
                assert_eq!(m.data_m as u128, (num * n + den / 2) / den);
                assert_eq!(m.link_m as u128, link);
                ok += 1;
            }
            Err(_) => assert!(!good, "{khz} {bpp} {lanes} {mbps}"),
        }
    }
    assert!(ok > 100, "{ok}");
}
