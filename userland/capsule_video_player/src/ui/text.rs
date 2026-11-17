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

const BODY_PX: f32 = 17.0;
const TITLE_PX: f32 = 20.0;
const HEAD_PX: f32 = 24.0;
const LINE_H: u32 = 21;

/// The body type size at the display scale in force.
pub fn body_px() -> f32 {
    scale().font(BODY_PX)
}

/// The title type size at the display scale in force.
pub fn title_px() -> f32 {
    scale().font(TITLE_PX)
}

/// The heading type size at the display scale in force.
pub fn head_px() -> f32 {
    scale().font(HEAD_PX)
}

/// A line box's height at the display scale in force, in drawing pixels.
pub fn line_h() -> u32 {
    scale().px(LINE_H)
}

pub fn center_y(y: u32, h: u32) -> i32 {
    (y + h.saturating_sub(line_h()) / 2) as i32
}

const MAX_SECS: i64 = 99 * 3600 + 59 * 60 + 59;

pub fn hhmmss(ms: i64, out: &mut [u8; 8]) -> &str {
    let secs = if ms <= 0 { 0 } else { (ms / 1000).min(MAX_SECS) };
    let h = (secs / 3600) as u32;
    let m = ((secs % 3600) / 60) as u32;
    let s = (secs % 60) as u32;
    out[0] = b'0' + (h / 10) as u8;
    out[1] = b'0' + (h % 10) as u8;
    out[2] = b':';
    out[3] = b'0' + (m / 10) as u8;
    out[4] = b'0' + (m % 10) as u8;
    out[5] = b':';
    out[6] = b'0' + (s / 10) as u8;
    out[7] = b'0' + (s % 10) as u8;
    core::str::from_utf8(out).unwrap_or("00:00:00")
}
