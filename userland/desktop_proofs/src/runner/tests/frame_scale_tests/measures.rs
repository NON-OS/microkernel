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

//! The frame's measures grow by the shell's scale and rounding.

use super::super::super::chrome::quarters_for;
use super::rig::{px, H, SCALES, W};
use crate::shell_scale;
use nonos_toolkit::decorations::{
    accessory_rect, accessory_rect_at, border_at, chrome_growth_at, content_rect, content_rect_at,
    frame_rect_at, hit_test, hit_test_at, margin_at, titlebar_h_at, TITLEBAR_H,
};

#[test]
fn the_frame_takes_the_shells_scale_for_its_display() {
    for (w, h) in [(1366, 768), (1920, 1080), (2560, 1600), (3840, 2160)] {
        assert_eq!(quarters_for(w, h), shell_scale::quarters_for(w, h), "{w}x{h}");
    }
}

#[test]
fn every_measure_grows_by_the_shells_rounding() {
    for q in SCALES {
        assert_eq!(titlebar_h_at(q), px(TITLEBAR_H, q), "title bar at {q}");
        assert_eq!(margin_at(false, q), px(10, q), "shadow margin at {q}");
        assert_eq!(margin_at(true, q), 0, "a maximised window has no margin");
        assert_eq!(border_at(q), px(1, q), "border at {q}");
        let f = frame_rect_at(W, H, false, q);
        let c = content_rect_at(W, H, false, q);
        assert_eq!(c.y, f.y + titlebar_h_at(q), "content starts under the bar at {q}");
        assert_eq!(c.x, f.x + border_at(q));
        assert_eq!(c.x + c.w + border_at(q), f.x + f.w, "the right border at {q}");
    }
}

#[test]
fn one_to_one_is_the_frame_every_other_caller_already_has() {
    for (x, y) in [(20, 20), (30, 25), (50, 30), (300, 30), (300, 60), (885, 15)] {
        assert!(hit_test(W, H, false, x, y) == hit_test_at(W, H, false, x, y, 4), "{x},{y}");
    }
    assert_eq!(content_rect(W, H, false), content_rect_at(W, H, false, 4));
    assert_eq!(accessory_rect(W, H, false, 200), accessory_rect_at(W, H, false, 200, 4));
}

#[test]
fn a_window_grows_by_the_frame_so_its_content_keeps_its_size() {
    for q in SCALES {
        let (gw, gh) = chrome_growth_at(q);
        let asked = content_rect(760, 520, false);
        let got = content_rect_at(760 + gw, 520 + gh, false, q);
        assert_eq!((got.w, got.h), (asked.w, asked.h), "content at {q}");
    }
    assert_eq!(chrome_growth_at(4), (0, 0));
}
