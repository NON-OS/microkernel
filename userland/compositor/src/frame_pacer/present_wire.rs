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

//! FLUSH on the wire: a frame's rectangles, already clipped to the screen
//! (one outside the primary is refused, and a refused batch is damaged
//! again, so an unclipped one would be refused every frame), made visible
//! before the answer. ESTALE also marks the primary for the poll to ask
//! for again.

use core::sync::atomic::{fence, AtomicU8, Ordering};

use nonos_gfx_wire::body::flush::MAX_RECTS;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::Rect;

use super::present_note::{bbox, lit, note, NOTES};
use crate::display::ops;
use crate::server::runner::breadcrumb::step;
use crate::server::runner::stall_step::Step;
use crate::state::damage::Rect as Damage;
use crate::state::Context;

/// Presents said so far (`present_note.rs`).
static NOTED: AtomicU8 = AtomicU8::new(0);

pub fn present_rects(ctx: &mut Context, rects: &[Damage]) -> Result<(), &'static str> {
    let Some(link) = ctx.wire else {
        return Err("display: no wire driver");
    };
    let zero = Rect { x: 0, y: 0, width: 0, height: 0 };
    let mut wire = [zero; MAX_RECTS];
    let n = rects.len().min(MAX_RECTS);
    for (w, r) in wire.iter_mut().zip(&rects[..n]) {
        *w = Rect { x: r.x, y: r.y, width: r.width, height: r.height };
    }
    // The composed pixels reach memory before the driver copies them.
    fence(Ordering::Release);
    step(Step::Flush, nonos_libc::mk_uptime_ms());
    let answer = ops::flush(ctx.gfx_port, 0, link.serial, &wire[..n]);
    step(Step::Compose, nonos_libc::mk_uptime_ms());
    if NOTED.load(Ordering::Relaxed) < NOTES {
        let k = NOTED.fetch_add(1, Ordering::Relaxed) + 1;
        let seen = lit(ctx.screen, bbox(&rects[..n]));
        crate::say::say(&note(k, &rects[..n], link.serial, seen, answer.is_ok()));
    }
    match answer {
        Ok(()) => {
            super::mirror::mirror(ctx, &rects[..n]);
            Ok(())
        }
        Err(ESTALE) => {
            if let Some(w) = ctx.wire.as_mut() {
                w.stale = true;
            }
            Err("display: primary stale")
        }
        Err(_) => Err("display: FLUSH refused"),
    }
}
