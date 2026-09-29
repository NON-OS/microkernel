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

use super::fields::Strength;
use super::filter_ops::{edge, Kind};
use super::planes::Plane;
use super::recon::Frame;

/// Loop-filter one reconstructed macroblock in frame order (RFC 6386 15):
/// its left edge, inner vertical edges, top edge, then inner horizontal
/// edges. `simple` filters luma only; the normal filter does chroma too.
pub(super) fn filter_mb(f: &mut Frame, (mbx, mby): (usize, usize), st: Strength, simple: bool) {
    if st.limit == 0 {
        return;
    }
    let (edge_t, inner_t) = (st.limit + 4, st.limit);
    let limits = |t| (t, st.ilevel, st.hev);
    let (mk, ik) = if simple { (Kind::Simple, Kind::Simple) } else { (Kind::Mb, Kind::Inner) };
    let mut planes: [(&mut Plane, usize); 3] = [(&mut f.y, 16), (&mut f.u, 8), (&mut f.v, 8)];
    let planes = if simple { &mut planes[..1] } else { &mut planes[..] };
    for (p, n) in planes.iter_mut() {
        let (s, n) = (p.stride, *n);
        let origin = mby * n * s + mbx * n;
        let inner: &[usize] = if n == 16 { &[4, 8, 12] } else { &[4] };
        if mbx > 0 {
            edge(&mut p.px, (origin, 1, s), n, mk, limits(edge_t));
        }
        if st.inner {
            for &d in inner {
                edge(&mut p.px, (origin + d, 1, s), n, ik, limits(inner_t));
            }
        }
        if mby > 0 {
            edge(&mut p.px, (origin, s, 1), n, mk, limits(edge_t));
        }
        if st.inner {
            for &d in inner {
                edge(&mut p.px, (origin + d * s, s, 1), n, ik, limits(inner_t));
            }
        }
    }
}
