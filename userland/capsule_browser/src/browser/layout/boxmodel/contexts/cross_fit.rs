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

use crate::browser::css::{Align, Size};

use super::super::border_box_w::border_box_w;
use super::super::tree::BoxNode;
use super::intrinsic::intrinsic;
use super::self_align::inline_offset;

/* An item's border-box width in `room` px across (a flex column's width,
 * or a grid area's) and its offset there. An auto width stretches when
 * the item's alignment is stretch and neither margin is auto, and else
 * fits the content; auto margins then centre the item or push it over,
 * and without them its alignment places it (start is the right side in
 * an rtl container). */
pub(in super::super) fn fit_across(
    it: &BoxNode,
    room: i32,
    align: Align,
    rtl: bool,
    depth: u32,
) -> (i32, i32) {
    let st = &it.style;
    let (lauto, rauto) = (st.margin_left_auto, st.margin_right_auto);
    let fit = |(a, b): (i32, i32)| b.min(room).max(a);
    let bw = match st.width {
        Size::Auto if align == Align::Stretch && !lauto && !rauto => border_box_w(st, room),
        Size::Auto => fit(intrinsic(it, depth + 1)),
        _ => border_box_w(st, room),
    };
    let dx = match (lauto, rauto) {
        (true, true) => (room - bw) / 2,
        (true, false) => room - bw,
        (false, true) => 0,
        _ => inline_offset(room, bw, align, rtl),
    };
    (bw, dx)
}
