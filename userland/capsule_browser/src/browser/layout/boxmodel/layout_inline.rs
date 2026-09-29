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

use crate::browser::css::Computed;

use super::collect_items::collect_items;
use super::contexts::line_box::LineBox;
use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::flush_line::Place;
use super::inline_items::InlineItem;
use super::tree::BoxNode;

/* Lay an inline formatting context into line boxes at (x, y), `w` wide,
 * and return the height used. A line breaks only where an item allows it:
 * at a wrapping space, around an atom, after a hyphen or slash. An item
 * that does not fit and cannot start a line breaks it at the last chance
 * before it, the rest carried over, so a word split across elements
 * ("micro<b>kernel</b>") moves as one; with no chance the line overflows,
 * as CSS lets it. Out of line: nested blocks do not pay for its frame. */
#[inline(never)]
pub(super) fn layout_inline(
    children: &[BoxNode],
    x: i32,
    y: i32,
    w: i32,
    container: &Computed,
    frags: &mut DisplayList,
    ctx: Ctx,
) -> i32 {
    let mut items: Vec<InlineItem> = Vec::new();
    collect_items(children, Some(w), &mut items, 0, ctx);
    if items.is_empty() {
        return 0;
    }
    let place = Place::new(container, x, w, ctx);
    let mut line = LineBox::new(container.line_height() as i32);
    let mut cy = 0i32;
    for item in items {
        if matches!(item, InlineItem::Break) {
            cy += line.flush(frags, &place, y + cy);
            continue;
        }
        let lead = item.lead();
        let gap = if line.is_empty() { 0 } else { lead.space };
        if !line.is_empty() && line.w + gap + item.advance_w() > w {
            if lead.brk {
                cy += line.flush(frags, &place, y + cy);
            } else if let Some(tail) = line.take_tail() {
                cy += line.flush(frags, &place, y + cy);
                tail.into_iter().for_each(|it| line.push(it));
            }
        }
        line.push(item);
    }
    if !line.is_empty() {
        cy += line.flush(frags, &place, y + cy);
    }
    cy
}
