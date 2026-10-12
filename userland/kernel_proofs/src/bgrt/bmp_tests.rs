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

//! The BMP check (the layout recalled, unverified): drawable files kept,
//! and each way a header can be wrong or a file short refused.

use super::bmp_check::{bmp, Bmp, MAX_SIDE};

/// A `w` × `h` BMP at `bpp`, rows padded to four bytes.
fn file(w: i32, h: i32, bpp: u16) -> Vec<u8> {
    let row = (w.unsigned_abs() as usize * bpp as usize / 8).div_ceil(4) * 4;
    let size = 54 + row * h.unsigned_abs() as usize;
    let mut f = b"BM".to_vec();
    f.extend_from_slice(&(size as u32).to_le_bytes());
    f.extend_from_slice(&[0; 4]);
    f.extend_from_slice(&54u32.to_le_bytes());
    f.extend_from_slice(&40u32.to_le_bytes());
    f.extend_from_slice(&w.to_le_bytes());
    f.extend_from_slice(&h.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&bpp.to_le_bytes());
    f.resize(size, 0);
    f
}

#[test]
fn drawable_files() {
    let f = file(3, 2, 24);
    let want = Bmp { file_size: 78, pixels_at: 54, width: 3, height: 2, top_down: false, bpp: 24 };
    assert_eq!(bmp(&f, true), Some(want), "rows of 9 bytes padded to 12");
    let m = bmp(&file(5, -4, 32), true).unwrap();
    assert!(m.top_down && m.height == 4 && m.bpp == 32);
    assert_eq!(bmp(&f[..54], false), Some(want), "the header alone, before the rest is read");
}

#[test]
fn refused_files() {
    let bad = |at: usize, v: &[u8]| {
        let mut f = file(3, 2, 24);
        f[at..at + v.len()].copy_from_slice(v);
        bmp(&f, true)
    };
    assert_eq!(bad(0, b"BA"), None, "magic");
    assert_eq!(bad(30, &[1]), None, "compressed");
    assert_eq!(bad(28, &[8]), None, "8 bits a pixel");
    assert_eq!(bad(26, &[2]), None, "two planes");
    assert_eq!(bad(14, &[12]), None, "an old core header");
    assert_eq!(bad(10, &[20]), None, "pixels inside the headers");
    assert_eq!(bad(2, &[60]), None, "rows past the file's size");
    assert_eq!(bmp(&file(0, 2, 24), true), None, "no width");
    assert_eq!(bmp(&file(3, 0, 24), true), None, "no height");
    let wide = MAX_SIDE as i32 + 1;
    assert_eq!(bad(18, &wide.to_le_bytes()), None, "wider than any screen");
    let f = file(3, 2, 24);
    assert_eq!(bmp(&f[..60], true), None, "a whole file shorter than it says");
    assert_eq!(bmp(&f[..40], false), None, "not even the headers");
}
