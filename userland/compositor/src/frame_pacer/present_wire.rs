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

//! FLUSH on the wire: the rectangle, clipped to the screen (one outside the
//! primary is refused, and a refused rectangle is damaged again, so an
//! unclipped one would be refused every frame), made visible before the
//! answer. On any error the caller damages it again; ESTALE also marks the
//! primary for the poll to ask for again.

use core::sync::atomic::{fence, Ordering};

use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::Rect;

use crate::display::ops;
use crate::state::Context;

pub fn present_rect(
    ctx: &mut Context,
    rect: crate::state::damage::Rect,
) -> Result<(), &'static str> {
    let Some(link) = ctx.wire else {
        return Err("display: no wire driver");
    };
    let Some(r) = super::clip::clip_to_screen(ctx, rect) else {
        return Ok(());
    };
    // The composed pixels reach memory before the driver copies them.
    fence(Ordering::Release);
    let rect = Rect { x: r.x, y: r.y, width: r.width, height: r.height };
    match ops::flush(ctx.gfx_port, link.serial, &[rect]) {
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
