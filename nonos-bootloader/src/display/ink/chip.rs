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

//! A status chip: a pill with a coloured dot, a name, and a state word.

use super::palette::{BORDER, SURFACE, TEXT_3};
use super::parts::dot;
use super::card::card;
use super::style::{unit, Style};
use super::text::{draw, metrics, width};

pub fn chip_width(name: &[u8], state: &[u8]) -> u32 {
    let u = unit();
    let d = (metrics(Style::Caption).px / 2).max(6) & !1;
    5 * u + d + width(name, Style::Caption) + width(state, Style::Caption)
}

pub fn chip_height() -> u32 {
    metrics(Style::Caption).line + 2 * unit()
}

/// Draw a chip with its top left at (x, y); returns its width.
pub fn chip(x: u32, y: u32, name: &[u8], state: &[u8], color: u32) -> u32 {
    let u = unit();
    let (w, h) = (chip_width(name, state), chip_height());
    card(x, y, w, h, h / 2, SURFACE, BORDER);
    let d = dot(x + 2 * u, y + u, Style::Caption, color);
    let nx = x + 2 * u + d + u;
    draw(nx, y + u, name, Style::Caption, TEXT_3);
    draw(nx + width(name, Style::Caption) + u, y + u, state, Style::Caption, color);
    w
}

/// Draw chips centred in the column at `x`, `w` wide, wrapping to new rows.
/// Returns the y below the last row.
pub fn chip_rows(x: u32, w: u32, y: u32, chips: &[(&[u8], &[u8], u32)]) -> u32 {
    let (u, h) = (unit(), chip_height());
    let (mut start, mut y) = (0, y);
    while start < chips.len() {
        let (mut end, mut row_w) = (start + 1, chip_width(chips[start].0, chips[start].1));
        while end < chips.len() && row_w + u + chip_width(chips[end].0, chips[end].1) <= w {
            row_w += u + chip_width(chips[end].0, chips[end].1);
            end += 1;
        }
        let mut cx = x + w.saturating_sub(row_w) / 2;
        for &(n, s, c) in &chips[start..end] {
            cx += chip(cx, y, n, s, c) + u;
        }
        (start, y) = (end, y + h + u);
    }
    y
}
