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

//! Where the left rail's entries and the top bar's search field sit, which
//! their painters and the target table both read.

use super::super::geometry::Rect;
use super::super::metrics::{metrics, px};

const ROW_H: i32 = 42;
const BRAND_H: i32 = 68;
const FIELD_H: i32 = 34;
const CHIP_W: i32 = 112;

pub fn nav_row(r: Rect, i: usize) -> Rect {
    let (m, h) = (metrics(), px(ROW_H));
    Rect::new(r.x + m.s3, r.y + px(BRAND_H) + i as i32 * (h + m.s1), r.w - m.s3 * 2, h)
}

pub fn settings_row(r: Rect) -> Rect {
    let (m, h) = (metrics(), px(ROW_H));
    Rect::new(r.x + m.s3, r.bottom() - m.s5 - h, r.w - m.s3 * 2, h)
}

pub fn clear(r: Rect) -> Rect {
    let f = field(r);
    Rect::new(f.right() - px(26), f.y, px(22), f.h)
}

pub fn field(r: Rect) -> Rect {
    let (m, h) = (metrics(), px(FIELD_H));
    let w = (r.w - m.s5 * 2 - px(CHIP_W) - m.s6 - m.s5).min(px(420));
    Rect::new(r.x + m.s5, r.cy() - h / 2, w, h)
}

/// The folder chip at the top bar's right end.
pub fn chip(r: Rect) -> Rect {
    let (w, h) = (px(CHIP_W), px(FIELD_H));
    Rect::new(r.right() - metrics().s6 - w, r.cy() - h / 2, w, h)
}
