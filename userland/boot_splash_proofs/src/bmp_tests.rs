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

//! The file read again by the splash: every pixel of 24- and 32-bit files,
//! top-down and bottom-up, with row padding; and every file the splash must
//! not index into refused.

use crate::build::{bmp, colour};
use crate::logo::bmp::Bmp;

#[test]
fn every_pixel_comes_back_in_any_layout() {
    for (w, h) in [(1, 1), (3, 2), (5, 7)] {
        for bpp in [24, 32] {
            for top_down in [false, true] {
                let f = bmp(w, h, bpp, top_down);
                let b = Bmp::parse(&f).unwrap();
                assert_eq!((b.width, b.height, b.bpp, b.top_down), (w, h, bpp, top_down));
                for (x, y) in (0..h).flat_map(|y| (0..w).map(move |x| (x, y))) {
                    assert_eq!(b.at(x, y), colour(x, y), "{w}x{h} {bpp} bpp at {x},{y}");
                }
            }
        }
    }
}

#[test]
fn files_the_splash_would_misread_are_refused() {
    let good = bmp(4, 3, 24, false);
    let with = |o: usize, v: &[u8]| {
        let mut f = good.clone();
        f[o..o + v.len()].copy_from_slice(v);
        f
    };
    assert!(Bmp::parse(&good).is_some());
    assert!(Bmp::parse(&with(0, b"MB")).is_none(), "magic");
    assert!(Bmp::parse(&with(14, &12u32.to_le_bytes())).is_none(), "old core header");
    assert!(Bmp::parse(&with(18, &0i32.to_le_bytes())).is_none(), "no width");
    assert!(Bmp::parse(&with(18, &(-4i32).to_le_bytes())).is_none(), "negative width");
    assert!(Bmp::parse(&with(22, &0i32.to_le_bytes())).is_none(), "no height");
    assert!(Bmp::parse(&with(18, &8193i32.to_le_bytes())).is_none(), "past 8192");
    assert!(Bmp::parse(&with(26, &2u16.to_le_bytes())).is_none(), "planes");
    assert!(Bmp::parse(&with(28, &16u16.to_le_bytes())).is_none(), "16 bpp");
    assert!(Bmp::parse(&with(30, &1u32.to_le_bytes())).is_none(), "RLE");
    assert!(Bmp::parse(&with(10, &20u32.to_le_bytes())).is_none(), "pixels inside the header");
    assert!(Bmp::parse(&with(10, &u32::MAX.to_le_bytes())).is_none(), "offset past the end");
    assert!(Bmp::parse(&good[..good.len() - 1]).is_none(), "a byte short");
    assert!(Bmp::parse(&good[..40]).is_none(), "headers cut");
}

#[test]
fn the_last_row_needs_no_padding() {
    // 3 pixels of 24 bits: 9 bytes a row, padded to 12; the file may end at 9.
    let f = bmp(3, 2, 24, true);
    let b = Bmp::parse(&f[..f.len() - 3]).unwrap();
    assert_eq!(b.at(2, 1), colour(2, 1));
}
