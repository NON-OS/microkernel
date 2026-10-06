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

//! aspect-ratio with an auto height: the ratio sets the height, but the
//! automatic minimum height (CSS Sizing 4) keeps content from spilling out
//! of a box that is not a scroll container.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

fn boxed(style: &str) -> Page {
    let html = format!(
        "<body style=margin:0><div id=b style='width:100px;aspect-ratio:2;{style}'>\
         <div id=c style='height:200px'></div></div><p id=n style=margin:0>n</p>"
    );
    Page::at(&html, VP)
}

/* 100px wide at 2:1 is 50px tall, but 200px of content grows it to 200,
 * and the next block starts below it; shorter content keeps the ratio. */
#[test]
fn a_ratio_box_grows_to_hold_its_content() {
    let p = boxed("");
    assert_eq!(p.rect("b")[3], 200, "min-height:auto is the content height");
    assert_eq!(p.rect("n")[1], 200, "the next block follows the content");
    let html = "<body style=margin:0><div id=b style='width:100px;aspect-ratio:2'>x</div>";
    assert_eq!(Page::at(html, VP).rect("b")[3], 50, "short content keeps the ratio");
}

/* A scroll container, an explicit min-height, and max-height all opt out
 * of or cap the content minimum. */
#[test]
fn overflow_min_height_and_max_height_bound_the_content_minimum() {
    assert_eq!(boxed("overflow:hidden").rect("b")[3], 50, "scroll container");
    assert_eq!(boxed("min-height:0").rect("b")[3], 50, "explicit min-height");
    assert_eq!(boxed("max-height:120px").rect("b")[3], 120, "capped by max-height");
}
