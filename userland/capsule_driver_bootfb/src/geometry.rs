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

//! The mode the kernel gives the holder (lead L31, libc
//! `mk_boot_framebuffer`, 24 bytes, repr(C) u32s): width, height, stride
//! in bytes, format (0 RGB, 1 BGR, 2 RGBX, 3 BGRX), offset of the first
//! pixel into bar 0, phys_mm (w low 16, h high 16, 0 unknown). Three-byte
//! pixels are not served; the kernel console keeps those.

use super::fb::Layout;

pub const INFO_LEN: usize = 24;
pub const RGB: u32 = 0;
pub const BGR: u32 = 1;
pub const RGBX: u32 = 2;
pub const BGRX: u32 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Geometry {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub offset: u32,
    pub physical_mm: u32,
}

/// The 24 bytes the call answers; None for a short answer.
pub fn decode(b: &[u8]) -> Option<Geometry> {
    let b = b.get(..INFO_LEN)?;
    let f = |i: usize| u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
    let (width, height, stride) = (f(0), f(1), f(2));
    Some(Geometry { width, height, stride, format: f(3), offset: f(4), physical_mm: f(5) })
}

/// The four-byte layouts the wire shows.
pub fn layout_of(format: u32) -> Option<Layout> {
    match format {
        BGRX => Some(Layout::BlueFirst),
        RGBX => Some(Layout::RedFirst),
        _ => None,
    }
}
