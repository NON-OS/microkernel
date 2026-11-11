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

//! Table: every point `hit` names a header or cell at lies in that header's
//! `head_rect` or that cell's `cell_rect`, so a rect holds all of its cell.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Cell, Rect, Table};

use super::boxes::inside;
use super::serial::{serial, SCALES};
use super::table_place_set::cases;

/// The points, every other one each way, that `t` names a header or cell at
/// outside its rect in `at`.
fn strays(t: Table, at: Rect) -> Vec<String> {
    let mut bad = Vec::new();
    for y in (at.y..at.y + at.h).step_by(2) {
        for x in (at.x..at.x + at.w).step_by(2) {
            let rect = match t.hit(at, x as i32, y as i32) {
                Some(Cell::Row { row, col }) => t.cell_rect(at, row, col),
                Some(Cell::Header(col)) => t.head_rect(at, col),
                None => continue,
            };
            if !rect.is_some_and(|r| inside(r, (x, y))) {
                bad.push(format!(
                    "{x},{y} hits {:?} outside {rect:?}",
                    t.hit(at, x as i32, y as i32)
                ));
            }
        }
    }
    bad
}

#[test]
fn table_hit_lands_inside_the_rect_of_what_it_names() {
    let _g = serial();
    let mut fails = Vec::new();
    for q in SCALES {
        set_scale(q);
        for (name, t, at, _) in cases() {
            fails.extend(strays(t, at).into_iter().map(|e| format!("{name} {at:?} {q}/4: {e}")));
        }
    }
    set_scale(SCALES[0]);
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}
