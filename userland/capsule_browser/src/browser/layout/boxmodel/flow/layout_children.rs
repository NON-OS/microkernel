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

use super::super::abs_out_of_flow::out_of_flow;
use super::super::ctx::Ctx;
use super::super::display_list::DisplayList;
use super::super::layout_inline::layout_inline;
use super::super::tree::BoxNode;
use super::block_flow::block_flow;

/* Lay a container's in-flow children into its content box at (x, y) by w
 * wide. All-inline children run as one inline context; otherwise they are
 * block-level (anonymous wrapping guarantees it) and stack in block flow. */
pub(crate) fn layout_children(
    node: &BoxNode,
    x: i32,
    y: i32,
    w: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    if node.children.is_empty() {
        return 0;
    }
    let in_flow = |c: &&BoxNode| !out_of_flow(&c.style);
    let all_inline = node.children.iter().filter(in_flow).all(|c| !c.kind.block_level());
    if all_inline {
        return layout_inline(&node.children, x, y, w, &node.style, frags, ctx);
    }
    block_flow(&node.children, [x, y, w], frags, depth, ctx)
}
