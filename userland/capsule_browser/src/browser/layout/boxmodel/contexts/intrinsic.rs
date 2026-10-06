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

use super::super::edges_x::edges_x;
use super::super::geom::margins::margins;
use super::super::tree::BoxNode;
use super::intrinsic_content::content;

const MAX_DEPTH: u32 = 400;
/* Wider than any page; keeps sums of runaway content in range. */
pub(in super::super) const CAP: i32 = 1 << 20;

/* The (min-content, max-content) border-box widths of `n`: the narrowest
 * it can be without overflowing (its widest unbreakable piece) and the
 * width it takes with nothing wrapped. A definite px width decides both;
 * px min-width and max-width clamp them. Measured once per layout pass. */
pub(in super::super) fn intrinsic(n: &BoxNode, depth: u32) -> (i32, i32) {
    if depth > MAX_DEPTH {
        return (0, 0);
    }
    n.aux.intrinsic.get_or(|| {
        let s = &n.style;
        let (el, er) = edges_x(s);
        let bb = |v: i32| if s.border_box { v } else { v.saturating_add(el + er) };
        let (min, max) = match s.width.definite_px() {
            Some(w) => (bb(w), bb(w)),
            None => {
                let (a, b) = content(n, depth);
                (a.saturating_add(el + er), b.saturating_add(el + er))
            }
        };
        let clamp = |v: i32| clamp_px(s, v, bb);
        (clamp(min).clamp(0, CAP), clamp(max).clamp(0, CAP))
    })
}

/* max-width caps, then min-width raises, where they are plain px. */
fn clamp_px(s: &Computed, v: i32, bb: impl Fn(i32) -> i32) -> i32 {
    let v = s.max_width.definite_px().map_or(v, |m| v.min(bb(m)));
    s.min_width.definite_px().map_or(v, |m| v.max(bb(m)))
}

/* A child's (min, max) contribution to its container: its widths with
 * its margins (percentages of an unknown width count as none). Negative
 * margins make it smaller, down to below zero, as they do in a sum. */
pub(in super::super) fn contribution(c: &BoxNode, depth: u32) -> (i32, i32) {
    let [_, mr, _, ml] = margins(&c.style, 0);
    let (a, b) = intrinsic(c, depth);
    (a.saturating_add(ml.saturating_add(mr)), b.saturating_add(ml.saturating_add(mr)))
}
