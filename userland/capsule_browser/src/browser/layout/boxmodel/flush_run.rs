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

use crate::browser::css::{Computed, WhiteSpace};

use super::tree::{BoxKind, BoxNode};

/* Wrap a pending run of inline children in one anonymous block and append
 * it. A run of nothing but collapsible white space makes no box: between
 * blocks, or as a flex or grid item, that space collapses away. */
#[inline(never)]
pub(super) fn flush_run(out: &mut Vec<BoxNode>, run: &mut Vec<BoxNode>, parent: &Computed) {
    if run.iter().all(collapsible) {
        run.clear();
        return;
    }
    out.push(BoxNode {
        kind: BoxKind::Block,
        style: Computed::inherit_from(parent),
        href: None,
        dom_id: 0,
        bg_image: None,
        grid_place: None,
        children: core::mem::take(run),
        aux: Default::default(),
    });
}

/* A text box holding only white space that its white-space mode collapses. */
fn collapsible(c: &BoxNode) -> bool {
    let collapses = matches!(c.style.white_space, WhiteSpace::Normal | WhiteSpace::Nowrap);
    let blank = |t: &str| t.bytes().all(|b| matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C'));
    matches!(&c.kind, BoxKind::Text(t) if collapses && t != "\n" && blank(t))
}
