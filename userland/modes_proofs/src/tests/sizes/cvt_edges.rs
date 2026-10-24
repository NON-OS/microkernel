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

//! CVT at every edge of width, height and rate: a timing whose totals and
//! clock fit their types, sized as asked, or None; never a wrap.

use super::edges::EDGES;
use crate::cvt::{cvt, Blanking};

#[test]
fn cvt_at_every_edge_is_a_timing_that_fits_or_none() {
    let rates = EDGES.iter().copied().chain([24_000, 60_000, 240_000, 1_800_000, 1_818_181]);
    let (mut some, mut none) = (0, 0);
    for (w, h, hz) in EDGES
        .iter()
        .flat_map(|&w| EDGES.iter().map(move |&h| (w, h)))
        .flat_map(|(w, h)| rates.clone().map(move |hz| (w, h, hz)))
    {
        for b in [Blanking::Standard, Blanking::Reduced, Blanking::ReducedV2] {
            let Some(t) = cvt(w, h, hz, b) else {
                none += 1;
                continue;
            };
            let width = if b == Blanking::ReducedV2 { w } else { w / 8 * 8 };
            assert_eq!((t.h_active, t.v_active), (width, h), "{w}x{h}@{hz} {b:?}");
            assert!(t.h_active.checked_add(t.h_blank).is_some(), "{w}x{h}@{hz} {b:?}");
            assert!(t.v_active.checked_add(t.v_blank).is_some(), "{w}x{h}@{hz} {b:?}");
            let h_parts = t.h_front as u64 + t.h_sync as u64;
            assert!(t.pixel_khz > 0 && h_parts < t.h_blank as u64, "{w}x{h}@{hz} {b:?} {t:?}");
            // The blanking covers CVT's minimum time (550 us of sync and back
            // porch standard, 460 us of blanking reduced), in lines of the
            // estimated period num / den us, read here in u128.
            let r = hz as u128;
            let (us, lines, rows) = match b {
                Blanking::Standard => (550, t.v_blank as u128 - 3, h as u128 + 3),
                _ => (460, t.v_blank as u128, h as u128),
            };
            let num = 1_000_000_000 - us * r;
            assert!(lines * num >= us * r * rows, "{w}x{h}@{hz} {b:?} {t:?}");
            some += 1;
        }
    }
    assert!(some > 1_000 && none > 1_000, "{some} {none}");
    for b in [Blanking::Standard, Blanking::Reduced, Blanking::ReducedV2] {
        assert!(cvt(1920, 1080, 60_000, b).is_some(), "an honest size still gives one");
    }
}
