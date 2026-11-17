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

//! The shell's menus as the shared `Menu`, lit by a `Pointer`: a row lights over
//! the box its click names, a press holds only the row it began on, and an off
//! row dims without moving the rest, at each scale the scale test sets.

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Item, Menu, Pointer, Rect};

use crate::shell_layer::layer;
use crate::shell_menu_view::{draw, lit};

const LAYER: Rect = Rect { x: 0, y: 0, w: 640, h: 320 };
const LIVE: [Item<'static>; 3] = [Item::new("Open"), Item::new("Browser"), Item::new("Delete")];
const DIM: [Item<'static>; 3] = [LIVE[0], Item::new("Browser").disabled(), LIVE[2]];

fn inside(r: Rect) -> impl Iterator<Item = (i32, i32)> {
    (r.y..r.y + r.h).flat_map(move |y| (r.x..r.x + r.w).map(move |x| (x as i32, y as i32)))
}

fn painted(menu: Menu, at: Rect, pointer: &Pointer) -> Vec<u32> {
    let (w, h) = (LAYER.w, LAYER.h);
    let mut words = vec![roles().bg.argb(); (w * h) as usize];
    draw(&mut unsafe { layer(words.as_mut_ptr() as u64, w * 4, w, h) }, menu, at, pointer);
    words
}

fn row_at(m: &Menu, at: Rect, row: usize) -> (i32, i32) {
    let on: Vec<_> = inside(at).filter(|&(x, y)| m.hit(at, x, y) == Some(row)).collect();
    on[on.len() / 2]
}
fn drawn_in(m: &Menu, at: Rect, p: &Pointer, row: usize) -> bool {
    let (plain, lit) = (painted(*m, at, &Pointer::new()), painted(*m, at, p));
    let pairs = plain.iter().zip(&lit);
    let mut moved = inside(LAYER).zip(pairs).filter(|(_, (a, b))| a != b).peekable();
    moved.peek().is_some() && moved.all(|((x, y), _)| m.hit(at, x, y) == Some(row))
}

pub(crate) fn menus_light_where_they_are_clicked(tag: &str) {
    let (live, dim) = (Menu::new(&LIVE), Menu::new(&DIM));
    let at = live.place(Rect { x: 40, y: 30, w: 0, h: 0 }, LAYER);
    for (row, other) in [(0, 2), (2, 0)] {
        let ((x, y), (ox, oy), mut p) =
            (row_at(&live, at, row), row_at(&live, at, other), Pointer::new());
        p.moved(x, y);
        assert_eq!(lit(&live, at, &p), (Some(row), None), "{tag}: hover {row}");
        assert!(drawn_in(&live, at, &p, row), "{tag}: hover {row} off its box");
        p.pressed(x, y);
        assert_eq!(lit(&live, at, &p), (Some(row), Some(row)), "{tag}: press {row}");
        assert!(drawn_in(&live, at, &p, row), "{tag}: press {row} off its box");
        p.moved(ox, oy);
        assert_eq!(lit(&live, at, &p), (Some(other), None), "{tag}: dragged off {row}");
        p.moved(at.x as i32 - 4, y);
        assert_eq!(lit(&live, at, &p), (None, None), "{tag}: left the menu");
    }
    let ((x, y), mut p) = (row_at(&live, at, 1), Pointer::new());
    p.moved(x, y);
    assert_eq!((dim.hit(at, x, y), lit(&dim, at, &p)), (None, (None, None)), "{tag}: off row");
    let same = |(x, y)| dim.hit(at, x, y) == live.hit(at, x, y).filter(|&r| r != 1);
    assert!(inside(at).all(same), "{tag}: dimming a row moved the others");
}
