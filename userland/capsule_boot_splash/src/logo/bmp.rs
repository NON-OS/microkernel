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

//! The logo file, read again here although the kernel checked it: the splash
//! indexes into it, so every bound it relies on is its own. Uncompressed
//! (BI_RGB) 24 or 32 bits per pixel, at most 8192 on a side, rows padded to
//! four bytes, bottom-up unless the height is negative (Microsoft's
//! BITMAPFILEHEADER and BITMAPINFOHEADER).

pub const MAX_SIDE: u32 = 8192;
const FILE_HEADER: usize = 14;
const INFO_HEADER: u32 = 40;

#[derive(Clone, Copy, Debug)]
pub struct Bmp<'a> {
    pub width: u32,
    pub height: u32,
    pub bpp: u16,
    pub top_down: bool,
    pub offset: u32,
    row: usize,
    file: &'a [u8],
}

impl<'a> Bmp<'a> {
    pub fn parse(file: &'a [u8]) -> Option<Bmp<'a>> {
        let u16_at = |o: usize| Some(u16::from_le_bytes(file.get(o..o + 2)?.try_into().ok()?));
        let u32_at = |o: usize| Some(u32::from_le_bytes(file.get(o..o + 4)?.try_into().ok()?));
        if file.get(..2)? != b"BM" || u32_at(FILE_HEADER)? < INFO_HEADER {
            return None;
        }
        let (width, raw_h) = (u32_at(18)? as i32, u32_at(22)? as i32);
        let (bpp, offset) = (u16_at(28)?, u32_at(10)?);
        let height = raw_h.unsigned_abs();
        let sides = width > 0 && height > 0 && width as u32 <= MAX_SIDE && height <= MAX_SIDE;
        if !sides || u16_at(26)? != 1 || !matches!(bpp, 24 | 32) || u32_at(30)? != 0 {
            return None;
        }
        let width = width as u32;
        let row = (width as usize * bpp as usize).div_ceil(32) * 4;
        let last = width as usize * (bpp as usize / 8);
        let need = (offset as usize).checked_add(row * (height as usize - 1) + last)?;
        if (offset as usize) < FILE_HEADER + INFO_HEADER as usize || need > file.len() {
            return None;
        }
        Some(Bmp { width, height, bpp, top_down: raw_h < 0, offset, row, file })
    }

    /// The pixel `x` across and `y` down from the image's top left, as
    /// opaque 0xAARRGGBB. The caller keeps x and y inside the image.
    pub fn at(&self, x: u32, y: u32) -> u32 {
        let r = if self.top_down { y } else { self.height - 1 - y } as usize;
        let p = self.offset as usize + r * self.row + x as usize * (self.bpp as usize / 8);
        let px = &self.file[p..p + 3];
        0xFF00_0000 | ((px[2] as u32) << 16) | ((px[1] as u32) << 8) | px[0] as u32
    }
}
