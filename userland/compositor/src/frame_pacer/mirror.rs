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

//! A frame's rectangles, once on the screen, onto every mirrored output:
//! each drawn there fitted and centred (`sw_blitter::resample`), then one
//! FLUSH per output; in their own colours (`display::mirror_rule`) each is
//! drawn from the true-colour canvas and given its own look. A mirrored
//! output that refuses is left to the poll
//! (ESTALE marks it for SURFACE again); the screen's own present stands.

use core::sync::atomic::{fence, Ordering};

use nonos_gfx_wire::body::flush::MAX_RECTS;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::Rect;

use crate::display::ops;
use crate::state::damage::Rect as Damage;
use crate::state::Context;
use crate::sw_blitter::{fitted, resample};

pub fn mirror(ctx: &mut Context, rects: &[Damage]) {
    if ctx.plan.is_some() {
        // Extended: each output was drawn from its own slice (`extend`).
        return;
    }
    let (port, own, canvas) = (ctx.gfx_port, ctx.own_colours(), super::to_screen::canvas(ctx));
    let size = (canvas.width, canvas.height);
    for (i, h) in ctx.heads.iter_mut().enumerate() {
        let zero = Rect { x: 0, y: 0, width: 0, height: 0 };
        let mut out = [zero; MAX_RECTS];
        let mut n = 0;
        for r in rects.iter().take(MAX_RECTS) {
            // In their own colours, from the true-colour canvas, then the look.
            let shown = match own {
                false => resample(ctx.screen, h.screen, *r),
                true => fitted((ctx.screen.width, ctx.screen.height), size, *r)
                    .and_then(|c| resample(canvas, h.screen, c)),
            };
            if let (true, Some(d), Some(l)) =
                (own, shown, ctx.looks.looks.get(i + 1).and_then(|l| l.as_deref()))
            {
                super::own_look::lay(h.screen, d, size, l);
            }
            if let Some(d) = shown {
                out[n] = Rect { x: d.x, y: d.y, width: d.width, height: d.height };
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        fence(Ordering::Release);
        if ops::flush(port, h.output, h.serial, &out[..n]) == Err(ESTALE) {
            h.stale = true;
        }
    }
}
