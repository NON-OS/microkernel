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

//! Placement: at the firmware's own mode the logo lands on the firmware's
//! very pixels; on a scaled or different canvas it keeps its centre's place
//! and its shape; a logo the mode cannot hold is not placed.

use crate::logo::place::{guess_mode, place, Spot};

#[test]
fn the_firmware_mode_itself_puts_every_pixel_back() {
    for (x, y, w, h) in [(810, 345, 300, 211), (0, 0, 1920, 1080), (1, 2, 3, 5)] {
        let s = place(x, y, w, h, (1920, 1080), (1920, 1080)).unwrap();
        assert_eq!(s, Spot { x: x as i64, y: y as i64, w, h });
    }
}

#[test]
fn a_doubled_canvas_halves_the_logo_and_its_place() {
    let s = place(1720, 690, 400, 300, (3840, 2160), (1920, 1080)).unwrap();
    assert_eq!(s, Spot { x: 860, y: 345, w: 200, h: 150 });
}

#[test]
fn another_shape_keeps_the_centre_and_is_never_stretched() {
    // 4:3 firmware mode onto a 16:9 canvas: the height ratio is the smaller.
    let s = place(412, 284, 200, 200, (1024, 768), (1920, 1080)).unwrap();
    assert_eq!((s.w, s.h), (281, 281), "1080/768 of 200 on both sides");
    assert_eq!(s.x + s.w as i64 / 2, (2 * 412 + 200) * 1920 / 2048);
    assert_eq!(s.y + s.h as i64 / 2, (2 * 284 + 200) * 1080 / 1536);
}

#[test]
fn a_logo_outside_its_mode_or_shrunk_to_nothing_is_not_placed() {
    assert_eq!(place(1800, 0, 200, 10, (1920, 1080), (1920, 1080)), None);
    assert_eq!(place(0, 1000, 10, 100, (1920, 1080), (1920, 1080)), None);
    assert_eq!(place(0, 0, 1, 1, (0, 1080), (1920, 1080)), None);
    assert_eq!(place(0, 0, 1, 1, (3840, 2160), (1920, 1080)), None, "half a pixel");
}

#[test]
fn a_centred_logo_gives_back_its_mode() {
    assert_eq!(guess_mode(810, 345, 300, 211, (1920, 1080)), (1920, 1080));
    let mode = guess_mode(1720, 690, 400, 300, (1920, 1080));
    assert_eq!(mode, (3840, 2160));
    assert_eq!(guess_mode(0, 900, 100, 400, (1000, 500)).1, 1300, "tall enough to hold it");
}
