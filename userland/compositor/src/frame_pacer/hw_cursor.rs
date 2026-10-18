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

//! The pointer on the wire driver's cursor plane (CURSOR, section 12): the
//! arrow drawn into a primary's slot the first time it is shown there and
//! announced with NEW_SHAPE, then the position once a frame to every output
//! when it moved, in place of compositing it and damaging around it. A
//! refusal on any output hands the pointer back to compositing on all of
//! them, so no output shows two arrows or none.

use core::sync::atomic::{fence, Ordering};

use super::cursor_shape::{draw_shape, shape_side};
use super::cursor_slot::slot;
use crate::display::cursor_rule::{message, on_head, on_screen};
use crate::display::ops;
use crate::state::Context;

pub fn sync(ctx: &mut Context) {
    if !ctx.hw_cursor()
        || (ctx.cursor_sent == Some(ctx.lid.pointer(ctx.cursor.current())) && all_shaped(ctx))
    {
        return;
    }
    if send(ctx).is_none() {
        super::hw_cursor_back::give_back(ctx);
    }
}

fn all_shaped(ctx: &Context) -> bool {
    ctx.heads.iter().all(|h| h.shaped)
}

fn send(ctx: &mut Context) -> Option<()> {
    let (port, now, q) = (ctx.gfx_port, ctx.lid.pointer(ctx.cursor.current()), ctx.scale_q);
    let scale = crate::display::scale_rule::whole(q);
    let new = ctx.cursor_sent.is_none();
    let moved = ctx.cursor_sent != Some(now);
    let side = shape_side(scale);
    if new {
        draw_shape(slot(ctx.screen, ctx.wire?.cursor_offset)?, scale)?;
        // The slot's pixels reach memory before the driver copies them.
        fence(Ordering::Release);
    }
    let screen = (ctx.screen.width, ctx.screen.height);
    let p = on_screen((now.x, now.y), (ctx.width, ctx.height), screen, q);
    if moved {
        ops::cursor(port, 0, &message(p, now.visible, new, side)).ok()?;
        ctx.cursor_sent = Some(now);
    }
    for h in ctx.heads.iter_mut().filter(|h| moved || !h.shaped) {
        let shape = !h.shaped;
        if shape {
            draw_shape(slot(h.screen, h.cursor_offset)?, scale)?;
            fence(Ordering::Release);
        }
        let q = on_head(p, screen, (h.screen.width, h.screen.height));
        ops::cursor(port, h.output, &message(q, now.visible, shape, side)).ok()?;
        h.shaped = true;
    }
    Some(())
}
