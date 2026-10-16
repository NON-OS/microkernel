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

//! The rows a copy writes at u32's edges (G-70): held to the bytes its
//! bounds allow, by the test's u128 reading, and not one past them.

use crate::copy::copy_rects;
use crate::sweep::edges::{pairs, EDGES};
use nonos_gfx_wire::Rect;

const LEN: usize = 256;
const STRIDES: [usize; 5] = [4, 64, 1 << 20, usize::MAX / 2, usize::MAX];

/// What a copy of `r` leaves in `LEN` bytes, by the test's reading: every
/// row, or none when its last row's end, in u128, passes either side.
fn model(src: &[u8], ss: usize, ds: usize, r: Rect) -> Vec<u8> {
    let mut out = vec![0u8; LEN];
    let (x, w, y, h) = (r.x as u128 * 4, r.width as u128 * 4, r.y as u128, r.height as u128);
    let last = (y + h).saturating_sub(1);
    let end = |s: usize| last * s as u128 + x + w;
    if end(ss) <= src.len() as u128 && end(ds) <= LEN as u128 {
        for row in y..y + h {
            let (a, b) = ((row * ss as u128 + x) as usize, (row * ds as u128 + x) as usize);
            out[b..b + w as usize].copy_from_slice(&src[a..a + w as usize]);
        }
    }
    out
}

#[test]
fn a_copy_writes_only_inside_its_bounds_at_every_edge() {
    let src: Vec<u8> = (0..LEN).map(|i| i as u8 | 1).collect();
    let (mut written, mut cases) = (0, 0);
    for ((x, width), &y, &height) in pairs()
        .flat_map(|p| EDGES[..9].iter().map(move |y| (p, y)))
        .flat_map(|(p, y)| EDGES.iter().map(move |h| (p, y, h)))
    {
        let r = Rect { x, y, width, height };
        for (&ss, &ds) in STRIDES.iter().flat_map(|a| STRIDES.iter().map(move |b| (a, b))) {
            let mut dst = vec![0u8; LEN + 64];
            // SAFETY: dst is aligned for u32 and holds LEN bytes and a guard.
            unsafe { copy_rects(&src, ss, dst.as_mut_ptr(), LEN, ds, &[r], false) };
            assert!(dst[LEN..].iter().all(|&b| b == 0), "past dst_len: {r:?} {ss} {ds}");
            assert_eq!(dst[..LEN], model(&src, ss, ds, r)[..], "{r:?} {ss} {ds}");
            (written, cases) = (written + dst.iter().any(|&b| b != 0) as u32, cases + 1);
        }
    }
    assert!(written > 1_000 && cases - written > 100_000, "{written} of {cases}");
}
