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
//! The built-in picture as the wallpaper's worker paints it: ink, then the
//! picture over the whole surface; ink alone when the bytes do not decode.
//! Nothing past a row's last pixel is written.

use nonos_toolkit::image::scale::scale_cover;

use crate::wallpaper_decode::decode_jpeg;
use crate::wallpaper_first_paint::{paint, Backing};

const EMBEDDED: &[u8] = include_bytes!("../../../nonos-data/wallpapers/special-variant-9.jpg");
const INK: u32 = 0xFF0A_0B0D;
/// What the spare pixels past each row hold before and after.
const SPARE: u32 = 0x5A5A_5A5A;

/// A backing `w` by `h` with `extra` spare pixels a row, all SPARE.
fn backing(w: u32, h: u32, extra: u32) -> (Vec<u32>, Backing) {
    let mut px = vec![SPARE; ((w + extra) * h) as usize];
    let b = Backing { va: px.as_mut_ptr() as u64, stride: (w + extra) * 4, width: w, height: h };
    (px, b)
}

/// The pixel at (x, y) of `px` painted through `b`.
fn at(px: &[u32], b: &Backing, x: u32, y: u32) -> u32 {
    px[(y * b.stride / 4 + x) as usize]
}

fn spare_kept(px: &[u32], b: &Backing) -> bool {
    (0..b.height).all(|y| (b.width..b.stride / 4).all(|x| at(px, b, x, y) == SPARE))
}

#[test]
fn bytes_that_do_not_decode_leave_ink() {
    for bytes in [&b"not a picture"[..], &[], &EMBEDDED[..64]] {
        let (px, b) = backing(40, 24, 3);
        assert!(!paint(b, bytes), "no picture is said to be up");
        assert!((0..24).all(|y| (0..40).all(|x| at(&px, &b, x, y) == INK)));
        assert!(spare_kept(&px, &b));
    }
}

#[test]
fn the_built_in_picture_covers_the_surface() {
    let img = decode_jpeg(EMBEDDED).expect("the built-in picture decodes");
    for (w, h, extra) in [(160, 90, 5), (90, 160, 0), (33, 17, 1)] {
        let (px, b) = backing(w, h, extra);
        assert!(paint(b, EMBEDDED));
        let mut want = vec![0u32; (w * h) as usize];
        assert!(scale_cover(&img.pixels, img.width, img.height, w, h, |y, row| {
            want[(y * w) as usize..][..row.len()].copy_from_slice(row);
        }));
        for y in 0..h {
            for x in 0..w {
                assert_eq!(at(&px, &b, x, y), want[(y * w + x) as usize], "({x}, {y}) of {w}x{h}");
            }
        }
        assert!(spare_kept(&px, &b));
        assert!(want.iter().any(|&p| p != INK), "a picture, not ink");
    }
}
