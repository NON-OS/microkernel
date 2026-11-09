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
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Rect as Spot, Tooltip};

use super::rect::Rect;

/// Rows at the bottom of the page that hold the hovered-link bubble: the
/// tooltip it is drawn as and the gap under it.
pub fn bubble_h() -> u32 {
    Tooltip::new("").size().1.saturating_add(scale().px(u32::from(SPACE.xs)))
}

/// The bubble showing `text`, at most `room` drawing pixels wide, and where
/// it sits at the bottom left of a page `w` by `h`, inside `bubble_h`.
pub fn bubble_at(text: &str, room: u32, w: u32, h: u32) -> (Tooltip<'_>, Spot) {
    let s = scale();
    let gap = s.px(u32::from(SPACE.xs));
    let room = room.min(w).saturating_sub(gap * 2);
    let tip = Tooltip::new(text).max_width(room.saturating_mul(4) / s.quarters().max(1));
    let bounds = Spot { x: gap, y: 0, w: w.saturating_sub(gap * 2), h };
    (tip, tip.place(Spot { x: gap, y: h, w: 0, h: 0 }, bounds))
}

/* The band at the bottom of the page area `page` that holds the
 * hovered-link bubble. */
pub fn bubble_band(page: Rect) -> Rect {
    let bottom = page.y.saturating_add(page.h);
    let y = bottom.saturating_sub(bubble_h()).max(page.y);
    Rect { x: page.x, y, w: page.w, h: bottom - y }
}

/// What a bubble repaint draws, for its commit to cover: viewport rows
/// `band` (from the page painter's band_rows) of the page area `page`, or
/// the whole page when the band could not be painted alone (None).
pub fn band_commit(band: Option<(i32, i32)>, page: Rect) -> Rect {
    match band {
        Some((a, b)) => {
            let (a, b) = (a.max(0) as u32, b.max(0) as u32);
            Rect { x: page.x, y: page.y.saturating_add(a), w: page.w, h: b.saturating_sub(a) }
        }
        None => page,
    }
}
