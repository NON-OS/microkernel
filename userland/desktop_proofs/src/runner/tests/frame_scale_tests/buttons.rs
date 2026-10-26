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
//! Each control answers where it is drawn, at every scale: on the right,
//! minimize, maximize, close, each a whole target of at least 24 px.

use super::rig::{px, H, SCALES, W};
use nonos_toolkit::decorations::{
    accessory_rect_at, control_rect_at, controls_left_at, frame_rect_at, hit_test_at,
    titlebar_rect_at, DecorationHit, CONTROLS,
};

#[test]
fn each_control_answers_where_it_is_drawn_at_every_scale() {
    assert_eq!(
        CONTROLS,
        [DecorationHit::MinimizeButton, DecorationHit::MaximizeButton, DecorationHit::CloseButton]
    );
    for q in SCALES {
        let (bar, f) = (titlebar_rect_at(W, H, false, q), frame_rect_at(W, H, false, q));
        let mut last_right = bar.x + bar.w / 2;
        for (i, want) in CONTROLS.iter().enumerate() {
            let r = control_rect_at(i as u32, W, H, false, q);
            assert_eq!((r.w, r.h), (px(36, q), px(28, q)), "control {i} is 36 by 28 at {q}");
            assert!(r.x >= last_right, "controls do not overlap at {q}");
            last_right = r.x + r.w;
            assert!(r.y >= bar.y && r.y + r.h <= bar.y + bar.h, "control {i} sits in the bar");
            for (x, y) in [(r.x, r.y), (r.x + r.w - 1, r.y + r.h - 1), (r.x + r.w / 2, r.y)] {
                assert!(hit_test_at(W, H, false, x, y, q) == *want, "control {i} at {x},{y} {q}");
            }
        }
        assert_eq!(last_right, f.x + f.w - px(4, q), "close ends an inset short of the edge");
        let first = control_rect_at(0, W, H, false, q);
        let mid = first.y + first.h / 2;
        assert!(hit_test_at(W, H, false, first.x - 1, mid, q) == DecorationHit::Titlebar);
        assert!(hit_test_at(W, H, false, last_right, mid, q) == DecorationHit::Titlebar);
        // Past the controls the bar moves the window; below it is content.
        let x = bar.x + bar.w / 3;
        assert!(hit_test_at(W, H, false, x, bar.y + bar.h - 1, q) == DecorationHit::Titlebar);
        assert!(hit_test_at(W, H, false, x, bar.y + bar.h, q) == DecorationHit::None);
    }
}

#[test]
fn every_target_is_at_least_24_at_one_to_one() {
    let r = control_rect_at(0, W, H, false, 4);
    assert!(r.w >= 24 && r.h >= 24, "{r:?}");
}

#[test]
fn an_accessory_stays_clear_of_the_controls_at_every_scale() {
    for q in SCALES {
        let room = controls_left_at(W, H, false, q) - px(12, q);
        let a = accessory_rect_at(W, H, false, 240, q).expect("a 900 wide window has room");
        assert!(a.x + a.w == room && a.w == 240, "ends an inset short of minimize at {q}");
        let wide = accessory_rect_at(W, H, false, 5000, q).expect("cut to the room");
        assert!(wide.x >= titlebar_rect_at(W, H, false, q).x + px(12, q), "inside at {q}");
    }
}
