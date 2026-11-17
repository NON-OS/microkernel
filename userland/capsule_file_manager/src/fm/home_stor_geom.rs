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

extern crate alloc;

use alloc::vec::Vec;

use super::home_cat_geom::{CatCell, CARD_GAP};
use super::home_geom::{stor_h, stor_row_y};
use super::layout::{metrics, px};

/// The two cards the row carries: the store as a whole, then the one subtree
/// with a capacity story of its own. Both browse to the prefix they describe, so
/// the row invents no destination the sidebar does not already name.
pub const STOR_PLACES: [(&str, &str); 2] = [("NØNOS Drive", "/"), ("Capsules", "/capsules/")];

// The drive card carries a longer occupancy line than the place card does, so
// the row is split by share rather than evenly.
const DRIVE_NUM: u32 = 3;
const DRIVE_DEN: u32 = 5;

/// The row's one layout pass, reusing the category cell so the painter and the
/// hit-test speak the same shape on both rows.
pub fn stor_cells(win_w: u32, win_h: u32) -> Vec<CatCell> {
    let (m, gap, h) = (metrics(), px(CARD_GAP), stor_h());
    let total = win_w.saturating_sub(m.sidebar_w + m.pad_x * 2).saturating_sub(gap);
    let drive = total * DRIVE_NUM / DRIVE_DEN;
    let y = stor_row_y(win_h);
    let x = m.sidebar_w + m.pad_x;
    alloc::vec![
        CatCell { x, y, w: drive, h, idx: 0 },
        CatCell { x: x + drive + gap, y, w: total - drive, h, idx: 1 },
    ]
}

/// Which storage card `(x, y)` is inside.
pub fn stor_at(win_w: u32, win_h: u32, x: u32, y: u32) -> Option<usize> {
    stor_cells(win_w, win_h)
        .into_iter()
        .find(|c| x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + c.h)
        .map(|c| c.idx)
}
