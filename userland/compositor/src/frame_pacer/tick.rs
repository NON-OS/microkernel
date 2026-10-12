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

use crate::state::Context;
use core::sync::atomic::{fence, Ordering};

use super::to_screen::to_screen;

/// MkSurfacePresent's EBUSY: driver.bootfb0 holds the boot framebuffer.
const GOP_HELD: i64 = -16;

pub fn tick(ctx: &mut Context) -> Result<(), &'static str> {
    // Composite every damaged rectangle this frame. Each is small and separate,
    // so the empty space between far-apart damage is never touched. A
    // rectangle the display refused stays damaged for the next frame. On the
    // wire a frame goes in batches, one FLUSH each.
    if ctx.wire.is_some() {
        super::hw_cursor::sync(ctx);
        let prepare = |ctx: &mut Context, r| {
            to_screen(ctx, r).and_then(|r| super::clip::clip_to_screen(ctx, r))
        };
        let send = super::present_wire::present_rects;
        return super::drain_batch::drain_batched(ctx, |ctx| &mut ctx.damage, prepare, send);
    }
    super::drain_damage::drain_damage(ctx, |ctx| &mut ctx.damage, show)
}

/// Compose `rect` of the canvas and put it on the display.
fn show(ctx: &mut Context, rect: crate::state::damage::Rect) -> Result<(), &'static str> {
    let Some(rect) = to_screen(ctx, rect) else {
        return Ok(());
    };
    if ctx.gop_mode {
        // The kernel copies whatever it is handed with the CPU, so give it
        // the rectangle that changed, not the screen.
        let Some(r) = super::clip::clip_to_screen(ctx, rect) else {
            return Ok(());
        };
        fence(Ordering::Release);
        let rc =
            nonos_libc::mk_surface_present_rect(ctx.surface_handle, r.x, r.y, r.width, r.height);
        if rc == GOP_HELD {
            // bootfb took the framebuffer: the next upkeep finds it.
            ctx.gop_held = true;
        }
        if rc < 0 {
            return Err("gop present rejected");
        }
        Ok(())
    } else {
        super::present_virtio::present_rect(ctx, rect)
    }
}
