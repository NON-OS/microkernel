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

//! The translucent tile under a desktop entry, so its name reads on any
//! wallpaper, light or dark.

use super::meta::{meta_text, META_CAP};
use super::metrics::icon;
use super::pill::{pill_pad, pill_vpad};
use crate::render::layout::Rect;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::palette;
use crate::render::ui_font::{line_h, LABEL_PX, META_PX};
use crate::state::Context;

/*
 * Each entry sits on a translucent dark tile. The names were drawn straight
 * onto the wallpaper in light type, which reads on a dark wallpaper and not
 * at all on a light one (a teal or white wallpaper left the names and their
 * counts unreadable). On a dark wallpaper the tile all but vanishes.
 */
const SCRIM: u32 = palette::ICON_SCRIM;

// The tile under one entry: as wide as the widest of its mark and its two
// lines, from just above the mark to just below the second line.
pub(super) fn scrim(
    ctx: &Context,
    item: &crate::vfs_client::Entry,
    cx: u32,
    cy: u32,
    cw: u32,
    label_y: u32,
) {
    let shown = truncate_to_width(&item.name, LABEL_PX, cw.saturating_sub(pill_pad() * 2));
    let mut buf = [0u8; META_CAP];
    let meta = meta_text(item, &mut buf);
    let widest = measure_aa(shown, LABEL_PX).max(measure_aa(meta, META_PX)).max(icon());
    let width = (widest + pill_pad() * 2).min(cw);
    let top = cy.saturating_sub(pill_pad());
    let bottom = label_y + line_h(LABEL_PX) + line_h(META_PX) + pill_vpad() * 2;
    let rect = Rect { x: cx + (cw - width) / 2, y: top, width, height: bottom - top };
    crate::render::panel::round_fill(ctx, rect, palette::R_TILE, SCRIM);
}
