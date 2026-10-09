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

/* Format detection and dimension probe from an image header, so the raster
 * buffer can be sized before the full decode runs. */

use super::bytes::{be32, le16, le_i32};
use super::jpeg::jpeg_dims;
use super::webp::webp_dims;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Webp,
}

pub(crate) struct Probe {
    pub format: Format,
    pub w: u32,
    pub h: u32,
}

pub(crate) fn probe(b: &[u8]) -> Option<Probe> {
    const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    if b.starts_with(&PNG) {
        return Some(Probe { format: Format::Png, w: be32(b, 16)?, h: be32(b, 20)? });
    }
    if b.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return jpeg_dims(b);
    }
    if b.starts_with(b"BM") {
        /* A negative width is invalid (probed as zero, which decode refuses);
         * a negative height marks a top-down image of that many rows. */
        let w = le_i32(b, 18)?.max(0) as u32;
        return Some(Probe { format: Format::Bmp, w, h: le_i32(b, 22)?.unsigned_abs() });
    }
    if b.starts_with(b"GIF87a") || b.starts_with(b"GIF89a") {
        return Some(Probe { format: Format::Gif, w: le16(b, 6)?, h: le16(b, 8)? });
    }
    if b.len() >= 16 && &b[0..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        return webp_dims(b);
    }
    None
}
