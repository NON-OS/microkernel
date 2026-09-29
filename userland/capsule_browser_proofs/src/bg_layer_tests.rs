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
/* background-size and background-position: both parse from the longhands
 * and from the background shorthand, and together they place the tile the
 * painter draws and repeats from. */

use crate::probe::Page;

const VP: (u32, u32) = (1200, 800);

fn layer(style: &str) -> crate::browser::css::BgLayer {
    let html = format!("<div id=d style=\"width:1000px;height:500px;{style}\"></div>");
    Page::at(&html, VP).frag("d").bg_layer
}

#[test]
fn the_shorthand_places_a_tile_by_size_and_position() {
    let l = layer("background:url(x.jpg) 62% 16% / auto 165% no-repeat");
    assert_eq!(l.pos, [(0, 620), (0, 160)]);
    assert!(!l.repeat);
    /* 165% of 500 is 825 tall, 1467 wide at 16:9; 62% and 16% of the room
     * left over (negative, the tile is larger than the box). */
    assert_eq!(l.tile((3200, 1800), [0, 0, 1000, 500]), [-289, -52, 1467, 825]);
}

#[test]
fn position_keywords_edges_and_a_lone_value() {
    let at = |v: &str| layer(&format!("background-position:{v}")).pos;
    assert_eq!(at("top"), [(0, 500), (0, 0)], "a lone vertical keyword centres x");
    assert_eq!(at("20px"), [(20, 0), (0, 500)]);
    assert_eq!(at("bottom right"), [(0, 1000), (0, 1000)], "swapped into x, y");
    assert_eq!(at("right 10px bottom 20px"), [(-10, 1000), (-20, 1000)]);
    assert_eq!(at("center 25%"), [(0, 500), (0, 250)]);
}

#[test]
fn size_forms_resolve_against_the_box_and_the_image() {
    let tile = |v: &str| layer(&format!("background-size:{v}")).tile((200, 100), [0, 0, 1000, 500]);
    assert_eq!(tile("auto"), [0, 0, 200, 100], "natural size at 0% 0%");
    assert_eq!(tile("50%"), [0, 0, 500, 250], "a width keeps the aspect");
    assert_eq!(tile("cover"), [0, 0, 1000, 500]);
    assert_eq!(tile("contain"), [0, 0, 1000, 500]);
    assert_eq!(tile("100px 30px"), [0, 0, 100, 30]);
    let centred = layer("background:url(x.png) center / contain");
    assert_eq!(centred.tile((100, 100), [0, 0, 1000, 500]), [250, 0, 500, 500]);
}
