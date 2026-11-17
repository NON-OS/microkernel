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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::clip;
use nonos_toolkit::widgets::Separator;

use crate::pm::format_caps::CAP_TABLE;

use super::super::chrome::Rect;
use super::super::matrix_geom::{self as geom, MATRIX};
use super::super::metrics::metrics;
use super::super::text;
use super::super::tint::cap_tint;

const PER_ROW: usize = 5;
const LINE_H: u32 = 21;
const TOP_PAD: u32 = 4;
const PAIR_GAP: u32 = 6;

// An abbreviated header is only honest if the full name is on screen, so the
// strip under the grid pairs each one with the name format_caps::CAP_TABLE gives it,
// five to a line. The abbreviation keeps its column colour; the spelt-out name
// is muted, because the eye is meant to come here once and then stop.
pub fn paint(fb: &mut PaintBuffer, r: &Rect) {
    let (c, s, px) = (roles(), scale(), metrics().body_px);
    let (y, hair, pad_x) = (r.y + geom::grid_h(r.h), s.hair(), s.px(geom::PAD_X));
    let (line_h, pair_gap) = (s.px(LINE_H), s.px(PAIR_GAP));
    Separator::new().draw(fb, Rect { x: r.x + hair, y, w: r.w.saturating_sub(hair * 2), h: hair });
    let col_w = r.w.saturating_sub(pad_x * 2) / PER_ROW as u32;
    for (i, (bit, short)) in MATRIX.iter().enumerate() {
        let x = r.x + pad_x + (i % PER_ROW) as u32 * col_w;
        let top = y + s.px(TOP_PAD) + (i / PER_ROW) as u32 * line_h;
        let after = text::left(fb, x, top, short, cap_tint(*bit).colour(&c), px).max(0) as u32;
        let used = after.saturating_sub(x) + pair_gap;
        let full = clip(text::valid(full_name(*bit)), col_w.saturating_sub(used), px).as_bytes();
        text::left(fb, after + pair_gap, top, full, c.muted, px);
    }
}

fn full_name(bit: u64) -> &'static [u8] {
    CAP_TABLE.iter().find(|(mask, _)| *mask == bit).map(|(_, name)| *name).unwrap_or(b"")
}
