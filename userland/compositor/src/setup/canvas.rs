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

//! The canvas clients draw on, and the screen it reaches.
//!
//! Applications draw one pixel per pixel at fixed sizes. On a panel of 2560
//! by 1440 or more, a laptop's at about 220 per inch, that made every window
//! and its text half the size it was drawn to be. Such a screen gets a canvas
//! half as wide and tall: clients are told its size, everything is composed
//! onto it as before, and each damaged rectangle is doubled onto the screen
//! when it is presented. A screen under 1024x720 gets a canvas that covers
//! it, shrunk when presented (canvas_floor). Otherwise the canvas is the
//! screen, unless output 0 is magnified.

use nonos_libc::mk_mmap;

use super::canvas_floor::floor_canvas;
use super::canvas_scale::scale_for_panel;
use super::panel_mm::panel_mm;
use crate::display::scale_rule::{canvas_for, chosen};
use crate::state::Context;
use crate::sw_blitter::Surface;

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

/// Make the buffer `ctx` presents from its screen, and give clients a canvas
/// for it; one that cannot be mapped leaves the screen as the canvas.
pub fn fit_canvas(ctx: &mut Context) {
    ctx.screen = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    };
    ctx.scale_q = 4;
    let panel = scale_for_panel(ctx.width, ctx.height, panel_mm(ctx));
    let q = chosen(ctx.output_scale[0], panel);
    let (width, height) = match (q, floor_canvas(ctx.width, ctx.height)) {
        (4, None) if !ctx.looks.access[0].magnified() && !ctx.own_colours() => return,
        // The lens, or mirrors in their own colours, read a canvas of their own.
        (4, None) => (ctx.width, ctx.height),
        (4, Some(floor)) => floor,
        (q, _) => canvas_for(ctx.width, ctx.height, q),
    };
    let stride = width * 4;
    let byte_len = stride as u64 * height as u64;
    let Ok(size) = usize::try_from(byte_len) else { return };
    let va = mk_mmap(core::ptr::null_mut(), size, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if va.is_null() {
        return;
    }
    ctx.scale_q = q;
    ctx.width = width;
    ctx.height = height;
    ctx.stride = stride;
    ctx.backing_va = va as u64;
    ctx.backing_len = byte_len;
    ctx.damage.mark_full(width, height);
    ctx.cursor = crate::state::CursorTracker::at(width / 2, height / 2);
}
