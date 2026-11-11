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

//! Table: the header rects tile the table's width along its top edge, each
//! over its column's cells, and each cell's `text_rect` lies inside it.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Rect, Table};

use super::boxes::within;
use super::serial::{serial, SCALES};
use super::table_place_set::{cases, fits};

/// What is wrong with the header rects of `t`, with `cols` columns over
/// `rows` rows from `first`, in `at`.
fn heads(t: Table, at: Rect, (cols, rows, first): (usize, usize, usize)) -> Vec<String> {
    let mut bad = Vec::new();
    let mut x = at.x;
    for col in 0..cols {
        let Some(h) = t.head_rect(at, col).filter(|h| within(*h, at) && h.x == x && h.y == at.y)
        else {
            bad.push(format!("head {col}: {:?} not at {x},{}", t.head_rect(at, col), at.y));
            continue;
        };
        x += h.w;
        for row in first..first + fits(t, at, rows) {
            let c =
                t.cell_rect(at, row, col).filter(|c| c.x == h.x && c.w == h.w && c.y >= h.y + h.h);
            let Some(c) = c else {
                bad.push(format!("cell {row},{col} not under head {h:?}"));
                continue;
            };
            if !within(Table::text_rect(c), c) {
                bad.push(format!("text of {row},{col} {:?} outside {c:?}", Table::text_rect(c)));
            }
        }
    }
    if x != at.x + at.w || t.head_rect(at, cols).is_some() {
        bad.push(format!("heads end at {x}, past the last {:?}", t.head_rect(at, cols)));
    }
    bad
}

#[test]
fn table_head_rects_tile_the_top_over_their_cells() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, t, at, n) in cases() {
            fails.extend(heads(t, at, n).into_iter().map(|e| format!("{name} {at:?} {q}/4: {e}")));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
