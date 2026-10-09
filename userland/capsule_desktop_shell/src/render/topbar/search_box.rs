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

//! Where the magnifier sits. The painter and the hit test both read the box
//! from here, so a click always lands on the glyph that was drawn.

use crate::render::layout::menubar_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::{px, scale, STATUS_PX};
use crate::state::Context;

use super::metrics::{batt_glyph_w, gap, net_glyph_w, right_margin, search_glyph_w};

/// Whether the battery label is a reading ("57%"), the only case that gets
/// the gauge glyph. "No battery" and "Battery status unavailable" are words
/// only: an empty gauge beside them would read as 0%.
pub(super) fn has_gauge(btext: &[u8]) -> bool {
    btext.last() == Some(&b'%')
}

/// The battery's glyph (for a reading), its label and the gap after them, or
/// nothing when the label was dropped for room (`btext` empty).
pub(super) fn battery_w(btext: &[u8]) -> u32 {
    if btext.is_empty() {
        return 0;
    }
    let glyph = if has_gauge(btext) { batt_glyph_w() + px(6) } else { 0 };
    glyph + measure_aa_bytes(btext, STATUS_PX) + gap()
}

/// The battery label to draw: the full one when the cluster fits with it,
/// else none, so a narrow screen keeps its clock and network glyph rather
/// than losing the whole cluster to a long sentence.
pub(super) fn fitted<'a>(ctx: &Context, btext: &'a [u8], when: &[u8]) -> &'a [u8] {
    if cluster_x(ctx, btext, when).is_some() {
        btext
    } else {
        &btext[..0]
    }
}

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
