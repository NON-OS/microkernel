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

use crate::frame_pacer::composite;

use crate::state::Context;
use core::sync::atomic::{fence, Ordering};

pub fn tick(ctx: &mut Context) -> Result<(), &'static str> {
    // Composite every damaged rectangle this frame. Each is small and separate,
    // so the empty space between far-apart damage is never touched. A
    // rectangle the display refused stays damaged for the next frame.
    super::drain_damage::drain_damage(ctx, |ctx| &mut ctx.damage, show)
}

/// Compose `rect` of the canvas and put it on the display.
fn show(ctx: &mut Context, rect: crate::state::damage::Rect) -> Result<(), &'static str> {
    composite::paint(ctx, rect);
    /* Composed on the canvas; doubled or shrunk onto the screen when they
     * differ. A canvas larger than the screen is the floor one. */
    let shrunk = ctx.width > ctx.screen.width || ctx.height > ctx.screen.height;
    let on_screen = match ctx.scale {
        _ if shrunk => crate::sw_blitter::downscale(canvas(ctx), ctx.screen, rect),
        1 => Some(rect),
        scale => crate::sw_blitter::upscale(canvas(ctx), ctx.screen, rect, scale),
    };
    let Some(rect) = on_screen else {
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
        if rc < 0 {
            return Err("gop present rejected");
        }
        Ok(())
    } else {
        super::present_virtio::present_rect(ctx, rect)
    }
}

/// The canvas clients draw on, as the blitter reads it.
fn canvas(ctx: &Context) -> crate::sw_blitter::Surface {
    crate::sw_blitter::Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    }
}
