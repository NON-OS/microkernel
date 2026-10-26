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
//! Paint the switcher's strip while a run of Alt+Tab presses is up.

use nonos_libc::mk_uptime_ms;

use super::face::{face, Face};
use super::geometry::{cell, first, fits, panel, TILE};
use crate::render::launchpad::gen_icon;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::off_tile::tile_icon;
use crate::render::palette;
use crate::render::panel::{panel as ring, shadow_panel};
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{px, valid_str, UI_PX};
use crate::state::{Context, LAUNCHER_APPS};

pub fn paint_switcher(ctx: &Context) {
    let Some(run) = ctx.switch.session.as_ref().filter(|s| s.live(mk_uptime_ms())) else {
        return;
    };
    let (windows, at) = run.shown();
    let shown = fits(ctx.width, windows.len());
    if shown == 0 {
        return;
    }
    let p = panel(ctx.width, ctx.height, shown);
    shadow_panel(ctx, p, palette::R_CARD, palette::PANEL, palette::LINE);
    let from = first(at, shown);
    for (i, &w) in windows.iter().enumerate().skip(from).take(shown) {
        let c = cell(p, i - from);
        if i == at {
            ring(ctx, c, palette::R_TILE, palette::TILE_ACTIVE, palette::ACCENT);
        }
        let (x, y, size) = (c.x + (c.width - px(TILE)) / 2, c.y + px(10), px(TILE));
        let label = match face(ctx, w) {
            Face::App(a) => {
                tile_icon(ctx, x, y, &LAUNCHER_APPS[a], size);
                LAUNCHER_APPS[a].label
            }
            Face::Named(name) => {
                gen_icon::draw(ctx, x, y, size, name);
                name
            }
        };
        let fg = if i == at { palette::TEXT } else { palette::TEXT_DIM };
        let text = truncate_to_width(valid_str(label), UI_PX, c.width - px(12));
        let tx = c.x + c.width.saturating_sub(measure_aa(text, UI_PX)) / 2;
        text_aa(ctx, tx, y + size + px(10), text, fg, UI_PX);
    }
}
