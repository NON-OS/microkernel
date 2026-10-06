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
 * The Appearance step: which wallpapers this machine keeps, and which one
 * is the desktop's. Only the wallpapers kept are ever read out of the
 * store's collection, so one not kept never comes into a session; Settings
 * changes the choice later. Every wallpaper is kept until one is dropped.
 */

use alloc::format;

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;
use nonos_policy_proto::wallpapers_kept::{kept, ALL};

use crate::render::layout::WALL_AFTER_LINES;
use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

use super::appearance_rows;

/// The desktop's wallpaper until another is chosen: Special Variant 09, the
/// one the wallpaper capsule carries for the first frame and the policy
/// store's own default.
pub const DESKTOP_DEFAULT: u8 = 55;

const K_SPACE: u32 = 0x20;
const K_A: u32 = 0x61;
const K_N: u32 = 0x6E;

const KEEPING: &[&[u8]] = &[
    b"Only the wallpapers kept are ever read into a session.",
    b"A keeps them all, N keeps only the desktop's. Settings changes this later.",
];

/// The desktop's wallpaper, by catalog index.
pub fn wallpaper(ctx: &Context) -> u8 {
    ctx.wall_desktop
}

/// The desktop's wallpaper, by name.
pub fn name(ctx: &Context) -> &'static [u8] {
    WALLPAPER_LABELS.get(ctx.wall_desktop as usize).copied().unwrap_or(b"")
}

/// How many wallpapers are kept.
pub fn count(ctx: &Context) -> u32 {
    ctx.walls_kept.count_ones()
}

pub fn draw(ctx: &Context) {
    let sub = b"SPACE keeps or drops one. ENTER makes it the desktop's and goes on.";
    render::frame(ctx, b"Appearance", sub, b"SPACE KEEP  ENTER DESKTOP  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let room = l.line_rows(WALL_AFTER_LINES, 2);
    let y = appearance_rows::draw(buf, spx, w, h, x, l.body_y, ctx, room);
    let total = WALLPAPER_LABELS.len();
    let kept_line = format!("{} of {total} kept; the desktop's is {}.", count(ctx), shown(name(ctx)));
    let y = lines::text(buf, spx, w, h, x, y + l.gap, &[kept_line.as_bytes()], FG);
    lines::text(buf, spx, w, h, x, y + l.gap, KEEPING, HINT);
}

fn shown(name: &[u8]) -> &str {
    core::str::from_utf8(name).unwrap_or("")
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let rows = WALLPAPER_LABELS.len() as u8;
    if let Some(o) = list_nav(&mut ctx.wall_sel, rows, code) {
        return o;
    }
    let sel = ctx.wall_sel;
    match code {
        // The desktop's wallpaper stays kept; dropping it would leave the
        // desktop showing one the session may not read.
        K_SPACE if sel != ctx.wall_desktop => ctx.walls_kept ^= 1u64 << sel,
        K_SPACE => {}
        K_A => ctx.walls_kept = ALL,
        K_N => ctx.walls_kept = 1u64 << ctx.wall_desktop,
        _ => {
            let o = default_key(code);
            if matches!(o, Outcome::Advance) {
                ctx.wall_desktop = sel;
                ctx.walls_kept |= 1u64 << sel;
            }
            return o;
        }
    }
    debug_assert!(kept(ctx.walls_kept, ctx.wall_desktop));
    Outcome::Stay
}
