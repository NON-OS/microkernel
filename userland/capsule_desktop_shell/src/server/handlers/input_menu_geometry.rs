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

//! Where the input menu and its rows sit, shared by the press routing and
//! the paint pass so the two never disagree. At most eight rows show; the
//! window follows the keyboard down the list.

use crate::render::layout::Rect;
use crate::render::ui_font;
use crate::state::input_menu::InputMenu;

const PANEL_W: u32 = 520;
/// The title and the note above the rows.
const HEAD_H: u32 = 92;
const ROW_H: u32 = 32;
const FOOT_H: u32 = 20;
pub(crate) const VISIBLE: usize = 8;

/// The first row shown, keeping the keyboard's row in the window.
pub(crate) fn first_row(at: usize) -> usize {
    at.saturating_sub(VISIBLE - 1)
}

pub(crate) fn panel_rect(w: u32, h: u32, rows: usize) -> Rect {
    let tall = HEAD_H + rows.min(VISIBLE) as u32 * ROW_H + FOOT_H;
    let (pw, ph) = (ui_font::px(PANEL_W).min(w), ui_font::px(tall).min(h));
    Rect { x: w.saturating_sub(pw) / 2, y: h.saturating_sub(ph) / 2, width: pw, height: ph }
}

/// The `i`th row shown (0 is the window's first).
pub(crate) fn row_rect(w: u32, h: u32, rows: usize, i: usize) -> Rect {
    let p = panel_rect(w, h, rows);
    let side = ui_font::px(20);
    let y = p.y + ui_font::px(HEAD_H + i as u32 * ROW_H);
    Rect {
        x: p.x + side,
        y,
        width: p.width.saturating_sub(2 * side),
        height: ui_font::px(ROW_H - 4),
    }
}

/// The row under the press, None for one outside every row shown.
pub(crate) fn row_at(menu: &InputMenu, w: u32, h: u32, px: u32, py: u32) -> Option<usize> {
    let (rows, first) = (menu.rows().count(), first_row(menu.at()));
    (0..rows.saturating_sub(first).min(VISIBLE)).find_map(|i| {
        let r = row_rect(w, h, rows, i);
        let hit = px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height;
        hit.then_some(first + i)
    })
}
