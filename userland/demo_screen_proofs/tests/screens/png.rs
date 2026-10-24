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

//! A picture as PNG in stored blocks, written when DEMO_SHOTS names a directory.

use std::io::Write;

fn crc(bytes: &[u8]) -> u32 {
    let mut c = !0u32;
    for &b in bytes {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
        }
    }
    !c
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let c = crc(&out[start..]);
    out.extend_from_slice(&c.to_be_bytes());
}

/// zlib of `raw` in stored blocks of at most 65535 bytes.
fn stored(raw: &[u8]) -> Vec<u8> {
    let mut z = vec![0x78, 0x01];
    let blocks: Vec<&[u8]> = raw.chunks(65_535).collect();
    for (i, b) in blocks.iter().enumerate() {
        z.push((i + 1 == blocks.len()) as u8);
        z.extend_from_slice(&(b.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(b.len() as u16)).to_le_bytes());
        z.extend_from_slice(b);
    }
    let (mut a, mut s) = (1u32, 0u32);
    raw.iter()
        .for_each(|&x| (a, s) = ((a + x as u32) % 65_521, (s + (a + x as u32) % 65_521) % 65_521));
    z.extend_from_slice(&((s << 16) | a).to_be_bytes());
    z
}

/// Write ARGB `pixels`, `w` wide, as `name`.png under DEMO_SHOTS.
pub fn save(name: &str, pixels: &[u32], w: u32) {
    let Ok(dir) = std::env::var("DEMO_SHOTS") else { return };
    let h = pixels.len() as u32 / w;
    let mut raw = Vec::with_capacity(pixels.len() * 3 + h as usize);
    for row in pixels.chunks(w as usize) {
        raw.push(0);
        row.iter().for_each(|p| raw.extend_from_slice(&p.to_be_bytes()[1..]));
    }
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = [w.to_be_bytes(), h.to_be_bytes()].concat();
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &stored(&raw));
    chunk(&mut out, b"IEND", &[]);
    let path = format!("{dir}/{name}.png");
    std::fs::File::create(path).and_then(|mut f| f.write_all(&out)).expect("written");
}
