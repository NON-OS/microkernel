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

//! Where the status cluster and the magnifier in it start, from the widths
//! of what the cluster holds (`search_box`).

use super::glyph_w::{net_glyph_w, search_glyph_w};
use super::metrics::{gap, right_margin};
use super::search_box::battery_w;
use crate::render::layout::menubar_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::{scale, STATUS_PX};
use crate::state::Context;

pub(super) fn total(btext: &[u8], when: &[u8]) -> u32 {
    battery_w(btext)
        + net_glyph_w()
        + gap()
        + search_glyph_w()
        + gap()
        + measure_aa_bytes(when, STATUS_PX)
}

/// Where the status cluster starts, or None when the bar has no room for it.
pub(super) fn cluster_x(ctx: &Context, btext: &[u8], when: &[u8]) -> Option<u32> {
    let bar = menubar_rect(ctx.width);
    let span = total(btext, when);
    if bar.width <= span + right_margin() {
        return None;
    }
    Some(bar.x + bar.width - right_margin() - span)
}

pub(super) fn search_box(ctx: &Context, btext: &[u8], when: &[u8]) -> Option<(u32, u32, u32)> {
    let bar = menubar_rect(ctx.width);
    let x = cluster_x(ctx, btext, when)? + battery_w(btext) + net_glyph_w() + gap();
    let y = bar.y + (bar.height - 10 * scale()) / 2;
    Some((x, y, search_glyph_w()))
}
