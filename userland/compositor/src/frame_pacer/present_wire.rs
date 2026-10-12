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

use core::sync::atomic::{fence, Ordering};

use nonos_gfx_wire::body::flush::MAX_RECTS;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::Rect;

use crate::display::ops;
use crate::state::damage::Rect as Damage;
use crate::state::Context;

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
    match ops::flush(ctx.gfx_port, link.serial, &wire[..n]) {
        Ok(()) => Ok(()),
        Err(ESTALE) => {
            if let Some(w) = ctx.wire.as_mut() {
                w.stale = true;
            }
            Err("display: primary stale")
        }
        Err(_) => Err("display: FLUSH refused"),
    }
}
