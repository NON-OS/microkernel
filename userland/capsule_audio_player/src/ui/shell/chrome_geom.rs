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
use super::super::metrics::{S1, S3, S5, S6};

pub const ROW_H: i32 = 42;
pub const BRAND_H: i32 = 68;
pub const FIELD_H: i32 = 34;
pub const CHIP_W: i32 = 112;

pub fn nav_row(r: Rect, i: usize) -> Rect {
    Rect::new(r.x + S3, r.y + BRAND_H + i as i32 * (ROW_H + S1), r.w - S3 * 2, ROW_H)
}

pub fn settings_row(r: Rect) -> Rect {
    Rect::new(r.x + S3, r.bottom() - S5 - ROW_H, r.w - S3 * 2, ROW_H)
}

pub fn clear(r: Rect) -> Rect {
    let f = field(r);
    Rect::new(f.right() - 26, f.y, 22, f.h)
}

pub fn field(r: Rect) -> Rect {
    Rect::new(r.x + S5, r.cy() - FIELD_H / 2, (r.w - S5 * 2 - CHIP_W - S6 - S5).min(420), FIELD_H)
}
