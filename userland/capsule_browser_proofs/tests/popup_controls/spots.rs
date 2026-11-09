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

//! Where the list is, for the menu proofs: the select it opens by, the
//! middle of a row it shows, and the row a click chooses about the list.

use capsule_browser_proofs::browser::event::select_list::SelectList;
use capsule_browser_proofs::browser::ui::select_menu::row_at;
use nonos_toolkit::design::size::{MENU_ROW_H, SPACE};
use nonos_toolkit::widgets::Rect;

use super::kit::px;

pub fn select() -> Rect {
    Rect { x: px(100), y: px(80), w: px(120), h: px(24) }
}

/// The middle of the `n`th row shown on the list drawn at `at`.
pub fn row_mid(at: Rect, n: u32) -> (i32, i32) {
    let y = at.y + px(u32::from(SPACE.xs)) + n * px(MENU_ROW_H) + px(MENU_ROW_H) / 2;
    ((at.x + at.w / 2) as i32, y as i32)
}

/// Every third point of `at` with the row a click there chooses.
pub fn spots(list: &SelectList, at: Rect) -> Vec<((i32, i32), Option<usize>)> {
    let (xs, ys) = ((at.x..at.x + at.w).step_by(3), (at.y..at.y + at.h).step_by(3));
    let pts: Vec<(i32, i32)> = ys.flat_map(|y| xs.clone().map(move |x| (x as i32, y as i32))).collect();
    pts.into_iter().map(|(x, y)| ((x, y), row_at(list, at, x, y))).collect()
}
