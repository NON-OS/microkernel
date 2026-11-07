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

use crate::composite::{composite_row, mix, scalar};
use crate::reference;
use crate::rng::Rng;

fn words(b: &[u8]) -> Vec<u32> {
    b.chunks_exact(4).map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]])).collect()
}

/// Source rows with runs: whole blocks opaque, whole blocks clear, and
/// mixed, so every branch of the vector paths is crossed.
fn source(r: &mut Rng, n: usize) -> Vec<u8> {
    let mut v = r.pixels(n);
    let mut i = 0;
    while i < n {
        let run = (r.next() % 20) as usize + 1;
        let kind = r.next() % 3;
        for p in v.chunks_exact_mut(4).skip(i).take(run) {
            match kind {
                0 => p[3] = 0xFF,
                1 => p[3] = 0,
                _ => {}
            }
        }
        i += run;
    }
    v
}

fn each(mut f: impl FnMut(&mut [u8], &[u8])) {
    let mut r = Rng::new(0xC0DE);
    for len in 0..90 {
        for off in 0..4 {
            let src = source(&mut r, len + 1);
            let mut dst = r.pixels(len + 1);
            f(&mut dst[off..off + len * 4], &src[off..off + len * 4]);
        }
    }
}

fn want(dst: &[u8], src: &[u8]) -> Vec<u8> {
    let mut d = words(dst);
    reference::composite_row(&words(src), &mut d);
    d.iter().flat_map(|p| p.to_le_bytes()).collect()
}

#[test]
fn the_mix_is_the_old_blend_for_every_alpha_and_channel() {
    for a in 1..255u32 {
        for s in 0..=255u32 {
            for d in (0..=255u32).step_by(5) {
                let (sp, dp) = (a << 24 | s << 16 | s << 8 | s, d << 24 | d << 16 | d << 8 | d);
                assert_eq!(mix(sp, dp, a), reference::blend(sp, dp, a));
            }
        }
    }
}

#[test]
fn every_path_composites_as_the_old_row_did() {
    each(|dst, src| {
        let w = want(dst, src);
        let mut plain = dst.to_vec();
        scalar(&mut plain, src);
        assert_eq!(plain, w, "scalar");
        let mut picked = dst.to_vec();
        composite_row(&mut picked, src);
        assert_eq!(picked, w, "the host's path");
        let n = dst.len() / 16 * 16;
        let mut v = dst.to_vec();
        // SAFETY: SSE2 is in every x86-64; both hold `n` bytes.
        unsafe { crate::sse2::composite(v.as_mut_ptr(), src.as_ptr(), n) };
        scalar(&mut v[n..], &src[n..]);
        assert_eq!(v, w, "sse2");
        if std::is_x86_feature_detected!("avx2") {
            let n = dst.len() / 32 * 32;
            let mut v = dst.to_vec();
            // SAFETY: checked just now; both hold `n` bytes.
            unsafe { crate::avx2::composite(v.as_mut_ptr(), src.as_ptr(), n) };
            scalar(&mut v[n..], &src[n..]);
            assert_eq!(v, w, "avx2");
        }
    });
}

#[test]
fn a_clear_pixel_keeps_the_destinations_alpha() {
    let src = vec![0u8; 64];
    let mut dst: Vec<u8> = (0..16u32).flat_map(|i| (0x1234_5600 | i).to_le_bytes()).collect();
    let before = dst.clone();
    composite_row(&mut dst, &src);
    assert_eq!(dst, before);
}
