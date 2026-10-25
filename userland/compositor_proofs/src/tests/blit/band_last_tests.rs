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
//! The band is the last thing a frame draws, read from the compositor's
//! own source, so nothing a client draws or places can cover it; and its
//! glyphs face the right way, read from the art, not through `set`.

const COMPOSITE: &str = include_str!("../../../../compositor/src/frame_pacer/composite.rs");

#[test]
fn the_band_is_drawn_last() {
    let after = &COMPOSITE[COMPOSITE.find("super::indicator_band::draw(").expect("drawn")..];
    let rest = after[after.find(";\n").expect("one statement") + 2..].trim_start();
    assert!(rest.starts_with('}'), "nothing is drawn after the band: {}", &rest[..20]);
}

#[test]
fn the_glyphs_face_the_right_way() {
    use crate::frame_pacer::band_glyphs::{CAMERA, LOCK, MICROPHONE, SCREEN};
    assert_eq!((CAMERA[2], CAMERA[4]), (0b11_1111_0001, 0b11_1111_0111), "the lens on the right");
    assert_eq!((MICROPHONE[0], MICROPHONE[8]), (0b00_0111_1000, 0b00_0011_0000));
    assert_eq!((SCREEN[0], SCREEN[1], SCREEN[7]), (0b11_1111_1111, 0b10_0000_0001, 0b00_0011_0000));
    assert_eq!((LOCK[1], LOCK[6]), (0b00_1000_0100, 0b11_1100_1111), "the keyhole in the body");
    let set = crate::frame_pacer::band_glyphs::set;
    assert!(set(&CAMERA, 9, 2) && !set(&CAMERA, 6, 2) && !set(&CAMERA, 0, 0));
    assert!(set(&LOCK, 2, 1) && !set(&LOCK, 4, 6) && !set(&LOCK, 10, 0) && !set(&LOCK, 0, 10));
}

const PALETTE: &str = include_str!("../../../../capsule_desktop_shell/src/render/palette.rs");
const SPLASH: &str = include_str!("../../../../capsule_boot_splash/src/warning.rs");

fn token(src: &str, name: &str) -> u32 {
    let line = src.lines().find(|l| l.starts_with(&format!("pub const {name}: u32"))).expect(name);
    let hex = line.split("0x").nth(1).expect("a hex value").trim_end_matches(';').replace('_', "");
    u32::from_str_radix(&hex, 16).expect("hex")
}

#[test]
fn the_band_takes_the_shells_tokens() {
    use crate::frame_pacer::indicator_band::*;
    assert_eq!(BAND_ARGB, token(PALETTE, "BAR") | 0xFF00_0000, "the bar's colour, opaque");
    assert_eq!(CAMERA_ON, token(PALETTE, "POSITIVE"));
    assert_eq!(MICROPHONE_ON, token(PALETTE, "WARN"));
    assert_eq!(SCREEN_ON, token(PALETTE, "ACCENT"));
    assert_eq!(TRUSTED_ON, token(PALETTE, "TEXT"));
    assert_eq!(MARK_OFF, token(PALETTE, "TEXT_MUTED"));
    assert_eq!(DMA_UNCONFINED, token(PALETTE, "NEGATIVE"));
    assert_eq!(token(SPLASH, "RED"), DMA_UNCONFINED, "the splash's line is the band's red");
}
