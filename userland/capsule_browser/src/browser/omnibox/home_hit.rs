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

use super::geometry::{search_rect, BADGE, BADGE_Y, CELL_W};

/* Horizontal centre of shortcut `i` in a row of `count`, centred in `width`. */
pub fn center_x(width: u32, count: u32, i: u32) -> u32 {
    let row = count.saturating_mul(CELL_W);
    width.saturating_sub(row) / 2 + i * CELL_W + CELL_W / 2
}

/* Which of `count` shortcut badges is under (x, y), for a page `width`
 * pixels wide: the same width the badges were painted at. */
pub fn shortcut_at(x: i32, y: i32, width: u32, count: u32) -> Option<usize> {
    if x < 0 || y < 0 {
        return None;
    }
    let (xu, yu) = (x as u32, y as u32);
    if !(BADGE_Y..BADGE_Y + BADGE).contains(&yu) {
        return None;
    }
    (0..count)
        .find(|&i| {
            let bx = center_x(width, count, i).saturating_sub(BADGE / 2);
            xu >= bx && xu < bx + BADGE
        })
        .map(|i| i as usize)
}

pub fn search_bar_hit(x: i32, y: i32, width: u32) -> bool {
    search_rect(width).contains(x, y)
}
