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
use super::super::metrics::{cap_h, metrics, px};

const QROW_H: i32 = 46;

/// One "Up next" row's height at the display scale.
pub fn qrow_h() -> i32 {
    px(QROW_H)
}

pub fn body(r: Rect) -> Rect {
    let s5 = metrics().s5;
    r.pad(s5, s5)
}

pub fn reserved() -> i32 {
    let m = metrics();
    let (heads, groove) = (cap_h(m.section) * 4 + cap_h(m.label) * 2, px(6) * 2);
    m.s6 * 2 + m.s5 * 2 + m.s4 + heads + groove + px(44) + px(32) + qrow_h()
}

pub fn art_rect(r: Rect) -> Rect {
    let b = body(r);
    let h = (b.h - reserved()).clamp(px(72), b.w);
    Rect::new(b.x, b.y, b.w, h)
}

pub fn seek_rect(r: Rect) -> Rect {
    let b = body(r);
    let m = metrics();
    Rect::new(b.x, art_rect(r).bottom() + m.s6 + cap_h(m.section) * 4, b.w, px(6))
}

pub fn transport_row(r: Rect) -> Rect {
    let b = body(r);
    let m = metrics();
    Rect::new(b.x, seek_rect(r).bottom() + m.s5 + cap_h(m.label) * 2, b.w, px(44))
}

pub fn vol_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x + px(26), transport_row(r).bottom() + metrics().s5, b.w - px(26), px(6))
}

pub fn play_rect(r: Rect) -> Rect {
    let tr = transport_row(r);
    Rect::new(tr.cx() - px(22), tr.y, px(44), px(44))
}

// The four small buttons beside play, in the order prev, next, shuffle, repeat.
pub fn small_rects(r: Rect) -> [Rect; 4] {
    let b = body(r);
    let tr = transport_row(r);
    let (big, s3, y, d) = (play_rect(r), metrics().s3, tr.y + px(7), px(30));
    [
        Rect::new(big.x - px(44) - s3, y, d, d),
        Rect::new(big.right() + s3, y, d, d),
        Rect::new(b.x, y, d, d),
        Rect::new(b.right() - d, y, d, d),
    ]
}

pub fn speaker_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, vol_rect(r).cy() - px(11), px(22), px(22))
}

// The queue's heading. It names the one list the rail shows; the player has
// no lyrics or recommendations to switch to, so it is a label, not tabs.
pub fn head_rect(r: Rect) -> Rect {
    let b = body(r);
    Rect::new(b.x, vol_rect(r).bottom() + metrics().s6, b.w, px(32))
}

pub fn queue_row(r: Rect, i: usize) -> Rect {
    let b = body(r);
    let h = qrow_h();
    Rect::new(b.x, head_rect(r).bottom() + metrics().s3 + i as i32 * h, b.w, h)
}

pub fn queue_visible(r: Rect) -> usize {
    let top = queue_row(r, 0).y;
    ((r.bottom() - metrics().s5 - top).max(0) / qrow_h()) as usize
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
