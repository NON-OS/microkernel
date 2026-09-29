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

//! Table formatting (CSS 2.1 section 17, the automatic layout). The cells
//! form a grid with their colspans and rowspans; the columns take their
//! widths from the cells' min- and max-content widths; an auto-width table
//! shrinks to fit its columns, and each row is as tall as its tallest cell.

mod cell;
mod columns;
mod grid;
mod measure;
mod place;
mod rows;
mod share;
mod stack;
mod table;
mod width;

use super::ctx::Ctx;
use super::display_list::DisplayList;
use super::tree::BoxNode;

/* Lay table `node` at (x, y) in `avail` px: its captions, then its rows
 * over the column widths, the spacing around every cell. margin: auto,
 * or a -webkit-center around it, centres a table narrower than `avail`.
 * A table with no cells lays out as a block. Returns the border-box
 * height. */
pub(super) fn layout_table(
    node: &BoxNode,
    x: i32,
    y: i32,
    avail: i32,
    frags: &mut DisplayList,
    depth: u32,
    ctx: Ctx,
) -> i32 {
    table::lay(node, [x, y, avail], frags, depth, ctx)
}
