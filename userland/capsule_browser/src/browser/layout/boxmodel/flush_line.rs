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

use crate::browser::css::{Computed, TextAlign};

use super::contexts::bidi::{needs_bidi, visual};
use super::contexts::flush_item::flush_item;
use super::contexts::line_join::join_pieces;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::inline_items::InlineItem;

/* Where a container's lines go: its content box x and width, the side
 * its text lines up on (text-align with start and end resolved against
 * the direction), and its direction, the bidi paragraph level. */
pub(super) struct Place {
    x: i32,
    w: i32,
    align: TextAlign,
    rtl: bool,
    ctx: Ctx,
}

impl Place {
    pub(super) fn new(c: &Computed, x: i32, w: i32, ctx: Ctx) -> Self {
        let align = match (c.text_align, c.rtl) {
            (TextAlign::Start, false) | (TextAlign::End, true) => TextAlign::Left,
            (TextAlign::Start, true) | (TextAlign::End, false) => TextAlign::Right,
            (a, _) => a,
        };
        Place { x, w, align, rtl: c.rtl, ctx }
    }

    /* Emit one finished line at `top`, `h` tall, `line_w` used: items at
     * their line-relative x, in visual order when any run right to left.
     * An underlined word after underlined text and an underlined space
     * takes the space into its fragment, so the underline is unbroken. */
    pub(super) fn emit(
        &self,
        frags: &mut DisplayList,
        items: Vec<(i32, InlineItem)>,
        [top, h, line_w]: [i32; 3],
    ) {
        let extra = (self.w - line_w).max(0);
        let shift = match self.align {
            TextAlign::Center => extra / 2,
            TextAlign::Right => extra,
            _ => 0,
        };
        let items = join_pieces(items);
        let bidi = needs_bidi(self.rtl, &items);
        let items = if bidi { visual(items, self.rtl) } else { items };
        let mut ul_end: Option<i32> = None;
        for (ix, item) in items {
            let x = self.x + shift + ix;
            ul_end = flush_item(frags, item, [x, top, h], ul_end.filter(|_| !bidi), &self.ctx);
        }
    }
}
