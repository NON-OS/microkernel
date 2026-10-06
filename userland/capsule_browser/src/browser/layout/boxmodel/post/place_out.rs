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

use super::super::abs_out_of_flow::{out_of_flow, positioned};
use super::super::ctx::Ctx;
use super::super::display_list::DisplayList;
use super::super::geom::overflow_clip::overflow_clip;
use super::super::layout_block::MAX_DEPTH;
use super::super::tree::BoxNode;
use super::place_one::{place_one, Cb};

/// Once a positioned box has its final size (its fragment is the one at
/// `start`), place the absolutes it contains against its padding box and
/// overflow clip; a transformed box contains the fixed ones too.
pub(crate) fn place_inside(n: &BoxNode, frags: &mut DisplayList, start: usize, d: u32, ctx: Ctx) {
    let Some(f) = frags.get(start).filter(|f| f.node == n.dom_id) else { return };
    let s = &n.style;
    let (bl, bt) = (s.border_left as i32, s.border_top as i32);
    let w = (f.w - bl - s.border_right as i32).max(0);
    let h = (f.h - bt - s.border_bottom as i32).max(0);
    let mut inner = ctx;
    if let Some(c) = overflow_clip(s, [f.x, f.y, f.w], Some(f.h)) {
        inner = inner.clip_box(s, c, f.w);
    }
    let cb = Cb { r: [f.x + bl, f.y + bt, w, h], ctx: inner };
    let passes: &[bool] = if s.fx.transform.is_some() { &[false, true] } else { &[false] };
    for &fixed in passes {
        n.children.iter().for_each(|c| visit(c, cb, frags, d + 1, fixed));
    }
}

/// Place what the viewport contains once the page is laid out: absolutes
/// with no positioned ancestor, then fixed boxes with no transformed one.
pub(crate) fn place_root(root: &BoxNode, frags: &mut DisplayList, ctx: Ctx) {
    let cb = Cb { r: [0, 0, ctx.vp.0, ctx.vp.1], ctx };
    visit(root, cb, frags, 0, false);
    visit(root, cb, frags, 0, true);
}

/* One pass over a subtree for the fixed boxes, or else the absolute ones.
 * A positioned box places its own absolutes, so that pass stops there; the
 * fixed pass goes on through everything a transform does not claim. */
fn visit(c: &BoxNode, cb: Cb, frags: &mut DisplayList, d: u32, fixed: bool) {
    if d > MAX_DEPTH {
        return;
    }
    let s = &c.style;
    if out_of_flow(s) && s.is_fixed == fixed {
        place_one(c, cb, frags, d);
    }
    let stop = if fixed { s.fx.transform.is_some() } else { positioned(s) };
    if !stop {
        for k in &c.children {
            visit(k, cb, frags, d + 1, fixed);
        }
    }
}
