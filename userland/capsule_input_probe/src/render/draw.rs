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

//! The echoed keys drawn in the roles they are given, so the proofs can draw
//! them under every scheme without the policy store.

use nonos_toolkit::theme::Roles;

use super::super::state::Context;
use super::font;

const SCALE: u32 = 4;

/// The theme's ground over the whole surface, and each echoed key on it in the
/// theme's text.
pub fn draw(ctx: &Context, r: &Roles) {
    fill_rect(ctx, 0, 0, ctx.width, ctx.height, r.bg.argb());
    let mut x0: u32 = 8;
    for i in 0..ctx.cursor {
        draw_glyph(ctx, ctx.buf[i], x0, 8, r.text.argb());
        x0 += font::GLYPH_W * SCALE + SCALE;
    }
}

fn draw_glyph(ctx: &Context, ascii: u8, ox: u32, oy: u32, ink: u32) {
    let rows = font::rows(ascii);
    for (ry, bits) in rows.iter().enumerate() {
        for rx in 0..8u32 {
            if bits & (0x80 >> rx) != 0 {
                fill_rect(ctx, ox + rx * SCALE, oy + ry as u32 * SCALE, SCALE, SCALE, ink);
            }
        }
    }
}

fn fill_rect(ctx: &Context, x: u32, y: u32, w: u32, h: u32, argb: u32) {
    for yy in y..(y + h).min(ctx.height) {
        for xx in x..(x + w).min(ctx.width) {
            let cell = (ctx.base + (yy as u64 * ctx.stride as u64 + xx as u64 * 4)) as *mut u32;
            // SAFETY: ctx.base is a mapped ARGB8888 surface of stride*height bytes
            // exclusively owned by this capsule; the loop clamps xx<width and
            // yy<height so every cell falls inside the mapping.
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}
