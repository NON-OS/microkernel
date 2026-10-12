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

//! A damaged rectangle composed on the canvas and carried onto the screen:
//! doubled or shrunk when they differ (a canvas larger than the screen is
//! the floor one). None when nothing of it lands on the screen.

use super::composite;
use crate::display::layout_rule::local;
use crate::state::damage::Rect;
use crate::state::Context;

pub fn to_screen(ctx: &mut Context, rect: Rect) -> Option<Rect> {
    composite::paint(ctx, rect);
    // Extended, output 0 shows its slice of the canvas, from its own memory.
    let (view, rect) = match ctx.plan {
        Some(p) => (super::extend::view(ctx, p.slices[0]), local(rect, p.slices[0])?),
        None => (canvas(ctx), rect),
    };
    let shrunk = view.width > ctx.screen.width || view.height > ctx.screen.height;
    match ctx.scale {
        _ if shrunk => crate::sw_blitter::downscale(view, ctx.screen, rect),
        1 if ctx.plan.is_none() => Some(rect),
        scale => crate::sw_blitter::upscale(view, ctx.screen, rect, scale),
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
