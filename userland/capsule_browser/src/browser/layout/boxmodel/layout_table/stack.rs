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

use super::super::content_width::content_width;
use super::super::edges_x::edges_x;
use super::super::min_content_width::min_content_width;
use super::super::tree::BoxNode;
use super::measure::{intrinsic, CAP};

/* A block's children: block-level ones stack (the widest wins), each
 * run of inline ones shares a line (their widths add). */
pub(super) fn stacked(node: &BoxNode, depth: u32) -> (i32, i32) {
    let (mut mn, mut mx, mut run) = (0i32, 0i32, 0i32);
    for c in &node.children {
        if c.kind.block_level() {
            let m = c.style.margin_left.max(0) + c.style.margin_right.max(0);
            let (a, b) = intrinsic(c, depth + 1);
            (mn, mx, run) = (mn.max(a + m), mx.max(run).max(b + m), 0);
        } else {
            mn = mn.max(min_content_width(c, depth + 1));
            run = run.saturating_add(content_width(c, depth + 1));
        }
    }
    let (el, er) = edges_x(&node.style);
    ((mn + el + er).clamp(0, CAP), (mx.max(run) + el + er).clamp(0, CAP))
}
