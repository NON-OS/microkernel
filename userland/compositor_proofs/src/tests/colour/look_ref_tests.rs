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

//! A look's fixed-point pass against the same filter worked in floating
//! point, channel by channel: within one level for every filter, on colours
//! that tell red, green and blue apart.

use crate::display::access_rule::{FILTER_DEUTAN, FILTER_GREYSCALE, FILTER_PROTAN, FILTER_TRITAN};
use crate::display::colour_rule::matrix;
use crate::display::cvd_rule::apply;
use crate::display::srgb_rule::{from_linear, LIN_MAX};
use crate::tests::colour::look_tests::{looked, rgb};

#[test]
fn the_pass_matches_floating_point_channel_by_channel() {
    let colours = [0xFFFF_0000, 0xFF00_FF00, 0xFF00_00FF, 0xFFD0_3020, 0xFF20_60C0, 0xFF9A_C010];
    for f in [FILTER_GREYSCALE, FILTER_PROTAN, FILTER_DEUTAN, FILTER_TRITAN] {
        let m = matrix(f).unwrap();
        for (c, got) in colours.iter().zip(looked(f, &colours)) {
            let lin =
                apply(&m, rgb(*c)).map(|v| (v * LIN_MAX as f32).round().clamp(0.0, LIN_MAX as f32));
            let want = lin.map(|v| from_linear(v as u16));
            let [b, g, r, a] = got.to_le_bytes();
            assert_eq!(a, 0xFF);
            for (ch, (x, y)) in [r, g, b].iter().zip(want.iter()).enumerate() {
                assert!(x.abs_diff(*y) <= 1, "filter {f} colour {c:#x} channel {ch}: {x} vs {y}");
            }
        }
    }
}
