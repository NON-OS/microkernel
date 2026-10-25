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

//! Where Tab's candidates are shown and how they are laid out.
//!
//! They used to be printed as one line into the last command's output, again
//! on every Tab. Now they are a list beside the prompt that the next key takes
//! away: columns as wide as the longest name, filled down then across, cut to
//! a few rows with a count of the rest. Nothing goes into the scrollback.

/// The most rows the list takes; past them it says how many more there are.
pub const MAX_ROWS: usize = 8;
/// Cells between one column and the next.
pub const GUTTER: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grid {
    /// Cells per column, the gutter included.
    pub width: usize,
    pub cols: usize,
    pub rows: usize,
    /// How many names the grid holds; the rest are counted, not drawn.
    pub shown: usize,
}

/// The grid for names of these cell widths in `room` cells across.
pub fn grid(widths: &[usize], room: usize) -> Grid {
    let width = widths.iter().copied().max().unwrap_or(0) + GUTTER;
    let cols = (room / width.max(1)).max(1);
    let rows = widths.len().div_ceil(cols).min(MAX_ROWS);
    let shown = widths.len().min(rows * cols);
    Grid { width, cols, rows, shown }
}

/// Name `i`'s cell, filled down then across: (column, row).
pub fn cell(g: &Grid, i: usize) -> (usize, usize) {
    (i / g.rows.max(1), i % g.rows.max(1))
}

/// What a candidate shows: a path's last part, a trailing `/` kept, so
/// `/home/nonos/workspace/` reads `workspace/`.
pub fn label(path: &[u8]) -> &[u8] {
    let body = path.strip_suffix(b"/").unwrap_or(path);
    match body.iter().rposition(|&b| b == b'/') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

/// The list's top: under the prompt row when it fits above `floor`, or else
/// over the body just above the prompt, never above `ceiling`.
pub fn top(prompt_y: u32, row_h: u32, lines: u32, ceiling: u32, floor: u32) -> u32 {
    let h = lines * row_h;
    let below = prompt_y + row_h;
    if below + h <= floor {
        below
    } else {
        prompt_y.saturating_sub(h).max(ceiling)
    }
}
