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

//! The cursor plane's position, and its shape when it is owed one, to the
//! primary and then to every other output.

use core::sync::atomic::{fence, Ordering};

use super::cursor_shape::{draw_shape, shape_hot, shape_side};
use super::cursor_slot::slot;
use crate::display::cursor_rule::{message, on_head, on_screen};
use crate::display::ops;
use crate::state::Context;

pub(super) fn send(ctx: &mut Context) -> Option<()> {
    let (port, now, q) = (ctx.gfx_port, ctx.lid.pointer(ctx.cursor.current()), ctx.scale_q);
    let scale = crate::display::scale_rule::whole(q);
    let new = ctx.cursor_sent.is_none();
    let moved = ctx.cursor_sent != Some(now);
    let code = ctx.cursor_code;
    let (side, hot) = (shape_side(scale, code), shape_hot(scale, code));
    if new {
        draw_shape(slot(ctx.screen, ctx.wire?.cursor_offset)?, scale, code)?;
        // The slot's pixels reach memory before the driver copies them.
        fence(Ordering::Release);
    }
    let screen = (ctx.screen.width, ctx.screen.height);
    let p = on_screen((now.x, now.y), (ctx.width, ctx.height), screen, q);
    if moved {
        ops::cursor(port, 0, &message(p, now.visible, new, side, hot)).ok()?;
        ctx.cursor_sent = Some(now);
    }
    for h in ctx.heads.iter_mut().filter(|h| moved || !h.shaped) {
        let shape = !h.shaped;
        if shape {
            draw_shape(slot(h.screen, h.cursor_offset)?, scale, code)?;
            fence(Ordering::Release);
        }
        let q = on_head(p, screen, (h.screen.width, h.screen.height));
        ops::cursor(port, h.output, &message(q, now.visible, shape, side, hot)).ok()?;
        h.shaped = true;
    }
    Some(())
}
