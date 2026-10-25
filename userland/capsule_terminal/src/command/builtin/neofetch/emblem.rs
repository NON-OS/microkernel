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

//! The emblem's colours, and its rows of half blocks (`emblem_shape`).
use alloc::vec::Vec;

use super::compose::Row;
use super::emblem_cell::{block, cell};
use super::emblem_light::pixel;
use super::emblem_shape::{cover, EMBLEM_H, EMBLEM_W};

/// The emblem's rows over the theme's background `bg` (0xRRGGBB); a file gets
/// the face alone in plain blocks.
pub fn emblem(bg: u32) -> Vec<Row> {
    let bg = [bg >> 16 & 0xFF, bg >> 8 & 0xFF, bg & 0xFF];
    let mut rows = Vec::new();
    for r in 0..(EMBLEM_H / 2) as i32 {
        let (mut plain, mut styled) = (Vec::new(), Vec::new());
        for x in 0..EMBLEM_W as i32 {
            let (up, down) = (pixel(x, 2 * r, bg), pixel(x, 2 * r + 1, bg));
            let (a, b) = (cover(x, 2 * r, 0.0) >= 8, cover(x, 2 * r + 1, 0.0) >= 8);
            plain.extend_from_slice(block(a, b));
            cell(&mut styled, up, down);
        }
        rows.push((plain, styled));
    }
    rows
}
