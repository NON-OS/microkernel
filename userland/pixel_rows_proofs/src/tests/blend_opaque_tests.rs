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

//! The vector blends take a block whose upper pixels are all opaque as
//! those pixels, with no arithmetic. Random rows almost never hold such a
//! block, so these rows are opaque but for a pixel here and there: whole
//! opaque blocks, blocks with one translucent pixel, and runs across both.

use crate::blend;
use crate::reference::{lay_row, paint_over, paint_row};
use crate::rng::Rng;

use super::paths::kernels;

/// `n` pixels, each opaque but where the draw says one in `odds` is not.
fn mostly_opaque(r: &mut Rng, n: usize, odds: u64) -> Vec<u8> {
    let mut px = r.pixels(n);
    for p in px.chunks_exact_mut(4) {
        if !r.next().is_multiple_of(odds) {
            p[3] = 0xFF;
        }
    }
    px
}

/// Rows to 70 pixels at every byte offset, opaque but for one pixel in
/// 1, 9, 40 or never.
fn each_row(mut f: impl FnMut(&mut [u8], &[u8], u32)) {
    let mut r = Rng::new(0x0FA0);
    for odds in [1, 9, 40, u64::MAX] {
        for len in 0..70 {
            for off in 0..4 {
                let src = mostly_opaque(&mut r, len + 1, odds);
                let mut dst = mostly_opaque(&mut r, len + 1, odds);
                let colour = r.next() as u32 | 0xFF00_0000;
                f(&mut dst[off..off + len * 4], &src[off..off + len * 4], colour);
            }
        }
    }
}

#[test]
fn every_path_lays_opaque_blocks_over_and_under_as_the_old_loop_did() {
    for (name, k) in kernels() {
        each_row(|dst, src, c| {
            let (mut over, mut under) = (dst.to_vec(), dst.to_vec());
            lay_row(&mut over, src, true);
            lay_row(&mut under, src, false);
            let mut d = dst.to_vec();
            k(&mut d, src, c, blend::SRC_OVER);
            assert_eq!(d, over, "{name} over");
            k(dst, src, c, blend::SRC_UNDER);
            assert_eq!(dst, &under[..], "{name} under");
        });
    }
}

#[test]
fn every_path_flattens_opaque_blocks_as_the_paint_loop_did() {
    for (name, k) in kernels() {
        each_row(|dst, src, c| {
            let mut want = vec![0u32; src.len() / 4];
            paint_row(&mut want, src, c);
            let want: Vec<u8> = want.iter().flat_map(|p| p.to_le_bytes()).collect();
            let place: Vec<u8> = dst
                .chunks_exact(4)
                .flat_map(|p| paint_over(u32::from_le_bytes([p[0], p[1], p[2], p[3]]), c).to_le_bytes())
                .collect();
            let mut d = dst.to_vec();
            k(&mut d, src, c, blend::OVER_COLOUR);
            assert_eq!(d, want, "{name} over a colour");
            k(dst, &[], c, blend::FLATTEN);
            assert_eq!(dst, &place[..], "{name} in place");
        });
    }
}
