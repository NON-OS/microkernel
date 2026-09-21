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
//! Mixing two colours.

use crate::theme::derive::{mix, opaque, with_alpha};

#[test]
fn the_ends_of_a_mix_are_exact() {
    let a = 0xFF12_3456;
    let b = 0xFF98_7654;
    assert_eq!(mix(a, b, 0), a, "no movement is the first colour");
    assert_eq!(mix(a, b, 255), b & 0x00FF_FFFF | (a & 0xFF00_0000), "all the way is the second");
}
#[test]
fn the_midpoint_rounds_rather_than_truncates() {
    // 0x00 and 0x03 average to 1.5, which rounds to 2 rather than down to 1.
    assert_eq!(mix(0xFF00_0000, 0xFF03_0303, 128) & 0xFF, 2);
    assert_eq!(mix(0xFF00_0000, 0xFFFF_FFFF, 128) & 0xFF, 128);
}
#[test]
fn a_mix_keeps_the_first_colours_alpha() {
    let translucent = 0x2011_2233;
    assert_eq!(mix(translucent, 0xFFFF_FFFF, 128) >> 24, 0x20);
    assert_eq!(mix(0xFF11_2233, 0x0011_2233, 128) >> 24, 0xFF);
}
/*
 * Every channel mixes independently. One channel leaking into another is a hue
 * shift that only some themes would show.
 *
 * 128 of 255 is a hair past half, so a channel falling lands on 127 and one rising
 * on 128. Pinning both is what makes this a test of the channels rather than of
 * where the midpoint happens to sit.
 */
#[test]
fn channels_do_not_bleed_into_each_other() {
    let got = mix(0xFFFF_0000, 0xFF00_00FF, 128);
    assert_eq!((got >> 16) & 0xFF, 127, "red falls");
    assert_eq!((got >> 8) & 0xFF, 0, "green stays absent");
    assert_eq!(got & 0xFF, 128, "blue rises");
}
