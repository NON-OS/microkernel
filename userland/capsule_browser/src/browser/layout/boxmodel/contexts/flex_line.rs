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

use alloc::vec::Vec;

use crate::browser::css::{Align, Computed, Size};

use super::super::ctx::{Ctx, Pin};
use super::super::display_list::DisplayList;
use super::super::layout_box::layout_box;
use super::flex_item::FlexItem;
use super::self_align::cross_shift;

/* One flex line to lay: its box [x, y, width], main-axis offsets (first
 * item, step after each, extra per auto margin), its cross size when known
 * up front (a single-line box of definite height), the container style. */
pub(in super::super) struct Line<'s> {
    pub r: [i32; 3],
    pub offsets: (i32, i32, i32),
    pub cross: Option<i32>,
    pub style: &'s Computed,
}

/* Lay a line's items at their main positions (mirrored in an rtl box),
 * each at its pinned width; then line them up across it by align-self
 * (else the box's align-items). A stretched item of auto height is given
 * the line's height: up front when the line's height is known, else by
 * growing its box once the tallest item has set it. Returns the line's
 * height. */
pub(in super::super) fn lay_line(
    items: &[FlexItem],
    line: &Line,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    let ([x, y, w], (first, step, share), s) = (line.r, line.offsets, line.style);
    let mut cx = first;
    let mut laid: Vec<(usize, usize, i32, Align)> = Vec::with_capacity(items.len());
    for it in items {
        let st = &it.node.style;
        let auto = |on: bool| share * on as i32;
        let (ml, mr) = (it.m[3] + auto(st.margin_left_auto), it.m[1] + auto(st.margin_right_auto));
        let align = st.align_self.unwrap_or(s.align);
        let stretch = align == Align::Stretch && st.height == Size::Auto;
        let h = line.cross.filter(|_| stretch).map(|c| (c - it.m[0] - it.m[2]).max(0));
        let ix = if s.rtl { w - cx - ml - it.size } else { cx + ml };
        let pinned = Ctx { pin: Some(Pin { w: it.size, h }), ..ctx };
        let start = frags.len();
        let bh = layout_box(it.node, x + ix, y + it.m[0], it.size, frags, depth + 1, pinned);
        let align = if align == Align::Stretch && !stretch { Align::Start } else { align };
        laid.push((start, frags.len(), bh, align));
        cx = cx.saturating_add(ml + it.size + mr + step);
    }
    let tallest = items.iter().zip(&laid).map(|(it, l)| l.2 + it.m[0] + it.m[2]).max().unwrap_or(0);
    let cross = line.cross.unwrap_or(tallest);
    for (it, &(a, b, bh, align)) in items.iter().zip(&laid) {
        cross_shift(frags, [a, b], [it.m[0], bh, it.m[2]], cross, align, ctx.clip);
    }
    cross
}
