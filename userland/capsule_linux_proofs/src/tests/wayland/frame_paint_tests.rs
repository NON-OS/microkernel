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

//! The frame painted, with the guest's pixels over its ground.

use crate::wayland_frame::geometry::around;
use crate::wayland_frame::paint::{over, paint, Look};
use nonos_toolkit::decorations::DecorationHit;

#[test]
fn premultiplied_over_keeps_opaque_drops_clear_and_mixes_half() {
    assert_eq!(over(0xFF11_2233, 0xFF99_8877), 0xFF11_2233);
    assert_eq!(over(0x0000_0000, 0xFF99_8877), 0xFF99_8877);
    let half = over(0x8040_4040, 0xFF00_0000);
    assert_eq!(half >> 24, 0xFF);
    assert!((0x3F..=0x41).contains(&((half >> 16) & 0xFF)));
}

#[test]
fn a_clear_guest_shows_the_frames_ground_and_its_pixels_land_inside() {
    let (cw, ch) = (40, 20);
    let mut client = vec![0u8; cw * ch * 4];
    client[..4].copy_from_slice(&0xFF12_3456u32.to_le_bytes());
    let a = around(cw as u32, ch as u32, 4, false);
    let mut out = vec![0u32; (a.w * a.h) as usize];
    // SAFETY: a u32 buffer viewed as bytes, the way the capsule's frame is.
    let bytes =
        unsafe { core::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, out.len() * 4) };
    let look = Look { title: b"swayimg", hover: DecorationHit::None, focused: true };
    assert!(paint(bytes, a, &client, (cw as u32, ch as u32, cw as u32 * 4), (&look, true), 4));
    let at = |x: u32, y: u32| out[(y * a.w + x) as usize];
    assert_eq!(at(a.cx, a.cy), 0xFF12_3456, "the guest's first pixel");
    let ground = at(a.cx + 10, a.cy + 10);
    assert_eq!(ground >> 24, 0xFF, "opaque where the guest drew nothing");
    assert_eq!(at(0, 0) >> 24, 0, "the shadow's corner is clear");
}

#[test]
fn a_buffer_too_short_for_the_frame_is_refused() {
    let a = around(40, 20, 4, false);
    let mut out = vec![0u8; 16];
    let look = Look { title: b"", hover: DecorationHit::None, focused: false };
    assert!(!paint(&mut out, a, &[0u8; 3200], (40, 20, 160), (&look, true), 4));
}

#[test]
fn a_frame_left_as_painted_still_gets_the_whole_content_again() {
    let (cw, ch) = (8u32, 4u32);
    let a = around(cw, ch, 4, false);
    let mut out = vec![0u32; (a.w * a.h) as usize];
    // SAFETY: as above.
    let bytes =
        unsafe { core::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, out.len() * 4) };
    let look = Look { title: b"t", hover: DecorationHit::None, focused: false };
    let red = 0xFFFF_0000u32.to_le_bytes().repeat((cw * ch) as usize);
    assert!(paint(bytes, a, &red, (cw, ch, cw * 4), (&look, true), 4));
    let clear = vec![0u8; (cw * ch * 4) as usize];
    assert!(paint(bytes, a, &clear, (cw, ch, cw * 4), (&look, false), 4));
    let px = out[(a.cy * a.w + a.cx) as usize];
    assert_ne!(px, 0xFFFF_0000, "the last frame's pixel is not kept under a clear one");
    assert_eq!(px >> 24, 0xFF, "the ground is opaque");
}
