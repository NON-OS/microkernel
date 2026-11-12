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

use nonos_toolkit::design::scale::scale;

/// `v` CSS px in device px at `q` quarters, rounded as `Scale::px` rounds.
pub fn to_device(v: i32, q: u32) -> i32 {
    (v as i64 * q as i64 + 2).div_euclid(4).clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// `v` device px in CSS px at `q` quarters.
pub fn to_css(v: i32, q: u32) -> i32 {
    (v as i64 * 4).div_euclid(q.max(1) as i64) as i32
}

/// `v` CSS px in device px at the display's scale.
pub fn device_px(v: i32) -> i32 {
    to_device(v, scale().quarters())
}

/// `v` device px in CSS px at the display's scale.
pub fn css_px(v: i32) -> i32 {
    to_css(v, scale().quarters())
}
