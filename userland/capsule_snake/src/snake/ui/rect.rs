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

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use super::metrics::metrics;

/// The rect at `x`, `y`, `w` wide and `h` tall, in real pixels of the live
/// surface.
pub const fn at(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, w, h }
}

pub fn index_at<F: Fn(usize) -> Rect>(count: usize, x: i32, y: i32, rect: F) -> Option<usize> {
    (0..count).find(|i| contains(rect(*i), x, y))
}

// The whole drawable area, inset by the window pad. `app_skeleton` already hands
// `paint` a sub-buffer of the content rect and rebases event coords onto the same
// origin, so the titlebar is not ours to subtract. Every screen starts here, so
// growing the window moves all of them together.
pub fn content(w: u32, h: u32) -> Rect {
    let pad = metrics().pad;
    at(pad, pad, w.saturating_sub(pad * 2), h.saturating_sub(pad * 2))
}

pub fn row(r: Rect, index: usize, h: u32, gap: u32) -> Rect {
    at(r.x, r.y + index as u32 * (h + gap), r.w, h)
}

pub fn column(r: Rect, index: usize, count: usize, gap: u32) -> Rect {
    let n = count.max(1) as u32;
    let w = r.w.saturating_sub(gap * (n - 1)) / n;
    at(r.x + index as u32 * (w + gap), r.y, w, r.h)
}
