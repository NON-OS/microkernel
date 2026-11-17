// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Draw one Launchpad tile: a desktop app uses its own artwork, an installed
//! tool uses its shipped glyph (or a generated tile when it has none), a
//! capsule-store app uses the generated tile since it ships no artwork we
//! embed, and all three carry a centred label underneath.

use super::gen_icon;
use super::grid::{cell_h, cell_origin, cell_w, tile};
use super::hit::Target;
use super::tool_icons;
use crate::apps_off::is_off;
use crate::render::off_tile::tile_icon;
use crate::render::surface::surface;
use crate::render::ui_font;
use crate::render::ui_font::valid_str;
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};
use nonos_toolkit::widgets::{Align, Ink, Label, Rect};

const LABEL_GAP_LOGICAL: u32 = 8;

fn label_gap() -> u32 {
    ui_font::px(LABEL_GAP_LOGICAL)
}

pub(super) fn paint(ctx: &Context, cell: usize, t: Target) {
    let (cx, cy) = cell_origin(ctx.width, cell);
    let icon_x = cx + (cell_w() - tile()) / 2;
    let label: &[u8] = match t {
        Target::App(a) => {
            let app = &LAUNCHER_APPS[a];
            tile_icon(ctx, icon_x, cy, app, tile());
            app.label
        }
        Target::Tool(t) => {
            let tool = &TOOL_APPS[t];
            tool_icons::draw(ctx, icon_x, cy, tile(), tool.label);
            tool.label
        }
        Target::Installed(i) => {
            let Some(name) = ctx.installed_apps.get(i) else { return };
            gen_icon::draw(ctx, icon_x, cy, tile(), name);
            name.as_slice()
        }
        Target::Package(i) => {
            let Some(name) = ctx.pkg_files.get(i) else { return };
            let stem = name.strip_suffix(".nonos").unwrap_or(name.as_str());
            gen_icon::draw(ctx, icon_x, cy, tile(), stem.as_bytes());
            stem.as_bytes()
        }
    };
    let ink = match t {
        Target::App(a) if is_off(LAUNCHER_APPS[a].service) => Ink::Muted,
        _ => Ink::Text,
    };
    paint_label(ctx, label, ink, cx, cy + tile() + label_gap());
}

// The tile label, centred within the cell.
fn paint_label(ctx: &Context, name: &[u8], ink: Ink, cx: u32, band_y: u32) {
    let band_h = cell_h().saturating_sub(tile() + label_gap());
    let band = Rect { x: cx, y: band_y, w: cell_w(), h: band_h };
    Label::new(valid_str(name)).ink(ink).align(Align::Centre).draw(&mut surface(ctx), band);
}
