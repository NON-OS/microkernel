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

//! Whether bytes are a BMP the splash can draw, before any is served: the
//! file header ("BM", the file size u32 at 2, the pixel data's offset u32
//! at 10), then an info header of at least 40 bytes (its size u32 at 14,
//! the width i32 at 18, the height i32 at 22, negative for top-down rows,
//! one plane u16 at 26, 24 or 32 bits a pixel u16 at 28, no compression
//! u32 at 30). The rows, each padded to four bytes, must fit the file.
//! The BMP layout is recalled from Microsoft's published bitmap format,
//! not read: unverified.

pub const FILE_HEADER: usize = 14;
const INFO_MIN: u32 = 40;
/// Larger than any screen a logo is drawn on.
pub const MAX_SIDE: u32 = 8192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bmp {
    pub file_size: u32,
    pub pixels_at: u32,
    pub width: u32,
    pub height: u32,
    pub top_down: bool,
    pub bpp: u16,
}

/// `b` holds at least the headers; the file's own size is checked against
/// `b`'s length only when `whole` says `b` is the whole file.
pub fn bmp(b: &[u8], whole: bool) -> Option<Bmp> {
    let h = b.get(..FILE_HEADER + INFO_MIN as usize)?;
    let u32_at = |at: usize| u32::from_le_bytes([h[at], h[at + 1], h[at + 2], h[at + 3]]);
    let u16_at = |at: usize| u16::from_le_bytes([h[at], h[at + 1]]);
    let (file_size, pixels_at, info) = (u32_at(2), u32_at(10), u32_at(14));
    let (width, raw_height) = (u32_at(18) as i32, u32_at(22) as i32);
    let (planes, bpp, compression) = (u16_at(26), u16_at(28), u32_at(30));
    let height = raw_height.unsigned_abs();
    let ok = &h[..2] == b"BM"
        && info >= INFO_MIN
        && planes == 1
        && matches!(bpp, 24 | 32)
        && compression == 0
        && width > 0
        && (width as u32) <= MAX_SIDE
        && (1..=MAX_SIDE).contains(&height)
        && pixels_at >= FILE_HEADER as u32 + info;
    let row = (u64::from(width as u32) * u64::from(bpp) / 8).div_ceil(4) * 4;
    let end = u64::from(pixels_at) + row * u64::from(height);
    let fits = end <= u64::from(file_size) && (!whole || b.len() as u64 >= u64::from(file_size));
    (ok && fits).then_some(Bmp {
        file_size,
        pixels_at,
        width: width as u32,
        height,
        top_down: raw_height < 0,
        bpp,
    })
}
