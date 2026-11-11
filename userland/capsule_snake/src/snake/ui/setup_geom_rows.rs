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

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pill, Rect};

use crate::snake::state::{difficulty, mode};

use super::metrics::metrics;
use super::rect::{self, at};
use super::setup_geom::band;
use super::text::as_str;

pub const CHIP_ROWS: usize = 2;
pub const CHIPS: usize = 4;
pub const TOGGLES: usize = 3;

pub const HEADS: [&[u8]; 3] = [b"Mode", b"Difficulty", b"Rules"];
pub const TOGGLE_LABELS: [&[u8]; TOGGLES] = [b"Obstacles", b"Wrap edges", b"Power-ups"];

pub fn chip_label(row: usize, index: usize) -> &'static [u8] {
    let index = index.min(CHIPS - 1);
    if row == 0 {
        mode::ALL[index].name()
    } else {
        difficulty::ALL[index].name()
    }
}

// A chip is as wide as its own label plus the pad, so the row cannot be laid
// out by glyph count. Painter and hit test both come through here.
pub fn chip_w(row: usize, index: usize) -> u32 {
    pill(row, index).size().0
}

/// The shared pill a chip is drawn as, before its tone and state.
pub fn pill(row: usize, index: usize) -> Pill<'static> {
    Pill::new(as_str(chip_label(row, index)))
}

/// The pill's own body, centred down the strip, so a click lands on what is drawn.
pub fn chip(w: u32, h: u32, row: usize, index: usize) -> Rect {
    let (strip, m) = (band(w, h, row.min(CHIP_ROWS - 1)), metrics());
    let prior: u32 = (0..index.min(CHIPS)).map(|i| chip_w(row, i) + m.chip_gap).sum();
    let tall = pill(row, index).size().1.min(m.chip_h);
    at(strip.x + prior, strip.y + (m.chip_h - tall) / 2, chip_w(row, index), tall)
}

pub fn chip_at(w: u32, h: u32, x: i32, y: i32) -> Option<(usize, usize)> {
    (0..CHIP_ROWS)
        .flat_map(|row| (0..CHIPS).map(move |i| (row, i)))
        .find(|(row, i)| contains(chip(w, h, *row, *i), x, y))
}

pub fn toggle_row(w: u32, h: u32, index: usize) -> Rect {
    rect::row(band(w, h, 2), index.min(TOGGLES - 1), metrics().row_h, 0)
}

// The whole row is the target, not just the switch: a 42 px switch inside a
// 546 px row is a click the user is entitled to miss.
pub fn toggle_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(TOGGLES, x, y, |i| toggle_row(w, h, i))
}
