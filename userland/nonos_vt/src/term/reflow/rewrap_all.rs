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

//! Every logical line of the gathered text, broken at the new width.

use alloc::vec::Vec;

use super::gather::Gathered;
use super::placed::Placed;
use super::rewrap::rewrap;
use crate::cell::Cell;
use crate::line::Line;

/// The re-wrapped lines, where the cursor went, and the line the old
/// screen's first row became.
pub(super) fn rewrap_all(g: &Gathered, cols: usize, old_cols: usize) -> (Vec<Line>, Placed, usize) {
    let phys = &g.phys;
    let mut out: Vec<Line> = Vec::new();
    let mut placed: Option<Placed> = None;
    let mut top_out = 0;
    let mut i = 0;
    while i < phys.len() {
        let start = i;
        while phys[i].wrapped && i + 1 < phys.len() {
            i += 1;
        }
        let end = i;
        i += 1;
        let mut cells: Vec<(usize, Cell)> = Vec::new();
        let mut cursor = None;
        for (li, line) in phys.iter().enumerate().take(end + 1).skip(start) {
            if li == g.cur_phys {
                cursor = Some(cells.len() + g.cur_col);
            }
            let take = if li == end { line.content_len() } else { line.len() };
            cells.extend(line.cells[..take].iter().map(|&c| (li, c)));
        }
        if let Some(o) = cursor {
            cells.resize(cells.len().max(o), (end, Cell::BLANK));
        }
        if (start..=end).contains(&g.old_top) {
            top_out = out.len() + (g.old_top - start) * old_cols / cols;
        }
        rewrap(phys, &cells, cols, cursor, &mut out, &mut placed);
    }
    let last = out.len().saturating_sub(1);
    (out, placed.unwrap_or(Placed { line: last, col: 0, pending: false }), top_out)
}
