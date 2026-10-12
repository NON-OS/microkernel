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

//! The framebuffer record decoded as the kernel lays it out: the kernel's
//! own `FramebufferInfo`, written as bytes, read back field for field; the
//! pixel codes shared; three-byte pixels and an absent framebuffer refused.

use crate::boot_info::{decode, layout_of, Record, BGR, BGRX, INFO_LEN, RGB, RGBX};
use crate::fb::Layout;
use crate::handoff::constants::pixel_format;
use crate::handoff::framebuffer::FramebufferInfo;

fn bytes(f: &FramebufferInfo) -> [u8; INFO_LEN] {
    assert_eq!(core::mem::size_of::<FramebufferInfo>(), INFO_LEN);
    // SAFETY: a repr(C) struct of integers with no padding, read as bytes.
    unsafe { core::mem::transmute_copy(f) }
}

fn info(pixel_format: u32) -> FramebufferInfo {
    FramebufferInfo {
        ptr: 0x8000_0000,
        size: 8 << 20,
        width: 1920,
        height: 1080,
        stride: 7680,
        pixel_format,
        cursor_y: 40,
        phys_mm: 344 | (194 << 16),
    }
}

#[test]
fn the_kernels_record_reads_back_field_for_field() {
    let want = Record {
        phys: 0x8000_0000,
        len: 8 << 20,
        size: (1920, 1080),
        stride: 7680,
        layout: Layout::BlueFirst,
        physical_mm: 344 | (194 << 16),
    };
    assert_eq!(decode(&bytes(&info(BGRX))), Some(want));
    let red = decode(&bytes(&info(RGBX))).map(|b| b.layout);
    assert_eq!(red, Some(Layout::RedFirst));
}

#[test]
fn the_pixel_codes_are_the_kernels_and_only_four_byte_pixels_are_served() {
    let k = (pixel_format::RGB, pixel_format::BGR, pixel_format::RGBX, pixel_format::BGRX);
    assert_eq!(k, (RGB, BGR, RGBX, BGRX));
    assert_eq!((layout_of(RGB), layout_of(BGR), layout_of(9)), (None, None, None));
    assert_eq!(decode(&bytes(&info(BGR))), None);
}

#[test]
fn no_framebuffer_or_a_short_record_is_refused() {
    assert_eq!(decode(&bytes(&FramebufferInfo::default())), None);
    let mut f = info(BGRX);
    f.stride = 0;
    assert_eq!(decode(&bytes(&f)), None);
    assert_eq!(decode(&bytes(&info(BGRX))[..INFO_LEN - 1]), None);
}
