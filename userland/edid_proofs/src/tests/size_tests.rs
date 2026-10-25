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

//! A timing's totals, height and refresh with every field at u32's edges
//! (G-70): saturated, never wrapped, and the refresh exact against the
//! test's u128 reading.

#[path = "../../../gfx_serve_proofs/src/sweep/edges.rs"]
mod edges;

use crate::base::dtd::Detailed;
use crate::build::dtd_1080p;
use edges::{pairs, EDGES};

#[test]
fn a_timing_s_totals_and_refresh_hold_at_every_edge() {
    let mut seed = 0x7000_ED1Du64;
    let mut e = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        EDGES[(seed % EDGES.len() as u64) as usize]
    };
    let sat = |a: u32, b: u32| (a as u128 + b as u128).min(u32::MAX as u128);
    for (a, b) in pairs() {
        let mut t = noise();
        (t.h_active, t.h_blank, t.v_active, t.v_blank) = (a, b, b, a);
        assert_eq!((t.h_total() as u128, t.v_total() as u128), (sat(a, b), sat(b, a)));
    }
    for _ in 0..300_000 {
        let mut t = noise();
        (t.pixel_khz, t.h_active, t.h_blank, t.v_active, t.v_blank) = (e(), e(), e(), e(), e());
        t.interlaced = e() % 2 == 1;
        assert_eq!(t.h_total() as u128, sat(t.h_active, t.h_blank));
        assert_eq!(t.v_total() as u128, sat(t.v_active, t.v_blank));
        let lines = t.v_active as u128 * (1 + t.interlaced as u128);
        assert_eq!(t.height() as u128, lines.min(u32::MAX as u128));
        let total = t.h_total() as u128 * t.v_total() as u128;
        let mhz = if total == 0 {
            0
        } else {
            (t.pixel_khz as u128 * 1_000_000 / total).min(u32::MAX as u128)
        };
        assert_eq!(t.refresh_mhz() as u128, mhz, "{t:?}");
    }
}

fn noise() -> Detailed {
    Detailed::decode(&dtd_1080p()).expect("a 1080p descriptor")
}
