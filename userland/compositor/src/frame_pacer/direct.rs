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

//! A frame on the plane with no composite: the rule (state/direct_rule.rs)
//! names a client buffer, the band is composed onto the primary where it
//! lies, and SCANOUT hands the buffer and the band to the driver. Any
//! refusal composes this frame whole; the plane keeps what it showed until
//! that FLUSH lands, and the buffer is released only after it. A layer
//! drawing a pinned client buffer over its own is scanned out from that
//! buffer when it alone is seen (direct_rule.rs seen).

use core::sync::atomic::{fence, Ordering};

use nonos_gfx_wire::body::surface::ARGB8888;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::{Rect, Scanout};

use super::indicator_band::band;
use super::to_screen::to_screen;
use crate::display::WireLink;
use crate::state::attach::{Attached, MAX_ATTACH};
use crate::state::attach_kernel::Kernel;
use crate::state::direct_rule::{pick, refusal, seen, way, Facts, Over, Refusal, Top, Way};
use crate::state::Context;
use crate::sw_blitter::Surface;

/// Some(result) when the frame went to the plane (or had nothing to send);
/// None when it composes, its damage made whole when the plane must come
/// back to the primary.
pub fn frame(ctx: &mut Context) -> Option<Result<(), &'static str>> {
    let link = ctx.wire?;
    let want = want(ctx, &link);
    match way(want.map(|w| w.0), ctx.direct.plane, !ctx.damage.is_empty()) {
        Way::Idle => Some(Ok(())),
        Way::Scan(_) => match want.ok_or(()).and_then(|w| scan(ctx, &link, w)) {
            Ok(()) => Some(Ok(())),
            Err(()) => {
                if ctx.direct.plane.is_some() {
                    ctx.damage.mark_full(ctx.width, ctx.height);
                }
                None
            }
        },
        Way::Compose { whole } => {
            if whole {
                ctx.damage.mark_full(ctx.width, ctx.height);
            }
            None
        }
    }
}

/// The buffer the rule picks, with its surface as attached: the top
/// layer's own, or the pinned client buffer it draws over it.
fn want(ctx: &mut Context, link: &WireLink) -> Option<(u64, Surface)> {
    let f = Facts {
        able: ctx.direct.able(link.pid),
        screen: (ctx.width, ctx.height),
        plain: plain(ctx),
        cursor_plane: link.cursor_offset != 0,
    };
    if !f.able || !f.plain || !f.cursor_plane {
        return None;
    }
    let (layers, n) = ctx.scene.z_sorted_snapshot();
    let top = *layers.get(..n)?.last()?;
    // Every surface the scene draws from, so no lookup here lets go of one.
    let mut in_scene = [0u64; 2 * MAX_ATTACH];
    for (i, l) in layers.iter().take(n.min(MAX_ATTACH)).enumerate() {
        (in_scene[2 * i], in_scene[2 * i + 1]) = (l.surface_handle, l.content.handle);
    }
    let in_scene = &in_scene[..2 * n.min(MAX_ATTACH)];
    let Attached::Live(s) = ctx.attach.lookup(top.surface_handle, in_scene, &mut Kernel) else {
        return None;
    };
    let own = Top {
        handle: top.surface_handle,
        at: (top.x, top.y),
        size: (top.width, top.height),
        buffer: (s.width, s.height),
        stride: s.stride,
        opaque: ctx.direct.opaque(top.surface_handle),
        refused: ctx.direct.refused(top.surface_handle),
    };
    let c = top.content;
    let (over, shown) = if c.handle == 0 {
        (None, s)
    } else {
        let Attached::Live(cs) = ctx.attach.lookup(c.handle, in_scene, &mut Kernel) else {
            return None;
        };
        let over = Over {
            handle: c.handle,
            at: (c.x, c.y),
            buffer: (cs.width, cs.height),
            stride: cs.stride,
            opaque: c.opaque,
            refused: ctx.direct.refused(c.handle),
            whole: c.view == (0, 0, cs.width, cs.height),
        };
        (Some(over), cs)
    };
    let t = seen(own, over)?;
    pick(&f, Some(&t)).map(|h| (h, shown))
}

/// The canvas is the screen, one to one, on one output, with no look.
fn plain(ctx: &Context) -> bool {
    let l = &ctx.looks;
    let screen = ctx.screen;
    let in_place = screen.base_va == ctx.backing_va && (screen.width, screen.height) == (ctx.width, ctx.height);
    let looks = l.night.is_none() && l.looks.iter().all(Option::is_none) && l.lens.iter().all(Option::is_none);
    in_place && ctx.scale_q == 4 && ctx.plan.is_none() && ctx.heads.is_empty() && looks
}

fn scan(ctx: &mut Context, link: &WireLink, (handle, s): (u64, Surface)) -> Result<(), ()> {
    // The band's pixels, current on the primary, for the driver's plane.
    let b = band(ctx.width, ctx.height);
    let since = ctx.trace.mark();
    let _ = to_screen(ctx, b);
    ctx.trace.composed(since, b);
    fence(Ordering::Release);
    let req = Scanout {
        handle,
        serial: link.serial,
        width: s.width,
        height: s.height,
        stride: s.stride,
        format: ARGB8888,
        band: Rect { x: b.x, y: b.y, width: b.width, height: b.height },
    };
    let since = ctx.trace.mark();
    let answer = crate::display::ops::scanout(ctx.gfx_port, 0, &req);
    ctx.trace.flipped(since);
    match answer {
        Ok(()) => {
            ctx.direct.plane = Some(handle);
            while ctx.damage.drain().is_some() {}
            ctx.trace.scanned(handle);
            Ok(())
        }
        Err(e) => {
            match refusal(e) {
                Refusal::Driver => ctx.direct.refuse_driver(),
                Refusal::Buffer => ctx.direct.refuse(handle),
                Refusal::Frame if e == ESTALE => {
                    if let Some(w) = ctx.wire.as_mut() {
                        w.stale = true;
                    }
                }
                Refusal::Frame => {}
            }
            Err(())
        }
    }
}
