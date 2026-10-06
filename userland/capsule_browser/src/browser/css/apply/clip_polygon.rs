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

use crate::browser::css::calc::split_top::{items, words};
use crate::browser::layout::boxmodel::{Clip, Rel};

use super::transform_fn::rel_len;

/// polygon([fill-rule,] x y, ...) as the rectangle bounding its points. The
/// corners have to be ordered before the box size is known, so every x (and
/// every y) must be written in one kind of unit: all lengths or all
/// percentages, as polygon clips are in practice. A mix is not read.
pub(super) fn polygon(inner: &str, fs: u32) -> Option<Clip> {
    let mut lo: [Option<Rel>; 2] = [None, None];
    let mut hi: [Option<Rel>; 2] = [None, None];
    for (i, item) in items(inner).enumerate() {
        let mut w = words(item);
        let first = w.next()?;
        if i == 0 && matches!(first, "nonzero" | "evenodd") {
            continue;
        }
        let point = [rel_len(first, fs)?, rel_len(w.next()?, fs)?];
        for axis in 0..2 {
            let p = point[axis];
            lo[axis] = Some(pick(lo[axis], p, |a, b| a < b)?);
            hi[axis] = Some(pick(hi[axis], p, |a, b| a > b)?);
        }
    }
    Some(Clip::Rect([lo[0]?, lo[1]?, hi[0]?, hi[1]?]))
}

/* Keep whichever of `best` and `p` wins `better`, comparing the unit part
 * both carry; a length against a percentage cannot be ordered yet. */
fn pick(best: Option<Rel>, p: Rel, better: impl Fn(i32, i32) -> bool) -> Option<Rel> {
    let Some(b) = best else { return Some(p) };
    match (b, p) {
        ((_, 0), (_, 0)) => Some(if better(p.0, b.0) { p } else { b }),
        ((0, _), (0, _)) => Some(if better(p.1, b.1) { p } else { b }),
        _ => None,
    }
}

/* inset(top right bottom left [round radii]): 1-4 offsets in from each edge
 * of the border box. Negative offsets reach outside it. */
pub(super) fn inset(inner: &str, fs: u32) -> Option<Clip> {
    let mut v: [Rel; 4] = [(0, 0); 4];
    let mut n = 0;
    for w in words(inner).take_while(|w| !w.eq_ignore_ascii_case("round")) {
        *v.get_mut(n)? = rel_len(w, fs)?;
        n += 1;
    }
    let [t, r, b, l] = match n {
        1 => [v[0]; 4],
        2 => [v[0], v[1], v[0], v[1]],
        3 => [v[0], v[1], v[2], v[1]],
        4 => v,
        _ => return None,
    };
    Some(Clip::Rect([l, t, (-r.0, 1000 - r.1), (-b.0, 1000 - b.1)]))
}
