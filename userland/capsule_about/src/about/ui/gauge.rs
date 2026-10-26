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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::line_height;

use crate::about::theme::{ACCENT, LABEL, MUTED, TRACK_BG};

use super::card;
use super::metrics::{
    BODY_PX, CARD_PAD, TILE_H, TILE_RING_R, TILE_RING_T, TILE_RING_TOP, TILE_SUB_GAP,
};
use super::text;

pub const HEIGHT: u32 = TILE_H;

/// What one tile reads: its label, the value in the ring, the provenance
/// line under it, and the fraction `num/den` the ring fills.
pub struct Reading<'a> {
    pub label: &'a [u8],
    pub value: &'a [u8],
    pub sub: &'a [u8],
    pub num: u64,
    pub den: u64,
}

// A ring, the value it stands for, and one line of provenance under it. The
// arc and the track share a centre and a radius, so the filled sweep is always
// exactly the fraction `num/den` of the ring the eye compares it against.
pub fn tile(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, r: &Reading) {
    card::panel(fb, x, y as i32, w, TILE_H);
    let cx = x + w / 2;
    let cy = y + TILE_RING_TOP + TILE_RING_R;
    fb.ring(cx, cy, TILE_RING_R, TILE_RING_T, TRACK_BG);
    fb.ring_arc(cx, cy, TILE_RING_R, TILE_RING_T, r.num, r.den.max(1), ACCENT);
    super::gauge_value::paint(fb, cx, cy, r.value);
    let inner = card::inner(w);
    let label_y = cy + TILE_RING_R + CARD_PAD;
    label_line(fb, x, label_y, inner, r.label, LABEL);
    let step = line_height(BODY_PX).max(1) as u32 + TILE_SUB_GAP;
    label_line(fb, x, label_y + step, inner, r.sub, MUTED);
}

// Both lines under the ring are centred on the card, which means measuring the
// cut string rather than the original: a truncated label centres on what is
// actually drawn.
fn label_line(fb: &mut PaintBuffer, x: u32, y: u32, inner: u32, bytes: &[u8], argb: u32) {
    let cut = text::fit(fb, bytes, BODY_PX, inner);
    let w = text::width(fb, cut, BODY_PX);
    text::line(fb, x + CARD_PAD + inner.saturating_sub(w) / 2, y as i32, cut, argb, BODY_PX);
}
