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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::text::{centre, centre_top, left, truncate, width};
use nonos_toolkit::widgets::Rect;

use super::card;
use super::metrics::metrics;
use super::mono::valid;

/// The tile's height, the ring's radius, stroke and top inset, and the gap
/// under the label line, in logical pixels.
const TILE_H: u32 = 168;
const TILE_RING_R: u32 = 36;
const TILE_RING_T: u32 = 7;
const TILE_RING_TOP: u32 = 16;
const TILE_SUB_GAP: u32 = 4;

/// A tile's height at the display scale.
pub fn height() -> u32 {
    scale().px(TILE_H)
}

// A ring, the value it stands for, and one line of provenance under it. The
// arc and the track share a centre and a radius, so the filled sweep is always
// exactly the fraction `num/den` of the ring the eye compares it against.
pub fn tile(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    label: &[u8],
    value: &[u8],
    sub: &[u8],
    num: u64,
    den: u64,
) {
    let (r, m, s) = (roles(), metrics(), scale());
    let (ring_r, ring_t, value_px) = (s.px(TILE_RING_R), s.px(TILE_RING_T), m.value_px);
    card::panel(fb, x, y as i32, w, height());
    let cx = x + w / 2;
    let cy = y + s.px(TILE_RING_TOP) + ring_r;
    fb.ring(cx, cy, ring_r, ring_t, r.border_strong.argb());
    fb.ring_arc(cx, cy, ring_r, ring_t, num, den.max(1), r.accent.argb());
    let value_top = centre_top(cy - ring_r, ring_r * 2, value_px);
    let value_w = width(valid(value), value_px);
    left(fb, cx.saturating_sub(value_w / 2), value_top, valid(value), r.text, value_px);
    let inner = card::inner(w);
    let label_y = cy + ring_r + m.card_pad;
    label_line(fb, x, label_y, inner, label, r.text);
    let step = line_height(m.body_px).max(1) as u32 + s.px(TILE_SUB_GAP);
    label_line(fb, x, label_y + step, inner, sub, r.muted);
}

// Both lines under the ring are centred on the card, which means measuring the
// cut string rather than the original: a truncated label centres on what is
// actually drawn.
fn label_line(fb: &mut PaintBuffer, x: u32, y: u32, inner: u32, bytes: &[u8], ink: Opaque) {
    let m = metrics();
    let cut = truncate(valid(bytes), inner, m.body_px);
    let area = Rect { x: x + m.card_pad, y, w: inner, h: 0 };
    centre(fb, area, y as i32, &cut, ink, m.body_px);
}
