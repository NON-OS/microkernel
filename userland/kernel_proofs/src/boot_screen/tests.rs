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

use super::early_frame::{identity_frame_ok, IDENTITY_FB_LIMIT};
use super::fb_window;

#[test]
fn the_window_is_the_loaders() {
    assert_eq!(IDENTITY_FB_LIMIT, fb_window::IDENTITY_FB_LIMIT);
}

#[test]
fn a_laptop_panel_below_4_gib_is_drawn() {
    // 1366x768 at a 1376 pixel pitch, the shape that made the pitch matter.
    assert!(identity_frame_ok(0x8000_0000, 1376 * 4 * 768, 1366, 768, 1376 * 4));
}

#[test]
fn a_resizable_bar_framebuffer_past_1_tib_is_drawn() {
    let ptr = 0x100_0000_0000;
    assert!(identity_frame_ok(ptr, 0x1000_0000, 3840, 2160, 3840 * 4));
    assert!(fb_window::framebuffer_identity_reachable(ptr, 3840 * 4 * 2160));
}

#[test]
fn whatever_is_drawn_the_loader_mapped() {
    // Every accepted frame ends inside the window the loader identity maps,
    // and the loader maps exactly stride times height from the base.
    let cases = [
        (0xC000_0000u64, 1920 * 4 * 1080u64, 1920u32, 1080u32, 1920 * 4u32),
        (0x7FFF_FFFF_0000, 0x1_0000, 64, 64, 256),
        (0x7FFF_FFFF_0000, 0x2_0000, 64, 129, 1024),
        (0x40_0000_0000, 2560 * 4 * 1600, 2560, 1600, 2560 * 4),
    ];
    for (ptr, size, w, h, stride) in cases {
        if identity_frame_ok(ptr, size, w, h, stride) {
            let frame = stride as u64 * h as u64;
            assert!(fb_window::framebuffer_identity_reachable(ptr, frame), "{ptr:#x}");
        }
    }
    assert!(identity_frame_ok(0x7FFF_FFFF_0000, 0x1_0000, 64, 64, 256));
    assert!(!identity_frame_ok(0x7FFF_FFFF_0000, 0x2_0000, 64, 129, 1024));
}

#[test]
fn a_frame_the_handoff_cannot_back_is_refused() {
    assert!(!identity_frame_ok(0, 4096, 1, 1, 4));
    assert!(!identity_frame_ok(0x8000_0000, 4096, 0, 1, 4));
    assert!(!identity_frame_ok(0x8000_0000, 4096, 1, 0, 4));
    // A pitch shorter than a row of 4-byte pixels.
    assert!(!identity_frame_ok(0x8000_0000, 1 << 20, 1024, 4, 1024 * 3));
    // FrameBufferSize smaller than pitch times height.
    assert!(!identity_frame_ok(0x8000_0000, 1024 * 4 * 767, 1024, 768, 1024 * 4));
    // A frame running past the end of the address space.
    assert!(!identity_frame_ok(u64::MAX - 4095, 1 << 20, 1024, 4, 4096));
}
