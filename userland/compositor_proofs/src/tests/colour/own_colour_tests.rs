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

//! Mirrors in their own colours: chosen only when a mirror's filter or
//! contrast differs from output 0's, never extended; and a rectangle less
//! the band is exactly the rest of it.

use crate::damage::Rect;
use crate::display::access_rule::{Access, FILTER_GREYSCALE};
use crate::display::mirror_rule::{around, own_colours};

fn t(r: Rect) -> (u32, u32, u32, u32) {
    (r.x, r.y, r.width, r.height)
}

fn ts(rs: [Option<Rect>; 4]) -> [Option<(u32, u32, u32, u32)>; 4] {
    rs.map(|r| r.map(t))
}

pub const GREY: Access =
    Access { filter: FILTER_GREYSCALE, contrast: false, magnify: 4, follow: 0 };

#[test]
fn own_colours_only_when_a_mirrors_colours_differ() {
    let same = [Access::default(); 4];
    assert!(!own_colours(false, &same, 3));
    let zoomed = [Access::default(), Access { magnify: 8, ..Access::default() }, same[2], same[3]];
    assert!(!own_colours(false, &zoomed, 1), "a lens is not a colour");
    let grey = [Access::default(), GREY, same[2], same[3]];
    assert!(own_colours(false, &grey, 1));
    assert!(!own_colours(true, &grey, 1), "extended: each slice has its own already");
    assert!(!own_colours(false, &grey, 0), "no mirror shows it");
    let far = [Access::default(), same[1], same[2], GREY];
    assert!(!own_colours(false, &far, 2), "an output not mirrored now");
}

#[test]
fn a_rectangle_less_the_band_is_the_rest_of_it() {
    let r = Rect { x: 0, y: 0, width: 100, height: 40 };
    let hole = Rect { x: 60, y: 0, width: 40, height: 16 };
    let parts: Vec<Rect> = around(r, Some(hole)).into_iter().flatten().collect();
    let area: u32 = parts.iter().map(|p| p.width * p.height).sum();
    assert_eq!(area, 100 * 40 - 40 * 16, "every pixel but the hole's");
    for p in &parts {
        let clear = p.x + p.width <= hole.x || p.y >= hole.y + hole.height;
        assert!(clear, "{:?} reaches into the hole", t(*p));
    }
    assert_eq!(ts(around(r, None)), [Some(t(r)), None, None, None]);
    let away = Rect { x: 200, y: 0, width: 4, height: 4 };
    assert_eq!(ts(around(r, Some(away))), [Some(t(r)), None, None, None], "a hole elsewhere");
}
