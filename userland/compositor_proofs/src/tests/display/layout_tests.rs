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

//! One canvas across several outputs: output 0 keeps the canvas it has
//! alone at the left, each further output a slice its screen holds at its
//! own scale, the canvas as tall as the tallest; and a canvas rectangle cut
//! into each output's part, in that output's own coordinates.

use crate::damage::Rect;
use crate::display::layout_rule::{local, plan, Slice};

fn t(r: Option<Rect>) -> Option<(u32, u32, u32, u32)> {
    r.map(|r| (r.x, r.y, r.width, r.height))
}

#[test]
fn outputs_sit_side_by_side_at_the_canvas_scale() {
    let p = plan((1920, 1080), 4, &[(1280, 1024, 4)]).unwrap();
    assert_eq!((p.size, p.n), ((3200, 1080), 2));
    assert_eq!(p.slices[1], Slice { x: 1920, w: 1280, h: 1024, q: 4 });
    let doubled = plan((1280, 720), 8, &[(3840, 2160, 8), (1920, 1080, 8)]).unwrap();
    assert_eq!(doubled.size, (1280 + 1920 + 960, 1080), "4K at half, 1080p at half");
    assert_eq!(doubled.slices[2].x, 3200);
}

#[test]
fn an_empty_slice_or_too_many_outputs_is_no_plan() {
    assert_eq!(plan((1920, 1080), 8, &[(1, 1080, 8)]), None, "half a pixel wide");
    let five = [(800, 600, 4); 4];
    assert_eq!(plan((800, 600), 4, &five), None, "the wire serves four");
    assert_eq!(plan((4096, 2160), 4, &[(4096, 2160, 4); 3]).map(|p| p.size.0), Some(16384));
}

#[test]
fn a_rectangle_is_cut_into_each_outputs_part() {
    let s = Slice { x: 1920, w: 1280, h: 1024, q: 4 };
    let r = Rect { x: 1900, y: 1000, width: 100, height: 80 };
    assert_eq!(t(local(r, s)), Some((0, 1000, 80, 24)), "below 1024 is no screen");
    assert_eq!(t(local(Rect { x: 0, y: 0, width: 1920, height: 10 }, s)), None);
    assert_eq!(local(Rect { x: 3199, y: 0, width: 10, height: 1 }, s).map(|r| r.width), Some(1));
}
