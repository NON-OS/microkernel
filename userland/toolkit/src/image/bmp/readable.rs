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

use crate::image::types::DecodeError;

/* The BMP forms this decoder reads: a BITMAPINFOHEADER or later (V2 to V5)
 * DIB header with BI_RGB at 1, 4, 8, 16, 24 or 32 bits per pixel, or
 * BI_BITFIELDS / BI_ALPHABITFIELDS at 16 or 32. OS/2 core headers, RLE and
 * embedded JPEG or PNG are Unsupported. */
pub(super) fn readable(dib: usize, compression: u32, bpp: usize) -> Result<(), DecodeError> {
    match (dib >= 40, compression, bpp) {
        (true, 0, 1 | 4 | 8 | 16 | 24 | 32) | (true, 3 | 6, 16 | 32) => Ok(()),
        _ => Err(DecodeError::Unsupported),
    }
}
