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

#![cfg(test)]
/* min/max sizes on an image: with both sides auto the ratio holds while
 * the bounds are met (CSS 2.1 10.4), and a set side is bounded alone. */

use crate::browser::layout::boxmodel::Content;
use crate::render::render_with;

fn size(style: &str, nat: (u32, u32)) -> [i32; 2] {
    let html = format!("<body style=\"margin:0\"><img src=x.png style=\"{style}\"></body>");
    let doc = render_with(&html, (800, 600), &|_| Some(nat));
    let f = doc.frags.iter().find(|f| matches!(f.content, Content::Image { .. })).unwrap();
    [f.w, f.h]
}

#[test]
fn both_maxima_keep_the_ratio() {
    let logo = "max-width:48px;max-height:40px";
    assert_eq!(size(logo, (626, 626)), [40, 40], "the height bound is tighter");
    assert_eq!(size(logo, (1200, 150)), [48, 6], "the width bound is tighter");
    assert_eq!(size("max-height:50px", (200, 100)), [100, 50]);
}

#[test]
fn a_minimum_grows_a_small_image() {
    assert_eq!(size("min-width:60px", (30, 10)), [60, 20]);
    assert_eq!(size("min-height:30px", (30, 10)), [90, 30]);
}

#[test]
fn a_set_side_is_bounded_and_the_other_follows() {
    assert_eq!(size("width:300px;max-width:100px", (200, 100)), [100, 50]);
    assert_eq!(size("height:80px;max-height:40px", (200, 100)), [80, 40]);
}
