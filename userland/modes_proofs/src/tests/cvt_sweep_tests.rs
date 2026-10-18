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

//! CVT over a sweep of sizes, rates and blankings.

use crate::cvt::{cvt, Blanking};

#[test]
fn every_size_and_rate_gives_a_self_consistent_timing_or_none() {
    for w in (320..=8192).step_by(232) {
        for h in (480..=8192).step_by(312) {
            for hz in [24_000, 50_000, 59_940, 60_000, 75_000, 120_000, 144_000, 240_000] {
                for b in [Blanking::Standard, Blanking::Reduced, Blanking::ReducedV2] {
                    let Some(t) = cvt(w, h, hz, b) else { continue };
                    assert!(t.h_front + t.h_sync < t.h_blank && t.v_front + t.v_sync < t.v_blank);
                    // CVT never runs faster than asked: the lines cover at
                    // least the blanking time and the clock is floored to a
                    // step. Extra lines (the minimum porches, the sync of an
                    // odd aspect ratio) slow it a little; under 5% here once
                    // the clock step itself is small, from 25 MHz.
                    let r = t.refresh_mhz();
                    assert!(r <= hz, "{w}x{h}@{hz} {b:?}: {r}");
                    if t.pixel_khz >= 25_000 {
                        assert!(r as u64 * 20 >= hz as u64 * 19, "{w}x{h}@{hz} {b:?}: {r}");
                    }
                }
            }
        }
    }
}
