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

//! GIF files assembled in the test. The LZW stream codes every index as a
//! 9-bit literal and clears the table before it would widen, which any
//! conforming decoder must read back index for index.
use std::vec::Vec;

fn lzw(indices: &[u8]) -> Vec<u8> {
    let (mut out, mut acc, mut bits) = (Vec::new(), 0u32, 0u32);
    let mut put = |code: u32| {
        acc |= code << bits;
        bits += 9;
        while bits >= 8 {
            out.push(acc as u8);
            (acc, bits) = (acc >> 8, bits - 8);
        }
    };
    for chunk in indices.chunks(254) {
        put(256);
        chunk.iter().for_each(|&i| put(i as u32));
    }
    put(257);
    put(0);
    out
}

/* A GIF89a with a 256-entry global palette (entry i is 0x0i, 0x20+i, 0xff-i
 * in its low bytes), an optional transparent index, and one frame at
 * [left, top, w, h] whose indices arrive in stream order. */
pub fn gif(
    screen: (u16, u16),
    frame: [u16; 4],
    interlace: bool,
    idx: &[u8],
    transparent: Option<u8>,
) -> Vec<u8> {
    let mut g = std::vec::Vec::from(&b"GIF89a"[..]);
    g.extend_from_slice(&screen.0.to_le_bytes());
    g.extend_from_slice(&screen.1.to_le_bytes());
    g.extend_from_slice(&[0xF7, 0, 0]);
    (0..=255u8).for_each(|i| g.extend_from_slice(&[i, i.wrapping_add(0x20), 255 - i]));
    if let Some(t) = transparent {
        g.extend_from_slice(&[0x21, 0xF9, 4, 1, 0, 0, t, 0]);
    }
    g.push(0x2C);
    frame.iter().for_each(|v| g.extend_from_slice(&v.to_le_bytes()));
    g.extend_from_slice(&[if interlace { 0x40 } else { 0 }, 8]);
    for block in lzw(idx).chunks(255) {
        g.push(block.len() as u8);
        g.extend_from_slice(block);
    }
    g.extend_from_slice(&[0, 0x3B]);
    g
}

pub fn color(i: u8) -> u32 {
    0xff00_0000 | (i as u32) << 16 | (i.wrapping_add(0x20) as u32) << 8 | (255 - i) as u32
}
