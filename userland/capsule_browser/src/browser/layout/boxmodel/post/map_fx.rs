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

use super::super::affine::Affine;
use super::super::display_list::{Content, DisplayList};
use super::super::fx::{rel, Rel};
use super::fmath::{round, sqrt};

/* Clip edges past this are the open side of a one-axis clip. */
const OPEN: i32 = 1 << 28;

/// Map every fragment from `start` on through `t` about `origin` of the
/// border box `b` ([x, y, w, h]); each rectangle becomes the bounding box of
/// its mapped corners, so a rotated box paints its bounds. A map that
/// flattens the box to no area hides it all. A clip set inside the box moves
/// with it; `base`, the clip from above, stays where it is.
pub(crate) fn map_all(
    frags: &mut DisplayList,
    start: usize,
    t: Affine,
    origin: [Rel; 2],
    b: [i32; 4],
    base: Option<[i32; 4]>,
) {
    let [x, y, w, h] = b;
    let (ox, oy) = ((x + rel(origin[0], w)) as f32, (y + rel(origin[1], h)) as f32);
    let flat = (t.a * t.d - t.b * t.c).abs() < 1e-6;
    let text_k = sqrt(t.c * t.c + t.d * t.d);
    let map = |r: [i32; 4]| -> [i32; 4] {
        let mut lo = (f32::MAX, f32::MAX);
        let mut hi = (f32::MIN, f32::MIN);
        for (px, py) in [(r[0], r[1]), (r[2], r[1]), (r[0], r[3]), (r[2], r[3])] {
            let (u, v) = t.apply(px as f32 - ox, py as f32 - oy, w as f32, h as f32);
            (lo, hi) = ((lo.0.min(u), lo.1.min(v)), (hi.0.max(u), hi.1.max(v)));
        }
        /* Far past any page; keeps the corner arithmetic inside i32. */
        let at = |v: f32, o: f32| round((o + v).clamp(-(OPEN as f32), OPEN as f32));
        [at(lo.0, ox), at(lo.1, oy), at(hi.0, ox), at(hi.1, oy)]
    };
    for f in frags.iter_mut().skip(start) {
        let m = map([f.x, f.y, f.x + f.w, f.y + f.h]);
        let gone = flat || (m[2] <= m[0] && f.w > 0) || (m[3] <= m[1] && f.h > 0);
        (f.x, f.y, f.w, f.h) = (m[0], m[1], m[2] - m[0], m[3] - m[1]);
        if gone {
            f.alpha = 0;
        }
        if let Content::Text { px, .. } = &mut f.content {
            *px *= text_k;
        }
        f.clip = match f.clip {
            Some(c) if Some(c) != base => {
                let m = map(c.map(|v| v.clamp(-OPEN, OPEN)));
                let n: [i32; 4] =
                    core::array::from_fn(|i| if c[i].abs() >= OPEN { c[i] } else { m[i] });
                Some(base.map_or(n, |o| {
                    [n[0].max(o[0]), n[1].max(o[1]), n[2].min(o[2]), n[3].min(o[3])]
                }))
            }
            other => other,
        };
    }
}
