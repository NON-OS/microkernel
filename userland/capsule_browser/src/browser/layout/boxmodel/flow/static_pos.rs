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
use super::super::tree::{BoxKind, BoxNode};

/* Inline nesting followed when looking for out-of-flow boxes. */
const MAX_INLINE_DEPTH: u32 = 64;

/// Record (x, y) as the static position of every out-of-flow box among
/// `children`, and among the children of their inline boxes, which flow
/// through the same place. A container records its content origin for all
/// of them first; block flow then refines each to its own spot.
pub(crate) fn record_static(children: &[BoxNode], x: i32, y: i32, ctx: Ctx) {
    record(children, x, y, ctx, 0);
}

fn record(children: &[BoxNode], x: i32, y: i32, ctx: Ctx, depth: u32) {
    for c in children {
        if out_of_flow(&c.style) {
            c.aux.flow_at.set(Some((x, y, ctx)));
        } else if matches!(c.kind, BoxKind::Inline) && depth < MAX_INLINE_DEPTH {
            record(&c.children, x, y, ctx, depth + 1);
        }
    }
}
