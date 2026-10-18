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

//! CVT 1.2 against the results its formulas give for well known modes.

use crate::cvt::{cvt, Blanking};
use nonos_edid::Detailed;

fn row(t: Detailed) -> (u32, [u32; 4], [u32; 4], bool, bool) {
    (
        t.pixel_khz,
        [t.h_active, t.h_front, t.h_sync, t.h_blank - t.h_front - t.h_sync],
        [t.v_active, t.v_front, t.v_sync, t.v_blank - t.v_front - t.v_sync],
        t.h_positive,
        t.v_positive,
    )
}

#[test]
fn standard_blanking_1080p60() {
    let t = cvt(1920, 1080, 60_000, Blanking::Standard).unwrap();
    assert_eq!(row(t), (173_000, [1920, 128, 200, 328], [1080, 3, 5, 32], false, true));
    assert_eq!((t.h_total(), t.v_total()), (2576, 1120));
}

#[test]
fn reduced_blanking_v1_for_common_panels() {
    let t = cvt(1920, 1080, 60_000, Blanking::Reduced).unwrap();
    assert_eq!(row(t), (138_500, [1920, 48, 32, 80], [1080, 3, 5, 23], true, false));
    let t = cvt(2560, 1440, 60_000, Blanking::Reduced).unwrap();
    assert_eq!((t.pixel_khz, t.h_total(), t.v_total()), (241_500, 2720, 1481));
    let t = cvt(3840, 2160, 60_000, Blanking::Reduced).unwrap();
    assert_eq!((t.pixel_khz, t.h_total(), t.v_total()), (533_250, 4000, 2222));
}

#[test]
fn reduced_blanking_v2_keeps_any_width_and_1_khz_steps() {
    let t = cvt(1920, 1080, 60_000, Blanking::ReducedV2).unwrap();
    assert_eq!(row(t), (133_320, [1920, 8, 32, 40], [1080, 17, 8, 6], true, false));
    let t = cvt(1366, 768, 60_000, Blanking::ReducedV2).unwrap();
    assert_eq!(t.h_active, 1366, "v2 does not cut the width to 8");
}

#[test]
fn standard_and_v1_cut_the_width_to_the_cell() {
    let t = cvt(1366, 768, 60_000, Blanking::Standard).unwrap();
    assert_eq!(row(t), (84_750, [1360, 72, 136, 208], [768, 3, 5, 22], false, true));
}

#[test]
fn a_rate_the_formula_cannot_meet_is_refused() {
    // 1 / rate shorter than the 550 us the vertical retrace needs.
    assert!(cvt(1024, 768, 1_818_182, Blanking::Standard).is_none());
    assert!(cvt(1024, 768, 2_173_913, Blanking::Reduced).is_none());
    assert!(cvt(0, 768, 60_000, Blanking::Standard).is_none());
    assert!(cvt(7, 768, 60_000, Blanking::Reduced).is_none(), "no whole cell");
    assert!(cvt(1024, 0, 60_000, Blanking::ReducedV2).is_none());
    assert!(cvt(1024, 768, 0, Blanking::Standard).is_none());
}
