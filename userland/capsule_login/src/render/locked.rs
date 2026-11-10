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

//! The lock screen drawn in the roles it is given, so the proofs can draw it
//! under every scheme without the policy store.

use nonos_toolkit::decorations::TRANSPARENT;
use nonos_toolkit::theme::Roles;

use crate::state::Context;

/// The bar's top edge and its height, in pixels.
const BAR_TOP: u32 = 0x20;
const BAR_H: u32 = 8;

/// The theme's ground over the whole screen, and its accent in a bar across it.
pub fn draw_locked(ctx: &Context, r: &Roles) {
    fill(ctx.backing_va, ctx.width, ctx.height, ctx.stride, r.bg.argb());
    draw_bar(ctx, r.accent.argb());
}

/// The overlay sits over the wallpaper and under the desktop; when no lock is
/// showing it is clear, so the wallpaper shows through it.
pub fn draw_clear(ctx: &Context) {
    fill(ctx.backing_va, ctx.width, ctx.height, ctx.stride, TRANSPARENT);
}

fn draw_bar(ctx: &Context, argb: u32) {
    if ctx.width < 32 || ctx.height < BAR_TOP + 16 {
        return;
    }
    let start_x = 16u32;
    let end_x = ctx.width.saturating_sub(16);
    for y in BAR_TOP..BAR_TOP + BAR_H {
        for x in start_x..end_x {
            let px = unsafe { pixel_mut(ctx.backing_va, ctx.stride, x, y) };
            unsafe {
                core::ptr::write_volatile(px, argb);
            }
        }
    }
}

fn fill(base: u64, width: u32, height: u32, stride: u32, argb: u32) {
    for y in 0..height {
        for x in 0..width {
            let px = unsafe { pixel_mut(base, stride, x, y) };
            unsafe {
                core::ptr::write_volatile(px, argb);
            }
        }
    }
}

unsafe fn pixel_mut(base: u64, stride: u32, x: u32, y: u32) -> *mut u32 {
    (base as usize + y as usize * stride as usize + x as usize * 4) as *mut u32
}
