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

//! The sRGB tables the colour filters stand on: black and white at the
//! ends, every byte level its own linear value, every level back to itself,
//! the curve's middle where IEC 61966-2-1 puts it, and the roots they are
//! taken with.

use crate::display::srgb_rule::{from_linear, root, to_linear, LIN_MAX};

#[test]
fn every_level_has_its_own_light_and_comes_back() {
    assert_eq!((to_linear(0), to_linear(255)), (0, LIN_MAX as u16));
    for v in 1..=255u8 {
        assert!(to_linear(v) > to_linear(v - 1), "level {v} kept apart");
        assert_eq!(from_linear(to_linear(v)), v, "level {v} back");
    }
    assert_eq!(from_linear(0), 0);
    assert_eq!(from_linear(u16::MAX), 255, "past the top clamps");
}

#[test]
fn the_middle_of_the_curve_and_the_roots() {
    // ((128/255 + 0.055) / 1.055)^2.4 = 0.2158605 of full light.
    assert!((to_linear(128) as i32 - 884).abs() <= 1, "{}", to_linear(128));
    assert!((root(0.5, 2) - core::f32::consts::FRAC_1_SQRT_2).abs() < 1e-5);
    assert!((root(1e-4, 5) - 0.158_489_32).abs() < 1e-5, "a dark level's root");
    assert!((root(0.2, 12) - 0.874_485_9).abs() < 1e-5);
    assert_eq!(root(0.0, 5), 0.0);
}
