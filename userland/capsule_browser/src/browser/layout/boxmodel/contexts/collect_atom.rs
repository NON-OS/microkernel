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

use crate::browser::css::Size;

use super::super::border_box_w::border_box_w;
use super::super::ctx::{Ctx, Pin};
use super::super::display_list::DisplayList;
use super::super::geom::margins::margins;
use super::super::inline_items::{InlineItem, Lead};
use super::super::layout_box::layout_box;
use super::super::replaced_size::replaced_size;
use super::super::tree::{BoxKind, BoxNode};
use super::intrinsic::{contribution, intrinsic};

/* An atomic inline on a line `content_w` px wide. An image is a leaf at
 * its replaced size, unless a filter, gradient mask or opacity needs a box
 * for them. Anything else takes its declared width, else shrinks to fit
 * (max-content, no less than min-content while that fits) as CSS sizes an
 * auto-width inline-block. That border box is pinned, so a percentage width
 * is not taken again of itself, and the box is laid out at the origin
 * inside its margins for the line to move into place. */
pub(in super::super) fn atom(
    c: &BoxNode,
    content_w: i32,
    lead: Lead,
    depth: u32,
    ctx: Ctx,
) -> InlineItem {
    let fx = &c.style.fx;
    let effects = fx.tint != 0 || fx.fade != 0 || c.style.opacity != 255;
    if let (BoxKind::Image { src, alt }, false) = (&c.kind, effects) {
        let (w, h) = replaced_size(c, content_w, ctx.cb.h);
        let (src, alt, href, node, fit) =
            (src.clone(), alt.clone(), c.href.clone(), c.dom_id, c.style.object_fit);
        return InlineItem::Image { src, alt, w, h, href, node, fit, lead };
    }
    let [mt, mr, mb, ml] = margins(&c.style, content_w);
    let room = (content_w - ml - mr).max(0);
    let bw = match c.style.width {
        Size::Auto => {
            let (min, max) = intrinsic(c, depth);
            max.min(room).max(min)
        }
        _ => border_box_w(&c.style, room),
    };
    let mut frags: DisplayList = Vec::new();
    let pinned = Ctx { pin: Some(Pin { w: bw, h: None }), ..ctx };
    let h = layout_box(c, ml, mt, bw, &mut frags, depth, pinned);
    InlineItem::Atom { frags, w: (ml + bw + mr).max(0), h: (mt + h + mb).max(0), lead }
}

/* An atomic inline while the line has no width yet (the max-content walk
 * of a shrink-to-fit box): its max-content contribution. A percentage of
 * that unknown width, in its width or margins, counts as auto (CSS Sizing
 * 3, cyclic percentages), where resolving it against an unwrapped line
 * would make it as wide as the page. Only its advance is read. */
pub(in super::super) fn measured(c: &BoxNode, lead: Lead, depth: u32) -> InlineItem {
    InlineItem::Atom { frags: Vec::new(), w: contribution(c, depth).1, h: 0, lead }
}
