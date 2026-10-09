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

//! GIF: interlaced rows land where the four-pass order puts them, frames are
//! clipped to the logical screen, and the inputs that once took a second
//! (a tall interlaced frame on a tiny screen) now take no time at all.
use nonos_toolkit::image::gif::decode_gif_argb8888;
use std::vec::Vec;

use crate::gif_build::{color, gif};

/* Frame rows in the order an interlaced stream carries them (GIF89a app. E). */
fn stream_order(h: usize) -> Vec<usize> {
    [(0, 8), (4, 8), (2, 4), (1, 2)].iter().flat_map(|&(s, st)| (s..h).step_by(st)).collect()
}

#[test]
fn interlaced_rows_land_in_pass_order_for_every_height() {
    for h in 1..=41usize {
        let file =
            gif((1, h as u16), [0, 0, 1, h as u16], true, &(0..h as u8).collect::<Vec<_>>(), None);
        let mut out = std::vec![0u32; h];
        decode_gif_argb8888(&file, &mut out).unwrap();
        for (k, y) in stream_order(h).into_iter().enumerate() {
            assert_eq!(out[y], color(k as u8), "height {h}, stream row {k} -> row {y}");
        }
    }
}

#[test]
fn a_frame_is_clipped_to_the_screen() {
    let idx: Vec<u8> = (0..36).collect();
    let mut out = std::vec![7u32; 16];
    decode_gif_argb8888(&gif((4, 4), [2, 2, 6, 6], false, &idx, Some(1)), &mut out).unwrap();
    for (i, &p) in out.iter().enumerate() {
        let (x, y) = (i % 4, i / 4);
        let at = if x >= 2 && y >= 2 { Some(((y - 2) * 6 + x - 2) as u8) } else { None };
        /* Index 1 is the transparent one, so it leaves the cleared screen. */
        let want = at.filter(|&i| i != 1).map_or(0, color);
        assert_eq!(p, want, "pixel {x},{y}");
    }
    /* Rows the screen shows but the stream never delivers are an error. */
    let short = gif((4, 4), [0, 0, 4, 4], false, &idx[..9], None);
    assert!(decode_gif_argb8888(&short, &mut out).is_err());
}
