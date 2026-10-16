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

//! Night light: every ramp at every temperature only ever rises (no level
//! darker than a lower one, so no banding inversion), 6500 K is no change,
//! red is always kept, a warmer light never brightens green or blue, and a
//! tinted rectangle is looked up channel by channel with alpha kept.

use crate::damage::Rect;
use crate::display::night_rule::{active, gains, ramps, NEUTRAL_K, WARMEST_K};
use crate::frame_pacer::night_tint::tint;
use crate::sw_blitter::Surface;

#[test]
fn every_ramp_only_ever_rises() {
    for k in (WARMEST_K..=NEUTRAL_K).step_by(50) {
        for (c, ramp) in ramps(k).iter().enumerate() {
            assert!(ramp.windows(2).all(|w| w[0] <= w[1]), "{k} K channel {c}");
            assert_eq!(ramp[0], 0, "black stays black");
        }
    }
}

#[test]
fn neutral_is_no_change_and_warmer_never_brightens_green_or_blue() {
    let r = ramps(NEUTRAL_K);
    assert!((0..256).all(|v| r.iter().all(|ramp| ramp[v] as usize == v)));
    assert!(!active(NEUTRAL_K) && !active(0) && active(3400));
    let mut last = gains(NEUTRAL_K);
    for k in (WARMEST_K..NEUTRAL_K).rev().step_by(25) {
        let g = gains(k);
        assert_eq!(g[0], 255, "red kept at {k} K");
        assert!(g[1] <= last[1] && g[2] <= last[2], "{k} K is no bluer than {} K", k + 25);
        last = g;
    }
    assert_eq!(gains(2000), [255, 137, 18]);
    assert_eq!(gains(2250), [255, 149, 45], "halfway between table rows");
}

#[test]
fn a_tinted_rectangle_keeps_alpha_and_the_rest_of_the_canvas() {
    let mut px = vec![0x80FF_FFFFu32; 4];
    let byte_len = px.len() as u64 * 4;
    let s = Surface { base_va: px.as_mut_ptr() as u64, stride: 16, width: 4, height: 1, byte_len };
    tint(s, Rect { x: 1, y: 0, width: 2, height: 1 }, &ramps(2000));
    assert_eq!(px, [0x80FF_FFFF, 0x80FF_8912, 0x80FF_8912, 0x80FF_FFFF]);
}
