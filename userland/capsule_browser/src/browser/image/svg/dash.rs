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

use alloc::vec;
use alloc::vec::Vec;

use super::math::sqrt;
use super::pen::Pen;

type P = [f32; 2];

/// The most dashes one subpath is cut into.
const MAX_DASHES: usize = 20_000;

/// The pieces of polyline `poly` the pen's dash pattern leaves drawn,
/// starting `offset` into the pattern; the whole line when undashed.
pub(super) fn dashed(poly: &[P], pen: &Pen) -> Vec<Vec<P>> {
    if pen.dashes == 0 || poly.is_empty() {
        return vec![poly.to_vec()];
    }
    let pattern = &pen.dash[..pen.dashes];
    let period: f32 = pattern.iter().sum();
    let mut at = pen.offset % period;
    if at < 0.0 {
        at += period;
    }
    let mut k = 0usize;
    while at >= pattern[k] {
        at -= pattern[k];
        k = (k + 1) % pattern.len();
    }
    let mut left = pattern[k] - at;
    let mut out: Vec<Vec<P>> = Vec::new();
    let mut cur: Vec<P> = if k % 2 == 0 { vec![poly[0]] } else { Vec::new() };
    for w in poly.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let len = sqrt(dx * dx + dy * dy);
        let mut done = 0f32;
        while len > 0.0 && len - done > left && out.len() < MAX_DASHES {
            done += left;
            let p = [a[0] + (b[0] - a[0]) * done / len, a[1] + (b[1] - a[1]) * done / len];
            if k % 2 == 0 {
                cur.push(p);
                out.push(core::mem::take(&mut cur));
            } else {
                cur = vec![p];
            }
            k = (k + 1) % pattern.len();
            left = pattern[k];
        }
        left -= len - done;
        if k % 2 == 0 {
            cur.push(b);
        }
    }
    if cur.len() > 1 {
        out.push(cur);
    }
    out
}
