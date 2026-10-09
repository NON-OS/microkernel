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

use crate::browser::css::Size;

use super::super::edges_x::edges_x;
use super::super::geom::margins::margins;
use super::super::tree::BoxNode;
use super::intrinsic::intrinsic;

/* A flex item on a row: its margins [top, right, bottom, left], its flex
 * base size and min/max clamps (border box), its grow and shrink factors
 * (hundredths), and the main size it is given. Percentages resolve
 * against the container's width once, here. */
pub(in super::super) struct FlexItem<'a> {
    pub node: &'a BoxNode,
    pub m: [i32; 4],
    pub base: i32,
    pub min: i32,
    pub max: i32,
    pub grow: i64,
    pub shrink: i64,
    pub size: i32,
}

impl FlexItem<'_> {
    /* The item's margin-box width at its current main size. */
    pub(in super::super) fn outer(&self) -> i32 {
        self.size.saturating_add(self.m[1]).saturating_add(self.m[3])
    }

    pub(in super::super) fn clamp(&self, v: i32) -> i32 {
        v.min(self.max).max(self.min)
    }
}

/* Build the item for child `c` of a row `w` px wide. The base is the
 * flex-basis, else the width, else the max-content width; min-width auto
 * is the content minimum (min-content, no more than a definite width) for
 * a box whose overflow is visible, as CSS defines it for flex items. */
pub(in super::super) fn flex_item(c: &BoxNode, w: i32, depth: u32) -> FlexItem<'_> {
    let s = &c.style;
    let (el, er) = edges_x(s);
    let edges = if s.border_box { 0 } else { el + er };
    let (imin, imax) = intrinsic(c, depth);
    let len = |v: Size| v.resolve(w).map(|v| v.saturating_add(edges));
    let base = match s.flex_basis {
        Size::Auto => len(s.width).unwrap_or(imax),
        b => len(b).unwrap_or(imax),
    };
    let max = len(s.max_width).unwrap_or(i32::MAX);
    let min = match s.min_width {
        Size::Auto if s.clips_x() => 0,
        Size::Auto => len(s.width).map_or(imin, |sw| imin.min(sw)).min(max),
        v => len(v).unwrap_or(0),
    };
    let (grow, shrink) = (s.flex_grow as i64, s.flex_shrink as i64);
    let it =
        FlexItem { node: c, m: margins(s, w), base: base.max(0), min, max, grow, shrink, size: 0 };
    FlexItem { size: it.clamp(it.base), ..it }
}
