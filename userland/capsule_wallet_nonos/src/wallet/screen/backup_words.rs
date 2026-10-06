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

//! The recovery words as a numbered grid, read left to right, so the order
//! a reader copies them in cannot be mistaken.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::Role;
use crate::wallet::etna::text::{draw, draw_in, line};
use crate::wallet::etna::tokens::{CORNER, OUTLINE, TEXT_3, TIGHT};
use crate::wallet::state::State;

const COLS: u32 = 3;
const PAD: u32 = 14;

/// Draw the words at `x, y` across `w`; returns the grid's height.
pub fn grid(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) -> u32 {
    let count = state.backup_count as usize;
    let cell_w = (w - (COLS - 1) * TIGHT) / COLS;
    let cell_h = line(Role::RowValue) as u32 + 2 * PAD;
    let rows = (count as u32).div_ceil(COLS);
    for (i, &index) in state.backup_words.iter().take(count).enumerate() {
        let (col, row) = (i as u32 % COLS, i as u32 / COLS);
        let cx = x + col * (cell_w + TIGHT);
        let cy = y + row * (cell_h + TIGHT);
        fb.stroke_round(cx, cy, cell_w, cell_h, CORNER, 1, OUTLINE);
        let n = alloc::format!("{:>2}", i + 1);
        let top = (cy + PAD) as i32;
        let after = draw_in(fb, (cx + PAD) as i32, top, Role::Fact, &n, TEXT_3);
        let word = nonos_hd::ENGLISH_WORDLIST.get(index as usize).copied().unwrap_or("");
        draw(fb, after + TIGHT as i32, top, Role::RowValue, word);
    }
    rows * cell_h + rows.saturating_sub(1) * TIGHT
}
