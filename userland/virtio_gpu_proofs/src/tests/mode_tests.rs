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

//! The mode a scanout is taken at, and the surface made for it. The size is
//! the device's word: a huge one (32767x32767, about 4 GiB) used to be
//! taken, and the primary surface's DMA map then failed every bring-up
//! attempt instead of falling back. A mode is now taken only when its frame
//! fits the broker's display grant. A small one (a 320x240 panel) is the
//! device's too: raising it to 1280x720 drew a frame the panel cannot show.

use crate::setup::geometry::derive;
use crate::setup::mode::{fit, DEFAULT, MAX_SURFACE_BYTES};
use crate::state::Scanout;

fn scanout(width: u32, height: u32) -> Scanout {
    Scanout { x: 0, y: 0, width, height, current_resource_id: 0, enabled: true }
}

#[test]
fn a_mode_whose_frame_fits_the_display_grant_is_taken_as_offered() {
    let small = [(16, 16), (320, 240), (639, 480), (640, 479), (16, 4096)];
    let large = [(640, 480), (1024, 768), (1920, 1080), (3840, 2160), (2048, 4096)];
    for (w, h) in small.into_iter().chain(large) {
        assert_eq!(fit(w, h), (w, h), "{w}x{h}");
    }
}

#[test]
fn a_mode_too_small_or_too_large_falls_back_to_the_default() {
    let huge = [(32767, 32767), (3840, 2400), (8192, 8192), (u32::MAX, u32::MAX), (u32::MAX, 1)];
    let small = [(0, 0), (15, 16), (16, 15), (1, 1), (1, u32::MAX)];
    for (w, h) in huge.into_iter().chain(small) {
        assert_eq!(fit(w, h), DEFAULT, "{w}x{h}");
    }
}

#[test]
fn a_surface_past_the_display_grant_is_refused_not_asked_of_the_broker() {
    let g = derive(scanout(3840, 2160)).expect("fits").expect("enabled");
    assert!(g.byte_len <= MAX_SURFACE_BYTES);
    for (w, h) in [(32767, 32767), (3840, 2400), (65536, 65535)] {
        assert!(derive(scanout(w, h)).is_err(), "{w}x{h}");
    }
    assert!(derive(scanout(u32::MAX, 1)).is_err(), "the stride overflows");
    assert!(derive(Scanout { enabled: false, ..scanout(1024, 768) }).expect("ok").is_none());
}
