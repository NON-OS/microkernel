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

//! PNG files assembled in the test: chunks with real CRCs around a zlib
//! stream of stored (uncompressed) deflate blocks, so a case can say exactly
//! which scanline bytes the decoder sees and how IDAT is split.
use std::vec::Vec;

use crate::png_crc::crc32;

pub fn chunk(out: &mut Vec<u8>, tag: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(tag);
    out.extend_from_slice(data);
    let crc = crc32(&out[start..]);
    out.extend_from_slice(&crc.to_be_bytes());
}

/* A zlib stream holding `raw` in stored blocks of at most `block` bytes. */
pub fn zlib_stored(raw: &[u8], block: usize) -> Vec<u8> {
    let mut z = std::vec![0x78, 0x01];
    let parts: Vec<&[u8]> = raw.chunks(block.max(1)).collect();
    for (i, part) in parts.iter().enumerate() {
        z.push((i + 1 == parts.len()) as u8);
        z.extend_from_slice(&(part.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(part.len() as u16)).to_le_bytes());
        z.extend_from_slice(part);
    }
    z.extend_from_slice(&[0, 0, 0, 0]);
    z
}

/* IHDR (w, h, depth, color type, interlace), any extra chunks before the
 * image data, then the filtered scanlines as IDAT chunks of `split` bytes. */
pub fn png(
    ihdr: (u32, u32, u8, u8, u8),
    extra: &[(&[u8; 4], &[u8])],
    raw: &[u8],
    split: usize,
) -> Vec<u8> {
    let mut out = std::vec![137, 80, 78, 71, 13, 10, 26, 10];
    let mut h = Vec::new();
    h.extend_from_slice(&ihdr.0.to_be_bytes());
    h.extend_from_slice(&ihdr.1.to_be_bytes());
    h.extend_from_slice(&[ihdr.2, ihdr.3, 0, 0, ihdr.4]);
    chunk(&mut out, b"IHDR", &h);
    for (tag, data) in extra {
        chunk(&mut out, tag, data);
    }
    for part in zlib_stored(raw, 1000).chunks(split.max(1)) {
        chunk(&mut out, b"IDAT", part);
    }
    chunk(&mut out, b"IEND", &[]);
    out
}
