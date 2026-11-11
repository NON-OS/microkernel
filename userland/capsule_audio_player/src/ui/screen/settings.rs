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

//! Settings geometry: two columns. Playback controls on the left, the facts
//! the capsule can answer for itself on the right, with the library counters
//! beneath them. The painter and the hit-test share every rect here.

use super::super::geometry::Rect;
use super::super::metrics::{line_h, metrics, px};

const ROW_H: i32 = 52;
const ROWS: i32 = 4;
const FACT_H: i32 = 36;
const METER_H: i32 = 72;
pub const TOGGLES: [&str; 3] = ["Shuffle", "Repeat", "Mute output"];
const TILE_H: i32 = 96;

/// A fact row's tallest height at the display scale.
pub fn fact_h() -> i32 {
    px(FACT_H)
}

fn head_h() -> i32 {
    let m = metrics();
    m.s4 + line_h(m.section) + m.s3
}

fn body(r: Rect) -> Rect {
    let m = metrics();
    let top = r.y + line_h(m.page) + line_h(m.body) + m.s3 + m.s4;
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}

fn column(r: Rect, i: i32) -> Rect {
    let b = body(r);
    let s6 = metrics().s6;
    let w = (b.w - s6) / 2;
    Rect::new(b.x + i * (w + s6), b.y, w, b.h)
}

pub fn panel_rect(r: Rect) -> Rect {
    let c = column(r, 0);
    Rect::new(c.x, c.y, c.w, (c.h - metrics().s6 - px(METER_H)).max(0))
}

pub fn facts_rect(r: Rect) -> Rect {
    let c = column(r, 1);
    Rect::new(c.x, c.y, c.w, (c.h - metrics().s6 - px(TILE_H)).max(0))
}

pub fn tiles_rect(r: Rect) -> Rect {
    let f = facts_rect(r);
    Rect::new(f.x, f.bottom() + metrics().s6, f.w, px(TILE_H))
}

/// The facts card's inside, below its head.
pub fn facts_body(r: Rect) -> Rect {
    let m = metrics();
    let inner = facts_rect(r).pad(m.s4 + m.s3, m.s4);
    let top = inner.y + line_h(m.section) + m.s3;
    Rect::new(inner.x, top, inner.w, inner.bottom() - top)
}

pub fn meter_rect(r: Rect) -> Rect {
    let p = panel_rect(r);
    Rect::new(p.x, p.bottom() + metrics().s6, p.w, px(METER_H))
}

pub fn toggle_row(r: Rect, i: usize) -> Rect {
    let (p, m) = (panel_rect(r), metrics());
    let pad = m.s5 + m.s3;
    let h = ((p.h - head_h() - m.s4) / ROWS).min(px(ROW_H)).max(1);
    Rect::new(p.x + pad, p.y + head_h() + i as i32 * h, p.w - pad * 2, h)
}

pub fn volume_rect(r: Rect) -> Rect {
    let row = toggle_row(r, 3);
    let w = (row.w / 2).min(px(220));
    Rect::new(row.right() - w, row.cy() - px(3), w, px(6))
}
