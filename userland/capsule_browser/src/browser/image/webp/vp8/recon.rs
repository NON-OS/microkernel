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

use super::idct::add_transform;
use super::modes::Mb;
use super::planes::Plane;
use super::predict4::pred4;
use super::recon_edges::{top_right, whole};

/// The frame's Y, U and V planes and its width in macroblocks.
pub(super) struct Frame {
    pub y: Plane,
    pub u: Plane,
    pub v: Plane,
    pub mbw: usize,
}

/// Predict one macroblock from its unfiltered neighbours and add its
/// residual (RFC 6386 12 and 14).
pub(super) fn reconstruct(f: &mut Frame, (mbx, mby): (usize, usize), mb: &Mb, c: &[i16; 384]) {
    let (x0, y0, s) = (mbx * 16, mby * 16, f.y.stride);
    if mb.i4x4 {
        let tr = top_right(&f.y, mbx, mby, f.mbw);
        for n in 0..16 {
            let (bx, by) = (x0 + (n % 4) * 4, y0 + (n / 4) * 4);
            let above = f.y.row(bx as isize, by as isize - 1, 8);
            let mut t = [f.y.at(bx as isize - 1, by as isize - 1); 9];
            t[1..5].copy_from_slice(&above[..4]);
            t[5..9].copy_from_slice(if n % 4 == 3 { &tr } else { &above[4..8] });
            let l = f.y.col(bx as isize - 1, by as isize, 4);
            f.y.put(bx, by, 4, &pred4(mb.bmodes[n], &t, &[l[0], l[1], l[2], l[3]]));
            add_transform(&c[n * 16..n * 16 + 16], &mut f.y.px, by * s + bx, s);
        }
    } else {
        whole(&mut f.y, (x0, y0), 16, mb.ymode, (mbx > 0, mby > 0));
        for n in 0..16 {
            let at = (y0 + (n / 4) * 4) * s + x0 + (n % 4) * 4;
            add_transform(&c[n * 16..n * 16 + 16], &mut f.y.px, at, s);
        }
    }
    for (p, base) in [(&mut f.u, 256), (&mut f.v, 320)] {
        whole(p, (mbx * 8, mby * 8), 8, mb.uvmode, (mbx > 0, mby > 0));
        for k in 0..4 {
            let at = (mby * 8 + (k / 2) * 4) * p.stride + mbx * 8 + (k % 2) * 4;
            let stride = p.stride;
            add_transform(&c[base + k * 16..base + k * 16 + 16], &mut p.px, at, stride);
        }
    }
}
