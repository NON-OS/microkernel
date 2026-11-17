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

//! Menu: making a row a danger row moves nothing. Every menu keeps its size
//! and its hits, at every scale and in every box.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Item, Menu, Rect};

use super::boxes::boxes;
use super::menu::MENUS;
use super::serial::{serial, SCALES};

/// `items` with item `i` made a danger row.
pub(crate) fn with_danger(items: &[Item<'static>], i: usize) -> Vec<Item<'static>> {
    let mut all = items.to_vec();
    all[i] = all[i].danger();
    all
}

/// Every point of `at` and 4 px round it.
fn around(at: Rect) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = (at.x as i32 - 4, at.y as i32 - 4);
    let ys = y0..y0 + at.h as i32 + 8;
    ys.flat_map(move |y| (x0..x0 + at.w as i32 + 8).map(move |x| (x, y)))
}

#[test]
fn menu_danger_row_keeps_every_box_and_hit() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for (name, items) in MENUS {
            for i in 0..items.len() {
                let (m, danger) = (Menu::new(items), with_danger(items, i));
                let d = Menu::new(&danger);
                assert_eq!(m.size(), d.size(), "{name} row {i} at {q}/4");
                for at in boxes(m.size()) {
                    let moved = around(at).find(|&(x, y)| m.hit(at, x, y) != d.hit(at, x, y));
                    assert_eq!(moved, None, "{name} row {i} in {at:?} at {q}/4");
                }
            }
        }
    }
    set_scale(SCALES[0]);
}
