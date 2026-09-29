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

use super::super::abs_out_of_flow::out_of_flow;
use super::super::collect_items::collect_items;
use super::super::content_width::content_width;
use super::super::ctx::Ctx;
use super::super::edges_x::edges_x;
use super::super::inline_items::InlineItem;
use super::super::min_content_width::min_content_width;
use super::super::tree::BoxNode;

/* Wider than any line; the measuring pass must not wrap. */
const UNWRAPPED: i32 = 1 << 20;

/// The width `n` wants with nothing wrapped. A box holding a run of
/// inline content measures its longest line the way line layout builds
/// it, with a space before every word after the first, since the
/// max-content walk counts the words but not the spaces between elements.
pub(crate) fn max_content(n: &BoxNode, d: u32, ctx: Ctx) -> i32 {
    let walk = content_width(n, d);
    let in_flow = n.children.iter().filter(|c| !out_of_flow(&c.style));
    if n.children.is_empty() || in_flow.clone().any(|c| c.kind.block_level()) {
        return walk;
    }
    let mut items = Vec::new();
    collect_items(&n.children, UNWRAPPED, &mut items, d, ctx);
    let (mut line, mut widest) = (0i32, 0i32);
    for it in &items {
        if matches!(it, InlineItem::Break) {
            (widest, line) = (widest.max(line), 0);
            continue;
        }
        let gap = if line == 0 { 0 } else { it.space_w() };
        line = line.saturating_add(gap + it.advance_w());
    }
    let (el, er) = edges_x(&n.style);
    walk.max(widest.max(line) + el + er)
}

/// Shrink-to-fit: the max-content width within `room`, but never below the
/// min-content width unless even that does not fit. Floats and absolutely
/// positioned boxes with an auto width size this way.
pub(crate) fn shrink_to_fit(c: &BoxNode, room: i32, d: u32, ctx: Ctx) -> i32 {
    let room = room.max(0);
    max_content(c, d + 1, ctx).min(room).max(min_content_width(c, d + 1)).min(room)
}
