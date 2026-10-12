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

//! Presenting an extended canvas (unit 10): each composed rectangle's part
//! in a further output's slice is drawn onto that output at the canvas's
//! scale and kept, and once a frame each output gets its own FLUSH of what
//! was drawn on it (at most fifteen rectangles; more become their bounds).

use nonos_gfx_wire::body::flush::MAX_RECTS;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::Rect as WireRect;

use crate::display::layout_rule::{local, Slice};
use crate::display::ops;
use crate::state::damage::Rect;
use crate::state::Context;
use crate::sw_blitter::{scaled, Surface};

/// The canvas as slice `s` of it: its own origin, the same rows.
pub fn view(ctx: &Context, s: Slice) -> Surface {
    let off = s.x as u64 * 4;
    let byte_len = ctx.backing_len.saturating_sub(off);
    Surface { base_va: ctx.backing_va + off, stride: ctx.stride, width: s.w, height: s.h, byte_len }
}

pub fn draw_heads(ctx: &mut Context, r: Rect) {
    let Some(p) = ctx.plan else { return };
    for (i, s) in p.slices.iter().take(p.n).enumerate().skip(1) {
        let (Some(h), Some(l)) = (ctx.heads.get(i - 1).copied(), local(r, *s)) else { continue };
        if let Some(d) = scaled(view(ctx, *s), h.screen, l, s.q) {
            ctx.head_pending.push((i - 1, d));
        }
    }
}

pub fn flush_heads(ctx: &mut Context) {
    let pending = core::mem::take(&mut ctx.head_pending);
    for (i, h) in ctx.heads.iter_mut().enumerate() {
        let mut rects = [WireRect { x: 0, y: 0, width: 0, height: 0 }; MAX_RECTS];
        let mut n = 0;
        for d in pending.iter().filter(|p| p.0 == i).map(|p| p.1) {
            let w = WireRect { x: d.x, y: d.y, width: d.width, height: d.height };
            if n < MAX_RECTS {
                (rects[n], n) = (w, n + 1);
            } else {
                rects[MAX_RECTS - 1] = bounds(rects[MAX_RECTS - 1], w);
            }
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
        if n > 0 && ops::flush(ctx.gfx_port, h.output, h.serial, &rects[..n]) == Err(ESTALE) {
            h.stale = true;
        }
    }
}

fn bounds(a: WireRect, b: WireRect) -> WireRect {
    let (x0, y0) = (a.x.min(b.x), a.y.min(b.y));
    let x1 = (a.x + a.width).max(b.x + b.width);
    let y1 = (a.y + a.height).max(b.y + b.height);
    WireRect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 }
}
