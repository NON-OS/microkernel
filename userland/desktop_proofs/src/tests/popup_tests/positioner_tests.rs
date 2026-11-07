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

//! A popup's place against its parent, as xdg_positioner defines it, kept on
//! the output by the adjustments its client allows.

use super::rig::{menu, Rng, DISPLAY};
use crate::geometry::positioner::{place, Positioner, FLIP_Y, RESIZE_Y, SLIDE_X};
use crate::geometry::Rect;

const OUTPUT: Rect = Rect { x: 0, y: 0, width: DISPLAY.0, height: DISPLAY.1 };

fn rect(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, width: w, height: h }
}

#[test]
fn a_menu_hangs_from_its_anchors_bottom_left_corner() {
    let parent = rect(100, 100, 600, 400);
    let p = place(&menu((10, 0, 80, 24), 200, 300), parent, OUTPUT);
    assert_eq!(p.rect, rect(110, 124, 200, 300));
    assert_eq!((p.rel_x, p.rel_y), (10, 24));
}

#[test]
fn a_popup_may_reach_past_its_parent_but_stays_on_the_output() {
    let parent = rect(100, 100, 200, 100);
    let p = place(&menu((150, 80, 40, 20), 300, 250), parent, OUTPUT);
    assert_eq!(p.rect, rect(250, 200, 300, 250), "past the parent's right and bottom edges");
    assert!(p.rect.x + p.rect.width > parent.x + parent.width);
}

#[test]
fn a_menu_with_no_room_below_flips_above_its_anchor() {
    let parent = rect(100, 500, 600, 290);
    let p = place(&menu((10, 250, 80, 24), 200, 300), parent, OUTPUT);
    // Below would end at 1074, past 800: above the anchor it fits.
    assert_eq!(p.rect, rect(110, 450, 200, 300));
    assert_eq!(p.rel_y, -50);
}

#[test]
fn a_flip_that_does_not_fit_either_is_not_taken() {
    let parent = rect(0, 300, 600, 200);
    let mut m = menu((10, 0, 80, 24), 200, 700);
    m.adjust = FLIP_Y | SLIDE_X;
    let p = place(&m, parent, OUTPUT);
    // Neither below (324..1024) nor above (-400..300) fits, so the flip is
    // not taken, and with no slide or resize allowed on y the last resort
    // keeps the menu on the output.
    assert_eq!(p.rect.height, 700);
    assert_eq!(p.rect.y, 100, "slid up just enough");
}

#[test]
fn a_slide_brings_a_popup_back_from_the_right_edge() {
    let parent = rect(1000, 100, 260, 300);
    let mut m = menu((200, 0, 40, 24), 300, 200);
    m.adjust = SLIDE_X;
    let p = place(&m, parent, OUTPUT);
    assert_eq!(p.rect, rect(980, 124, 300, 200));
    assert_eq!(p.rel_x, -20);
}

#[test]
fn a_resize_cuts_a_tall_list_to_the_output() {
    let parent = rect(0, 600, 400, 100);
    let mut m = menu((0, 40, 100, 20), 150, 400);
    m.adjust = RESIZE_Y;
    let p = place(&m, parent, OUTPUT);
    assert_eq!(p.rect, rect(0, 660, 150, 140));
}

#[test]
fn a_centred_tooltip_sits_on_its_anchors_middle() {
    let parent = rect(300, 300, 400, 300);
    let tip = Positioner { anchor: 2, gravity: 2, ..menu((100, 50, 60, 20), 120, 30) };
    let p = place(&tip, parent, OUTPUT);
    // Under the anchor's bottom edge, centred on its middle.
    assert_eq!(p.rect, rect(370, 370, 120, 30));
}

#[test]
fn an_offset_moves_the_popup_and_flips_with_it() {
    let parent = rect(100, 100, 600, 400);
    let m = Positioner { offset_x: 5, offset_y: 3, ..menu((10, 0, 80, 24), 200, 300) };
    assert_eq!(place(&m, parent, OUTPUT).rect, rect(115, 127, 200, 300));
    let low = rect(100, 500, 600, 290);
    let m = Positioner { offset_x: 5, offset_y: 3, ..menu((10, 250, 80, 24), 200, 300) };
    // Flipped above the anchor, the vertical offset turns upward too.
    assert_eq!(place(&m, low, OUTPUT).rect, rect(115, 447, 200, 300));
}

/// Whatever is asked, the popup lands on the output, at least a pixel each
/// way, and its place on the parent is its place on the output less the
/// parent's origin.
#[test]
fn every_placement_is_on_the_output() {
    let mut rng = Rng::new(0x506F_7075);
    for _ in 0..50_000 {
        let r = |rng: &mut Rng, n: u32| rng.below(n);
        let parent =
            rect(r(&mut rng, 1280), r(&mut rng, 800), 1 + r(&mut rng, 1280), 1 + r(&mut rng, 800));
        let p = Positioner {
            anchor_x: r(&mut rng, 3000) as i32 - 1500,
            anchor_y: r(&mut rng, 3000) as i32 - 1500,
            anchor_w: r(&mut rng, 600),
            anchor_h: r(&mut rng, 600),
            width: 1 + r(&mut rng, 2000),
            height: 1 + r(&mut rng, 2000),
            anchor: r(&mut rng, 9),
            gravity: r(&mut rng, 9),
            adjust: r(&mut rng, 64),
            offset_x: r(&mut rng, 400) as i32 - 200,
            offset_y: r(&mut rng, 400) as i32 - 200,
        };
        let out = place(&p, parent, OUTPUT);
        let q = out.rect;
        assert!(q.width >= 1 && q.height >= 1, "{p:?}");
        assert!(q.x + q.width <= OUTPUT.width && q.y + q.height <= OUTPUT.height, "{p:?} {q:?}");
        assert_eq!(i64::from(out.rel_x), i64::from(q.x) - i64::from(parent.x));
        assert_eq!(i64::from(out.rel_y), i64::from(q.y) - i64::from(parent.y));
        if q.width < p.width.min(OUTPUT.width) {
            assert!(p.adjust & 16 != 0, "only a resize narrows a popup that fits: {p:?}");
        }
    }
}
