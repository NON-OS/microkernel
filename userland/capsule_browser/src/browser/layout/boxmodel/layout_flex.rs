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

use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::flex_col::flex_col;
use super::flex_row::flex_row;
use super::flow::shell::open;
use super::layout_block::MAX_DEPTH;
use super::tree::BoxNode;

/* Lay a flex container: same border-box shell as a block, children placed
 * along the main axis instead of stacked. */
pub(super) fn layout_flex(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    if depth > MAX_DEPTH {
        return 0;
    }
    let sh = open(node, [x, y, avail], frags, ctx);
    let inner_h = if node.style.flex_col {
        flex_col(node, sh.cx, sh.cy, sh.cw, frags, depth, sh.inner)
    } else {
        flex_row(node, sh.cx, sh.cy, sh.cw, frags, depth, sh.inner)
    };
    sh.close(node, inner_h, frags)
}
