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

use super::cluster::cluster_x;
use super::glyph_w::batt_glyph_w;
use super::metrics::gap;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::{px, STATUS_PX};
use crate::state::Context;

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
