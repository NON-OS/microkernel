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

/* The pieces an inline is broken into around blocks: which runs become
 * pieces, and which edges each piece keeps. */

use alloc::vec::Vec;

use crate::browser::css::WhiteSpace;

use super::super::tree::{BoxKind, BoxNode};

/* Close the inline run as a piece, unless it is only collapsible white
 * space, which makes no line box between two blocks. */
pub(super) fn flush(
    run: &mut Vec<BoxNode>,
    out: &mut Vec<BoxNode>,
    at: &mut Vec<usize>,
    piece: &dyn Fn(Vec<BoxNode>) -> BoxNode,
) {
    let kids = core::mem::take(run);
    if !kids.iter().all(blank) {
        at.push(out.len());
        out.push(piece(kids));
    }
}

fn blank(k: &BoxNode) -> bool {
    let collapses = matches!(k.style.white_space, WhiteSpace::Normal | WhiteSpace::Nowrap);
    collapses && matches!(&k.kind, BoxKind::Text(t) if t.trim().is_empty())
}

/* The first piece keeps the left margin, border and padding, the last the
 * right ones; `start` is where the pieces begin in `out`. */
pub(super) fn trim_edges(out: &mut [BoxNode], start: usize, inline_at: &[usize]) {
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
