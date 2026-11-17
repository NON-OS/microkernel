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

//! Where the Folders page puts its parts: the rail of folders down the left,
//! the chosen folder's videos and the Open button under them.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::Rect;

use crate::catalog::folders::LABELS;

/// The rail's rows: every folder, then one per root.
pub const RAIL_ROWS: usize = LABELS.len() + 1;
/// The rail's heading, above its first row.
pub const RAIL_TOP: u32 = 28;
pub const RAIL_W: u32 = 190;

const GAP: u32 = 20;
const ROW_H: u32 = 36;
const ACTIONS_H: u32 = 52;
const BTN_W: u32 = 130;
const BTN_H: u32 = 38;

/// Row `i` of the rail: 0 is every folder, then one row per root.
pub fn rail_row(body: Rect, i: usize) -> Rect {
    let (s, row_h) = (scale(), scale().px(ROW_H));
    Rect { x: body.x, y: body.y + s.px(RAIL_TOP) + i as u32 * row_h, w: s.px(RAIL_W), h: row_h }
}

/// Where the chosen folder's videos are drawn.
pub fn folder_area(body: Rect) -> Rect {
    let (s, left) = (scale(), scale().px(RAIL_W) + scale().px(GAP));
    let (w, h) = (body.w.saturating_sub(left), body.h.saturating_sub(s.px(ACTIONS_H)));
    Rect { x: body.x + left, y: body.y, w, h }
}

pub fn open_button(body: Rect) -> Rect {
    let (a, s) = (folder_area(body), scale());
    let (bw, bh) = (s.px(BTN_W), s.px(BTN_H));
    let y = body.y + body.h.saturating_sub(bh);
    Rect { x: (a.x + a.w).saturating_sub(bw), y, w: bw, h: bh }
}
