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
 * block-level box, or a float the block container lays out, is broken
 * around it. The pieces take its place in the
 * parent's children, in order: the inline with what came before the block,
 * the block, the inline with what follows; the parent's anonymous blocks
 * then give each inline run lines of its own. The first piece keeps the
 * left margin, border and padding, the last the right ones. A flex or grid
 * item (`in_items`) is blockified whole as one item, so it is not split. */

use alloc::vec::Vec;

use super::super::abs_out_of_flow::out_of_flow;
use super::super::leaf::leaf;
use super::super::tree::{BoxKind, BoxNode};
use super::split_pieces::{flush, trim_edges};

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
    let mut kids = children.into_iter();
    loop {
        let next = kids.next();
        if let Some(c) = next {
            if !in_flow_block(&c) {
                run.push(c);
                continue;
            }
            flush(&mut run, out, &mut inline_at, &piece);
            out.push(c);
        } else {
            flush(&mut run, out, &mut inline_at, &piece);
            break;
        }
    }
    trim_edges(out, start, &inline_at);
}

fn in_flow_block(c: &BoxNode) -> bool {
    c.kind.block_level() && !out_of_flow(&c.style)
}
