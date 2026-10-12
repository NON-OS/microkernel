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

//! The shadow-to-VRAM copy.

use crate::copy::copy_rects;
use nonos_gfx_wire::Rect;

#[test]
fn rows_are_copied_across_different_strides_and_red_blue_swap_on_request() {
    let src: Vec<u8> = (0..8 * 4 * 4).map(|i| i as u8).collect();
    let mut dst = vec![0u32; 16 * 4];
    let r = [Rect { x: 1, y: 1, width: 2, height: 2 }];
    // SAFETY: dst is a live aligned Vec of the length passed.
    unsafe { copy_rects(&src, 32, dst.as_mut_ptr() as *mut u8, 256, 64, &r, false) };
    assert_eq!(dst[16 + 1], u32::from_le_bytes([36, 37, 38, 39]));
    assert_eq!(dst[32 + 2], u32::from_le_bytes([72, 73, 74, 75]));
    assert_eq!((dst[16], dst[16 + 3], dst[48 + 1]), (0, 0, 0));
    let mut sw = vec![0u32; 16 * 4];
    // SAFETY: as above.
    unsafe { copy_rects(&src, 32, sw.as_mut_ptr() as *mut u8, 256, 64, &r, true) };
    assert_eq!(sw[16 + 1], u32::from_le_bytes([38, 37, 36, 39]));
}

#[test]
fn a_rectangle_past_either_buffer_is_skipped_whole() {
    let src = vec![0xAAu8; 4 * 4 * 4];
    let mut dst = vec![0u32; 4 * 4];
    let r = [Rect { x: 2, y: 3, width: 3, height: 1 }, Rect { x: 0, y: 0, width: 1, height: 1 }];
    // SAFETY: as above.
    unsafe { copy_rects(&src, 16, dst.as_mut_ptr() as *mut u8, 64, 16, &r, false) };
    assert_eq!(dst[0], 0xAAAA_AAAA);
    assert!(dst[1..].iter().all(|&p| p == 0));
}
