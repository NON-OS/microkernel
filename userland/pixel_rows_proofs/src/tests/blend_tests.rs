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

use crate::blend::{self, flatten, over_colour, over_row, under_row};
use crate::reference::{lay_row, paint_over, paint_row};
use crate::rng::Rng;
use crate::scalar::over;

use super::paths::kernels;

#[test]
fn the_rule_is_the_old_rule_for_every_channel_and_alpha() {
    for a in 0..=255u32 {
        for u in 0..=255u32 {
            for l in (0..=255u32).step_by(3) {
                let up = a << 24 | u << 16 | u << 8 | u;
                let lo = l << 24 | l << 16 | l << 8 | l;
                assert_eq!(over(up, lo), paint_over(up, lo), "a {a} u {u} l {l}");
            }
        }
    }
}

#[test]
fn an_opaque_pixel_over_a_ground_is_itself() {
    let mut r = Rng::new(3);
    for _ in 0..10_000 {
        let px = r.next() as u32 | 0xFF00_0000;
        assert_eq!(over(px, r.next() as u32), px);
    }
}

/// Rows of every length to 70 pixels, at every byte offset, so the vector
/// blocks, the tail and unaligned loads are all crossed.
fn each_row(mut f: impl FnMut(&mut [u8], &[u8], u32)) {
    let mut r = Rng::new(0x5EED);
    for len in 0..70 {
        for off in 0..4 {
            let src = r.pixels(len + 1);
            let mut dst = r.pixels(len + 1);
            let colour = r.next() as u32 | 0xFF00_0000;
            f(&mut dst[off..off + len * 4], &src[off..off + len * 4], colour);
        }
    }
}

#[test]
fn every_path_lays_a_row_over_as_the_old_loop_did() {
    for (name, k) in kernels() {
        each_row(|dst, src, c| {
            let mut want = dst.to_vec();
            lay_row(&mut want, src, true);
            k(dst, src, c, blend::SRC_OVER);
            assert_eq!(dst, &want[..], "{name}");
        });
    }
}

#[test]
fn every_path_lays_a_row_under_as_the_old_loop_did() {
    for (name, k) in kernels() {
        each_row(|dst, src, c| {
            let mut want = dst.to_vec();
            lay_row(&mut want, src, false);
            k(dst, src, c, blend::SRC_UNDER);
            assert_eq!(dst, &want[..], "{name}");
        });
    }
}

#[test]
fn every_path_flattens_onto_the_ground_as_the_paint_loop_did() {
    for (name, k) in kernels() {
        each_row(|dst, src, c| {
            let mut want = vec![0u32; src.len() / 4];
            paint_row(&mut want, src, c);
            let want: Vec<u8> = want.iter().flat_map(|p| p.to_le_bytes()).collect();
            k(dst, src, c, blend::OVER_COLOUR);
            assert_eq!(dst, &want[..], "{name}");
            k(dst, src, c, blend::FLATTEN);
            assert_eq!(dst, &want[..], "{name}: an opaque row flattens to itself");
        });
    }
}

#[test]
fn every_path_flattens_in_place() {
    for (name, k) in kernels() {
        each_row(|dst, _, c| {
            let want: Vec<u8> = dst
                .chunks_exact(4)
                .flat_map(|p| {
                    paint_over(u32::from_le_bytes([p[0], p[1], p[2], p[3]]), c).to_le_bytes()
                })
                .collect();
            k(dst, &[], c, blend::FLATTEN);
            assert_eq!(dst, &want[..], "{name}");
        });
    }
}

#[test]
fn the_rows_this_host_picks_match_the_old_loops() {
    each_row(|dst, src, c| {
        let (mut a, mut b, mut f) = (dst.to_vec(), dst.to_vec(), dst.to_vec());
        let (mut ra, mut rb) = (dst.to_vec(), dst.to_vec());
        lay_row(&mut ra, src, true);
        lay_row(&mut rb, src, false);
        over_row(&mut a, src);
        under_row(&mut b, src);
        assert_eq!((a, b), (ra, rb));
        over_colour(dst, src, c);
        f.copy_from_slice(src);
        flatten(&mut f, c);
        assert_eq!(dst, &f[..]);
    });
}

#[test]
fn a_short_source_lays_only_what_it_has() {
    let mut r = Rng::new(9);
    let src = r.pixels(5);
    let mut dst = r.pixels(9);
    let mut want = dst.clone();
    lay_row(&mut want[..20], &src, true);
    over_row(&mut dst, &src);
    assert_eq!(dst, want, "past the source nothing changes");
}

#[test]
fn a_damaged_box_flattens_as_the_old_lay_did() {
    each_row(|dst, src, c| {
        let mut want = dst.to_vec();
        crate::reference::lay_box_row(&mut want, src, c);
        over_colour(dst, src, c);
        assert_eq!(dst, &want[..]);
    });
}
