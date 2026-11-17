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

//! Table: every pixel of a cell's text lies in that cell's `text_rect`, but
//! for a glyph's antialiased edge, which may reach the pixel either side of
//! the run it is measured to.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Cell, Rect, Table};

use super::boxes::inside;
use super::canvas::Canvas;
use super::serial::{serial, SCALES};
use super::table_place_set::cases;

/// The points where the text of `t`'s `rows` rows is drawn, each outside
/// the `text_rect`, a pixel wider each side, of the cell `hit` names there.
fn ink(t: Table, at: Rect, rows: usize) -> Vec<String> {
    let blank = vec![&[][..]; rows];
    let draw = |t: Table| Canvas::new(at, 0).painted(|fb| t.draw(fb, at));
    let text = draw(t).diff(&draw(t.rows(&blank)));
    let place = |x: u32, y: u32| match t.hit(at, x as i32, y as i32) {
        Some(Cell::Row { row, col }) => t.cell_rect(at, row, col).map(Table::text_rect).map(wide),
        _ => None,
    };
    let stray = text.into_iter().filter(|&(x, y)| !place(x, y).is_some_and(|r| inside(r, (x, y))));
    stray
        .map(|(x, y)| format!("text at {x},{y} hits {:?}", t.hit(at, x as i32, y as i32)))
        .collect()
}

/// `r` a pixel wider each side.
fn wide(r: Rect) -> Rect {
    Rect { x: r.x.saturating_sub(1), w: r.w + 2, ..r }
}

#[test]
fn table_cell_text_keeps_to_its_text_rect() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, t, at, n) in cases() {
            fails.extend(ink(t, at, n.1).into_iter().map(|e| format!("{name} {at:?} {q}/4: {e}")));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
