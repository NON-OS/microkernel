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

//! Menu: tagging rows moves no hit. Held to one width, a tagged menu and its
//! items untagged are one size, and every point in and round every box hits
//! the same row, the whole of it, at every scale.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Menu, Rect};

use super::boxes::boxes;
use super::menu::MENUS;
use super::menu_tag::{tagged, TAG_SETS};
use super::serial::{serial, SCALES};

/// Every point of `at` and 4 px round it.
fn around(at: Rect) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = (at.x as i32 - 4, at.y as i32 - 4);
    let ys = y0..y0 + at.h as i32 + 8;
    ys.flat_map(move |y| (x0..x0 + at.w as i32 + 8).map(move |x| (x, y)))
}

#[test]
fn menu_tags_keep_every_hit() {
    let _g = serial();
    for q in SCALES {
        set_scale(q);
        for (name, items) in MENUS {
            for tags in TAG_SETS {
                let all = tagged(items, tags);
                let t = Menu::new(&all);
                let plain = Menu::new(items).width(t.size().0);
                assert_eq!(plain.size(), t.size(), "{name} with {} tags at {q}/4", tags.len());
                for at in boxes(t.size()) {
                    let moved = around(at).find(|&(x, y)| plain.hit(at, x, y) != t.hit(at, x, y));
                    assert_eq!(moved, None, "{name} with {} tags in {at:?} at {q}/4", tags.len());
                }
            }
        }
    }
    set_scale(SCALES[0]);
}
