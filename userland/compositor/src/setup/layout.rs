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

//! The canvas laid out for the shell's choice (unit 10): mirrored, the
//! canvas output 0 alone would have (`fit_canvas`) shown fitted on every
//! output; or extended, one canvas across every output side by side
//! (`layout_rule::plan`), in a mapping of its own. Run on adopt, when the
//! shell chooses, and when outputs come or go; clients learn the new size
//! from DISPLAY_INFO as after a resize. A canvas that cannot be mapped
//! leaves the outputs mirrored.

use nonos_libc::mk_mmap;

use super::canvas_scale::scale_for;
use crate::display::layout_rule::plan;
use crate::display::scale_rule::chosen;
use crate::state::Context;

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

/// Lay the canvas out again from the screens as they are now.
pub fn relayout(ctx: &mut Context) {
    if ctx.want_extend && ctx.hw_cursor() {
        // Extended, the pointer is composited: take it off the plane.
        crate::frame_pacer::hw_cursor_back::give_back(ctx);
    }
    super::canvas_release::release_canvas(ctx);
    let s = ctx.screen;
    (ctx.width, ctx.height, ctx.stride) = (s.width, s.height, s.stride);
    (ctx.backing_va, ctx.backing_len) = (s.base_va, s.byte_len);
    fit(ctx);
}

/// Output 0's canvas, then the extended one when the shell wants it.
pub fn fit(ctx: &mut Context) {
    ctx.plan = None;
    super::canvas::fit_canvas(ctx);
    if !ctx.want_extend || ctx.heads.is_empty() {
        return;
    }
    let scale = |i: usize, w: u32, h: u32| {
        chosen(ctx.output_scale.get(i + 1).copied().flatten(), scale_for(w, h))
    };
    let heads: alloc::vec::Vec<_> = (ctx.heads.iter().enumerate())
        .map(|(i, h)| (h.screen.width, h.screen.height, scale(i, h.screen.width, h.screen.height)))
        .collect();
    let Some(p) = plan((ctx.width, ctx.height), ctx.scale_q, &heads) else { return };
    let stride = p.size.0 * 4;
    let len = stride as u64 * p.size.1 as u64;
    let Ok(size) = usize::try_from(len) else { return };
    let va = mk_mmap(core::ptr::null_mut(), size, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if va.is_null() {
        return;
    }
    super::canvas_release::release_canvas(ctx);
    (ctx.width, ctx.height, ctx.stride) = (p.size.0, p.size.1, stride);
    (ctx.backing_va, ctx.backing_len) = (va as u64, len);
    ctx.plan = Some(p);
    ctx.damage.mark_full(ctx.width, ctx.height);
    ctx.cursor = crate::state::CursorTracker::at(ctx.width / 2, ctx.height / 2);
}
