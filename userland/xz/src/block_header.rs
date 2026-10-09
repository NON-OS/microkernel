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

//! A block header (xz file format 3.1): its sizes, if declared, and the one
//! filter this decoder reads, LZMA2, with its dictionary size.

use super::crc32::matches;
use super::varint::varint;

const LZMA2: u64 = 0x21;

pub struct Header {
    pub size: usize,
    pub compressed: Option<u64>,
    pub uncompressed: Option<u64>,
    pub dict: u32,
}

pub fn header(d: &[u8]) -> Option<Header> {
    let size = (usize::from(*d.first()?) + 1) * 4;
    let h = d.get(..size)?;
    if !matches(&h[..size - 4], &h[size - 4..]) {
        return None;
    }
    let flags = h[1];
    // One filter only, and the reserved bits clear.
    if flags & 0x3F != 0 {
        return None;
    }
    let mut at = 2;
    let mut field = |present: bool| -> Option<Option<u64>> {
        if !present {
            return Some(None);
        }
        let (v, n) = varint(h.get(at..size - 4)?)?;
        at += n;
        Some(Some(v))
    };
    let compressed = field(flags & 0x40 != 0)?;
    let uncompressed = field(flags & 0x80 != 0)?;
    let (id, n) = varint(h.get(at..size - 4)?)?;
    let (props, m) = varint(h.get(at + n..size - 4)?)?;
    at += n + m;
    if id != LZMA2 || props != 1 {
        return None;
    }
    let bits = *h.get(at)?;
    if bits > 40 {
        return None;
    }
    let dict = match bits {
        40 => u32::MAX,
        b => (2 | u32::from(b & 1)) << (b / 2 + 11),
    };
    // What is left before the CRC is padding, and must be zero.
    h[at + 1..size - 4].iter().all(|&b| b == 0).then_some(Header {
        size,
        compressed,
        uncompressed,
        dict,
    })
}
