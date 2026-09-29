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

use crate::browser::css::Computed;

use super::super::display_list::DisplayList;
use super::clip_rect::clip_rect;
use super::map_fx::map_all;

/// Apply a box's clip-path and transform to everything it painted: the
/// fragments from `start` on, the first being its own border box. `base` is
/// the clip that was in force from its ancestors, which neither effect may
/// move. The clip is the shape's bounding box, cut in the box's own
/// coordinates; the transform then maps every fragment about the origin.
pub(crate) fn apply_fx(
    s: &Computed,
    frags: &mut DisplayList,
    start: usize,
    base: Option<[i32; 4]>,
) {
    let fx = &s.fx;
    if fx.transform.is_none() && fx.clip.is_none() {
        return;
    }
    let Some(b) = frags.get(start).map(|f| [f.x, f.y, f.w, f.h]) else { return };
    if let Some(clip) = fx.clip {
        let c = clip_rect(clip, b);
        let empty = c[2] <= c[0] || c[3] <= c[1];
        for f in frags.iter_mut().skip(start) {
            if empty {
                f.alpha = 0;
            }
            f.clip = Some(match f.clip {
                Some(o) => [o[0].max(c[0]), o[1].max(c[1]), o[2].min(c[2]), o[3].min(c[3])],
                None => c,
            });
        }
    }
    if let Some(t) = fx.transform {
        map_all(frags, start, t, fx.origin, b, base);
    }
}
