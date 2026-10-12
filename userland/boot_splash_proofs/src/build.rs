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

//! Logo files and records built for the proofs, written from the BMP layout
//! the splash reads (Microsoft's BITMAPFILEHEADER and BITMAPINFOHEADER), so a
//! test can name every byte it hands the splash.

/// The colour the proofs paint at (x, y): every pixel different.
pub fn colour(x: u32, y: u32) -> u32 {
    0xFF00_0000 | ((x * 37 % 251) << 16) | ((y * 53 % 241) << 8) | ((x + y) % 239)
}

/// A w by h BMP of `colour`, 24 or 32 bits, rows top-down or bottom-up.
pub fn bmp(w: u32, h: u32, bpp: u16, top_down: bool) -> Vec<u8> {
    let row = (w as usize * bpp as usize).div_ceil(32) * 4;
    let mut f = vec![0u8; 54 + row * h as usize];
    let len = f.len() as u32;
    f[..2].copy_from_slice(b"BM");
    f[2..6].copy_from_slice(&len.to_le_bytes());
    f[10..14].copy_from_slice(&54u32.to_le_bytes());
    f[14..18].copy_from_slice(&40u32.to_le_bytes());
    f[18..22].copy_from_slice(&(w as i32).to_le_bytes());
    let height = if top_down { -(h as i32) } else { h as i32 };
    f[22..26].copy_from_slice(&height.to_le_bytes());
    f[26..28].copy_from_slice(&1u16.to_le_bytes());
    f[28..30].copy_from_slice(&bpp.to_le_bytes());
    for y in 0..h {
        let r = if top_down { y } else { h - 1 - y } as usize;
        for x in 0..w {
            let p = 54 + r * row + x as usize * (bpp as usize / 8);
            let c = colour(x, y).to_le_bytes();
            f[p..p + 3].copy_from_slice(&c[..3]);
        }
    }
    f
}

/// The record MkBootLogo would give for `file` at (x, y) turned `turns`.
pub fn record(x: u32, y: u32, file: &[u8], flags: u8, turns: u8) -> [u8; 28] {
    let mut r = [0u8; 28];
    let w = u32::from_le_bytes(file[18..22].try_into().unwrap());
    let h = i32::from_le_bytes(file[22..26].try_into().unwrap()).unsigned_abs();
    for (o, v) in [(0, x), (4, y), (8, w), (12, h), (20, file.len() as u32), (24, 54)] {
        r[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    r[16] = flags;
    r[17] = turns;
    r[18..20].copy_from_slice(&file[28..30]);
    r
}
