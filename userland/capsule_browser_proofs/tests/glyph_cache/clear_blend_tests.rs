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

//! Glyph coverage over a pixel that is not opaque, the clear overlay a shell
//! draws its desktop labels on. Before the fix the edge of white text on a
//! cleared pixel came out as opaque grey, a dark ring around every letter.

use super::blend::mix;

const CLEAR: u32 = 0x0000_0000;
const WHITE: u32 = 0xffff_ffff;

#[test]
fn half_coverage_on_clear_keeps_the_text_colour_at_half_alpha() {
    assert_eq!(mix(CLEAR, WHITE, 128), 0x80ff_ffff);
}

#[test]
fn full_coverage_on_clear_is_the_opaque_text_colour() {
    assert_eq!(mix(CLEAR, WHITE, 255), WHITE);
    assert_eq!(mix(CLEAR, 0xff12_3456, 255), 0xff12_3456);
}

#[test]
fn zero_coverage_leaves_the_clear_pixel_alone() {
    assert_eq!(mix(CLEAR, WHITE, 0), CLEAR);
    assert_eq!(mix(0x8c06_0a0e, WHITE, 0), 0x8c06_0a0e);
}

#[test]
fn edges_on_clear_never_darken_the_text_colour() {
    for cov in 1..=255u32 {
        let p = mix(CLEAR, WHITE, cov as u8);
        assert_eq!(p & 0x00ff_ffff, 0x00ff_ffff, "cov {cov}");
        assert_eq!(p >> 24, cov, "cov {cov}");
    }
}

#[test]
fn text_on_a_translucent_scrim_raises_alpha_and_keeps_the_scrim_under_it() {
    // The desktop icon scrim, 0x8c alpha over near black, under white text.
    assert_eq!(mix(0x8c06_0a0e, WHITE, 255), WHITE);
    assert_eq!(mix(0x8c06_0a0e, WHITE, 128), 0xc6a7_a8aa);
}

#[test]
fn translucent_text_on_clear_carries_both_alphas() {
    assert_eq!(mix(CLEAR, 0x80ff_ffff, 255), 0x80ff_ffff);
    assert_eq!(mix(CLEAR, 0x80ff_ffff, 128), 0x40ff_ffff);
}
