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

//! A timing's rates against a range descriptor, every field drawn from
//! u32's edges: the same verdict as the test's reading in u128, never a
//! panic.

use super::edges::EDGES;
use crate::check::in_range;
use nonos_edid::{Detailed, RangeLimits};

fn model(t: &Detailed, r: &RangeLimits) -> bool {
    let (ht, vt) = (t.h_total() as u128, t.v_total() as u128);
    if ht == 0 || vt == 0 {
        return false;
    }
    let line = t.pixel_khz as u128 * 1000 / ht;
    let frame = line * 1000 / vt;
    let v = r.v_min_hz as u128 * 1000..(r.v_max_hz as u128 + 1) * 1000;
    let h = (r.h_min_khz as u128).saturating_sub(1) * 1000..(r.h_max_khz as u128 + 1) * 1000;
    let clock = t.pixel_khz != 0 && (r.max_pixel_khz == 0 || t.pixel_khz <= r.max_pixel_khz);
    let width = r.max_h_active == 0 || t.h_active <= r.max_h_active;
    v.contains(&frame) && h.contains(&line) && clock && width
}

#[test]
fn rates_against_a_range_agree_with_u128_at_every_edge() {
    let mut seed = 0x7000_0A1Eu64;
    let mut e = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        EDGES[(seed % EDGES.len() as u64) as usize]
    };
    let mut yes = 0;
    for _ in 0..300_000 {
        let mut t = crate::make::EMPTY;
        (t.pixel_khz, t.h_active, t.h_blank, t.v_active, t.v_blank) = (e(), e(), e(), e(), e());
        let lim = RangeLimits {
            v_min_hz: e() % 300,
            v_max_hz: e(),
            h_min_khz: e() % 300,
            h_max_khz: e(),
            max_pixel_khz: e(),
            max_h_active: e(),
        };
        let got = in_range(&t, &lim);
        assert_eq!(got, model(&t, &lim), "{t:?} {lim:?}");
        yes += got as u32;
    }
    assert!(yes > 100, "{yes}");
}
