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

//! The product mark at the top of the rail: a rounded accent chip carrying an
//! "N", then the wordmark. The chip's gradient is laid in three straight bands
//! that all fall inside the rounded silhouette, so the corners stay round.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::derive::mix;
use nonos_toolkit::theme::roles;

use super::metrics::{brand_side, brand_y, lh, rail_pad, rail_x, subhead};

const CHIP_R: u32 = 8;
const WORD_GAP: u32 = 10;

pub(super) fn paint_brand(fb: &mut PaintBuffer) {
    let (x, r) = (rail_x() + rail_pad(), roles());
    paint_chip(fb, x, brand_y(), r.accent.argb(), mix(r.accent.argb(), r.text.argb(), 64));
    let gw = fb.measure_ttf("N", subhead()).max(0) as u32;
    let gy = brand_y() + brand_side().saturating_sub(lh(subhead())) / 2;
    let gx = x + brand_side().saturating_sub(gw) / 2;
    let _ = fb.text_ttf(gx as i32, gy as i32, "N", r.on_accent.argb(), subhead());
    let lx = (x + brand_side() + scale().px(WORD_GAP)) as i32;
    let _ = fb.text_ttf(lx, gy as i32, "NØNOS Docs", r.text.argb(), subhead());
}

/// The chip, shading from `a` at its left edge to `b` at its right.
fn paint_chip(fb: &mut PaintBuffer, x: u32, y: u32, a: u32, b: u32) {
    let (side, r) = (brand_side(), scale().px(CHIP_R));
    let inner = side - r * 2;
    fb.fill_round(x, y, side, side, r, a);
    fb.gradient_h(x, y + r, side, inner, a, b);
    fb.gradient_h(x + r, y, inner, r, a, b);
    fb.gradient_h(x + r, y + side - r, inner, r, a, b);
}
