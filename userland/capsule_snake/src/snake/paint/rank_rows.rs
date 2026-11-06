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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{SPACE, TABLE_ROW_H};
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{Col, Ink, Label, Rect, Table};

use crate::snake::state::Game;
use crate::snake::ui::metrics::RANK_ROWS;
use crate::snake::ui::rank_geom::sheet;
use crate::snake::ui::text::as_str;

use super::num::{self, Digits};

const EMPTY: &str = "No runs stored yet";

/// Shares, not pixels: the table is as wide as the window leaves it, and a
/// fixed column would cut the mode name at the first resize.
const COLS: [Col; 4] = [
    Col::new("#").share(1),
    Col::new("Score").share(3),
    Col::new("Mode").share(3),
    Col::new("Length").share(2),
];

/// The stored runs, best first, as a shared table. The rank is the row's own
/// place, so a table that lost a record to a short read still counts from one.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let at = sheet(fb.width, fb.height);
    let runs = &game.runs[..RANK_ROWS.min(game.runs.len())];
    let figures: Vec<[Digits; 3]> = runs
        .iter()
        .enumerate()
        .map(|(i, run)| [num::dec(i as u32 + 1), num::dec(run.score), num::dec(run.length as u32)])
        .collect();
    let cells: Vec<[&str; 4]> = figures
        .iter()
        .zip(runs)
        .map(|([place, score, length], run)| {
            let mode = as_str(run.mode.name());
            [as_str(place.as_bytes()), as_str(score.as_bytes()), mode, as_str(length.as_bytes())]
        })
        .collect();
    let rows: Vec<&[&str]> = cells.iter().map(|c| &c[..]).collect();
    Table::new(&COLS).rows(&rows).draw(fb, at);
    if runs.is_empty() {
        Label::new(EMPTY).ink(Ink::Muted).draw(fb, first_row(at));
    }
}

/// Where the table's first row sits under its header, less the pad a cell
/// keeps each side.
fn first_row(at: Rect) -> Rect {
    let s = scale();
    let row_h = s.px(TABLE_ROW_H);
    Rect { y: at.y + row_h + s.hair(), h: row_h, ..inset(at, s.px(u32::from(SPACE.md))) }
}
