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

/*
 * The Appearance step's list: every wallpaper in the collection, one line
 * each, marked kept or not and the desktop's named. Where the canvas has
 * room for fewer than all, a run of them that keeps the highlighted row in
 * view is shown, and a line under it says which rows those are.
 */

use alloc::format;

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;
use nonos_policy_proto::wallpapers_kept::kept;

use crate::render::ink::{draw_label, draw_text};
use crate::render::layout::{layout, window, WALL_CELLS};
use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, HINT, ROW_SEL_BG};
use crate::state::Context;

/* Draws at most `room` rows from `y`; returns the y below them. */
pub fn draw(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    ctx: &Context,
    room: u32,
) -> u32 {
    let l = layout(w, h);
    let rows = WALLPAPER_LABELS.len() as u32;
    let (first, shown) = window(rows, room, ctx.wall_sel as u32);
    for at in 0..shown {
        let i = (first + at) as usize;
        let top = y + l.line_h * at;
        let is_kept = kept(ctx.walls_kept, i as u8);
        let color = if is_kept { FG } else { HINT };
        if ctx.wall_sel as usize == i {
            fill_rect(buf, spx, w, h, x, top, l.list_w, l.line_h, ROW_SEL_BG);
            draw_text(buf, spx, w, h, x + l.unit, top, b">", ACCENT);
        }
        let mark: &[u8] = if is_kept { b"kept" } else { b"" };
        let desktop: &[u8] = if ctx.wall_desktop as usize == i { b"desktop" } else { b"" };
        let cells: [&[u8]; 3] = [mark, WALLPAPER_LABELS[i], desktop];
        for (cell, at) in cells.iter().zip(WALL_CELLS) {
            draw_text(buf, spx, w, h, x + at * l.unit, top, cell, color);
        }
    }
    let end = y + l.line_h * shown;
    if shown == rows {
        return end;
    }
    let s = format!("rows {} to {} of {rows}, Up and Down scroll", first + 1, first + shown);
    draw_label(buf, spx, w, h, x, end + l.unit / 2, s.as_bytes(), HINT);
    end + l.line_h
}
