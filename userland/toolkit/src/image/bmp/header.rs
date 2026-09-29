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

use crate::image::types::{DecodeError, ImageSize};

use super::le::{le_u16, le_u32};
use super::masks::masks;

/* Where a BMP's pixels are and how to read them. Rows are `stride` bytes,
 * 4-byte aligned, bottom-up unless the stored height was negative. */
pub(super) struct Layout<'a> {
    pub size: ImageSize,
    pub top_down: bool,
    pub bpp: usize,
    pub data_off: usize,
    pub stride: usize,
    pub masks: [u32; 4],
    pub palette: &'a [u8],
}

pub fn bmp_dimensions(input: &[u8]) -> Result<ImageSize, DecodeError> {
    Ok(parse(input)?.size)
}

/* A file header and a DIB header of a form `readable` accepts. A negative
 * height is a top-down image; a negative width or a plane count other than
 * one is not a valid bitmap. */
pub(super) fn parse(input: &[u8]) -> Result<Layout<'_>, DecodeError> {
    if input.get(0..2) != Some(b"BM") {
        return Err(DecodeError::BadMagic);
    }
    let data_off = le_u32(input, 10)? as usize;
    let dib = le_u32(input, 14)? as usize;
    let (bpp, compression) = (le_u16(input, 28)? as usize, le_u32(input, 30)?);
    super::readable(dib, compression, bpp)?;
    let w = le_u32(input, 18)? as i32;
    let h = le_u32(input, 22)? as i32;
    if le_u16(input, 26)? != 1 {
        return Err(DecodeError::BadMagic);
    }
    if w < 0 {
        return Err(DecodeError::BadDimensions);
    }
    let size = ImageSize::new(w as u32, h.unsigned_abs())?;
    let masks = masks(input, dib, compression, bpp)?;
    let palette = if bpp <= 8 {
        let used = le_u32(input, 46)? as usize;
        let n = if used == 0 || used > 1 << bpp { 1 << bpp } else { used };
        let at = 14 + dib;
        input.get(at..at + 4 * n).ok_or(DecodeError::Truncated)?
    } else {
        &[]
    };
    let stride = (w as usize).saturating_mul(bpp).saturating_add(31) / 32 * 4;
    Ok(Layout { size, top_down: h < 0, bpp, data_off, stride, masks, palette })
}
