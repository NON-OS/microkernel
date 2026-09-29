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

/* Block-in-inline (CSS 2.1 9.2.1.1): an inline box holding an in-flow
 * block-level box is broken around it. The pieces take its place in the
 * parent's children, in order: the inline with what came before the block,
 * the block, the inline with what follows; the parent's anonymous blocks
 * then give each inline run lines of its own. The first piece keeps the
 * left margin, border and padding, the last the right ones. A flex or grid
 * item (`in_items`) is blockified whole as one item, so it is not split. */

use alloc::vec::Vec;

use crate::browser::css::Float;

use super::super::abs_out_of_flow::out_of_flow;
use super::super::leaf::leaf;
use super::super::tree::{BoxKind, BoxNode};

pub(super) fn push_split(node: BoxNode, in_items: bool, out: &mut Vec<BoxNode>) {
    let split = matches!(node.kind, BoxKind::Inline) && !in_items;
    if !split || !node.children.iter().any(in_flow_block) {
        out.push(node);
        return;
    }
    let BoxNode { style, href, dom_id, bg_image, children, .. } = node;
    let piece = |kids: Vec<BoxNode>| {
        let mut p = leaf(BoxKind::Inline, &style, &href, dom_id);
        (p.style, p.bg_image, p.children) = (style, bg_image.clone(), kids);
        p
    };
    let (start, mut run, mut inline_at) = (out.len(), Vec::new(), Vec::new());
    for c in children {
        if !in_flow_block(&c) {
            run.push(c);
            continue;
        }
        if !run.is_empty() {
            inline_at.push(out.len());
            out.push(piece(core::mem::take(&mut run)));
        }
        out.push(c);
    }
    if !run.is_empty() {
        inline_at.push(out.len());
        out.push(piece(run));
    }
    let (n, end) = (inline_at.len(), out.len());
    for (i, &at) in inline_at.iter().enumerate() {
        let s = &mut out[at].style;
        if i > 0 || at > start {
            (s.margin_left, s.pad_left, s.border_left) = (0, 0, 0);
        }
        if i + 1 < n || at + 1 < end {
            (s.margin_right, s.pad_right, s.border_right) = (0, 0, 0);
        }
    }
}

fn in_flow_block(c: &BoxNode) -> bool {
    c.kind.block_level() && !out_of_flow(&c.style) && c.style.float == Float::None
}
