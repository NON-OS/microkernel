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

//! The floor canvas: a screen below 1024x720 either way gets a canvas of its
//! own shape that covers 1024x720, and a screen at or above it keeps its own.

#[path = "../../compositor/src/setup/canvas_floor.rs"]
mod canvas_floor;

use canvas_floor::floor_canvas;

#[test]
fn small_firmware_modes_get_the_floor() {
    assert_eq!(floor_canvas(800, 600), Some((1024, 768)));
    assert_eq!(floor_canvas(640, 480), Some((1024, 768)));
    assert_eq!(floor_canvas(1024, 600), Some((1229, 720)));
    assert_eq!(floor_canvas(1280, 600), Some((1536, 720)));
}

#[test]
fn a_screen_at_or_above_the_floor_keeps_its_own_size() {
    for (w, h) in [(1024, 768), (1280, 720), (1366, 768), (1920, 1080), (3840, 2160)] {
        assert_eq!(floor_canvas(w, h), None);
    }
    assert_eq!(floor_canvas(0, 600), None);
}

#[test]
fn every_floor_canvas_covers_the_floor_and_keeps_the_shape() {
    for w in (320..1600).step_by(7) {
        for h in (200..900).step_by(5) {
            let Some((cw, ch)) = floor_canvas(w, h) else { continue };
            assert!(cw >= 1024 && ch >= 720, "{w}x{h} -> {cw}x{ch}");
            assert!(cw >= w && ch >= h);
            // Within a pixel of the screen's aspect ratio.
            let skew = (cw as i64 * h as i64 - ch as i64 * w as i64).abs();
            assert!(skew <= (w.max(h) as i64), "{w}x{h} -> {cw}x{ch}");
        }
    }
}
