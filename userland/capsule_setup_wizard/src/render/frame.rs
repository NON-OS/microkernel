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

//! A setup screen's frame: the brand panel, then in the column beside it the
//! step marker, the title, the line under it, and the keys under a full rule,
//! each where the canvas's layout puts it.

use alloc::format;

use crate::render::ink::{draw_label, draw_marker, draw_text, draw_title};

use crate::render::canvas::Canvas;
use crate::render::chrome;
use crate::render::layout::{layout, Layout};
use crate::render::paint::fill_rect;
use crate::render::theme::{self, ACCENT, BACKDROP, FG, HINT};
use crate::state::Context;

pub fn buffer(ctx: &Context) -> &'static mut [u32] {
    let spx = ctx.stride as usize / 4;
    unsafe { core::slice::from_raw_parts_mut(ctx.base as *mut u32, spx * ctx.height as usize) }
}

/// The layout of this setup's canvas.
pub fn layout_of(ctx: &Context) -> Layout {
    layout(ctx.width, ctx.height)
}

/// This setup's canvas, over the buffer above.
pub fn canvas(ctx: &Context) -> Canvas<'static> {
    let spx = ctx.stride as usize / 4;
    Canvas { buf: buffer(ctx), spx, w: ctx.width, h: ctx.height }
}

pub fn frame(ctx: &Context, title: &[u8], sub: &[u8], footer: &[u8]) {
    let w = ctx.width;
    let l = layout_of(ctx);
    let cv = &mut canvas(ctx);
    for p in cv.buf.iter_mut() {
        *p = BACKDROP;
    }
    chrome::panel(cv.buf, cv.spx, &l, ctx);
    let x = l.col_x;
    let step = format!("STEP {:02} OF {:02}", ctx.step + 1, theme::STEP_LABELS.len());
    draw_marker(cv, x, l.marker_y, &step, ACCENT, HINT);
    draw_title(cv, x, l.title_y, title, FG, l.title_px);
    draw_text(cv, x, l.sub_y, sub, theme::SUB);
    let hair = l.scale.px(1).max(1);
    fill_rect(cv, l.panel_w + 1, l.foot_y, w, hair, theme::RULE);
    draw_label(cv, x, l.keys_y, footer, HINT);
}
