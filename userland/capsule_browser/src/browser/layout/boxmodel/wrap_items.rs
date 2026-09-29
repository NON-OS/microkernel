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

use super::abs_out_of_flow::out_of_flow;
use super::flush_run::flush_run;
use super::tree::{BoxKind, BoxNode};
use super::wrap_mixed::wrap_mixed;

/* The items of a flex or grid container. Every in-flow child element is
 * an item of its own, blockified as CSS requires: an inline or
 * inline-block box becomes a block, inline-flex and inline-grid become
 * flex and grid containers, and an image stays a replaced item. Only a
 * run of adjacent text shares one anonymous item, and a run of nothing but
 * white space is no item at all. An out-of-flow child passes through. */
#[inline(never)]
pub(super) fn wrap_items(parent: &Computed, children: Vec<BoxNode>) -> Vec<BoxNode> {
    let mut out: Vec<BoxNode> = Vec::with_capacity(children.len());
    let mut run: Vec<BoxNode> = Vec::new();
    for mut c in children {
        if matches!(c.kind, BoxKind::Text(_)) && !out_of_flow(&c.style) {
            run.push(c);
            continue;
        }
        flush_run(&mut out, &mut run, parent);
        match c.kind {
            _ if out_of_flow(&c.style) => {}
            /* As a block it holds only blocks or only inline content. */
            BoxKind::Inline => {
                let kids = core::mem::take(&mut c.children);
                (c.kind, c.children) = (BoxKind::Block, wrap_mixed(&c.style, kids));
            }
            BoxKind::InlineBlock => c.kind = blockify(&c.style),
            _ => {}
        }
        out.push(c);
    }
    flush_run(&mut out, &mut run, parent);
    out
}

/* The block-level kind an inline item's box takes. */
fn blockify(s: &Computed) -> BoxKind {
    match (s.is_grid, s.is_flex) {
        (true, _) => BoxKind::Grid,
        (_, true) => BoxKind::Flex,
        _ => BoxKind::Block,
    }
}
