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
 * How an app setup turned off looks wherever it has a tile: a grey cell, its
 * mark at a third of its strength, and a small "off" disc in the corner, so
 * it reads as off before a click says so.
 */

use super::icons::{badge, icon_bytes};
use super::palette;
use super::surface::surface;
use crate::apps_off::{dim, is_off};
use crate::state::apps::LauncherApp;
use crate::state::Context;

/* The disc's bar: the dock's own dark, opaque. */
const BAR_FILL: u32 = palette::PANEL | 0xFF00_0000;

/* A dock tile's fill when its app is neither open nor focused. */
pub fn idle_fill(service: &[u8]) -> u32 {
    if is_off(service) {
        palette::TILE_OFF
    } else {
        palette::TILE_FILL
    }
}

/* A dock app's mark, over the cell the dock painted for it. */
pub fn dock_glyph(ctx: &Context, x: u32, y: u32, app: &LauncherApp, size: u32) {
    let off = is_off(app.service);
    let tint = if off { dim(palette::ACCENT) } else { palette::ACCENT };
    badge::glyph(ctx, x, y, size, icon_bytes(app.icon), tint);
    if off {
        marker(ctx, x, y, size);
    }
}

/* A Launchpad app tile, which brings its own backing. */
pub fn tile_icon(ctx: &Context, x: u32, y: u32, app: &LauncherApp, size: u32) {
    if !is_off(app.service) {
        super::draw_app_icon(ctx, x, y, app.icon, size);
        return;
    }
    let tint = dim(palette::ACCENT);
    badge::tiled(ctx, x, y, size, icon_bytes(app.icon), tint, palette::TILE_OFF);
    marker(ctx, x, y, size);
}

/* The "off" mark: a grey disc with a bar across it, in the top right corner. */
fn marker(ctx: &Context, x: u32, y: u32, size: u32) {
    let r = (size / 9).max(3);
    let edge = size / 12;
    let (cx, cy) = (x + size - edge - r, y + edge + r);
    let bar_h = (r / 3).max(2);
    let mut buf = surface(ctx);
    buf.circle(cx, cy, r, palette::TEXT_DIM);
    buf.fill_rect(cx - r / 2, cy - bar_h / 2, r, bar_h, BAR_FILL);
}
