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

//! Where the right rail puts its parts, shared by its painter and its
//! hit-test, and which track a click on an "Up next" row means.

use super::super::geometry::Rect;
use super::super::metrics::{cap_h, LABEL, S3, S4, S5, S6, SECTION};

pub const QROW_H: i32 = 46;

pub fn body(r: Rect) -> Rect {
    r.pad(S5, S5)
}

pub fn reserved() -> i32 {
    S6 + cap_h(SECTION) * 4 + 6 + S5 + cap_h(LABEL) * 2 + 44 + S5 + 6 + S6 + 32 + S4 + QROW_H
}

pub fn art_rect(r: Rect) -> Rect {
    let b = body(r);
    let h = (b.h - reserved()).clamp(72, b.w);
    Rect::new(b.x, b.y, b.w, h)
}

pub fn seek_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, art_rect(r).bottom() + S6 + cap_h(SECTION) * 4, b.w, 6)
}

pub fn transport_row(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, seek_rect(r).bottom() + S5 + cap_h(LABEL) * 2, b.w, 44)
}

pub fn vol_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x + 26, transport_row(r).bottom() + S5, b.w - 26, 6)
}

// The queue's heading. It names the one list the rail shows; the player has
// no lyrics or recommendations to switch to, so it is a label, not tabs.
pub fn head_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, vol_rect(r).bottom() + S6, b.w, 32)
}

pub fn queue_row(r: Rect, i: usize) -> Rect {
    let b = body(r);
    Rect::new(b.x, head_rect(r).bottom() + S3 + i as i32 * QROW_H, b.w, QROW_H)
}

pub fn queue_visible(r: Rect) -> usize {
    let top = queue_row(r, 0).y;
    ((r.bottom() - S5 - top).max(0) / QROW_H) as usize
}

/// The row of the queue under the point, counted from the top.
pub fn queue_at(r: Rect, x: i32, y: i32) -> Option<usize> {
    (0..queue_visible(r)).find(|&i| queue_row(r, i).contains(x, y))
}

/// The track a click on the queue means: the one the row shows, the queue's
/// `items()[row]`, as the painter draws it. The row number alone is that
/// track only while the queue is in library order, so under shuffle a click
/// played another.
pub fn queue_track_at(r: Rect, items: &[usize], x: i32, y: i32) -> Option<usize> {
    queue_at(r, x, y).and_then(|row| items.get(row).copied())
}
