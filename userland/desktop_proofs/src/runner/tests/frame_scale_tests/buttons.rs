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

//! Each button answers where it is drawn, at every scale.

use super::rig::{px, H, SCALES, W};
use nonos_toolkit::decorations::{hit_test_at, light_rect_at, titlebar_rect_at, DecorationHit};

#[test]
fn each_button_answers_where_it_is_drawn_at_every_scale() {
    let hits =
        [DecorationHit::CloseButton, DecorationHit::MinimizeButton, DecorationHit::MaximizeButton];
    for q in SCALES {
        let bar = titlebar_rect_at(W, H, false, q);
        let mut last_right = bar.x;
        for (i, want) in hits.iter().enumerate() {
            let r = light_rect_at(i as u32, W, H, false, q);
            assert_eq!(r.w, px(12, q), "button {i} is the scaled size at {q}");
            assert!(r.x >= last_right, "buttons do not overlap at {q}");
            last_right = r.x + r.w;
            assert!(r.y >= bar.y && r.y + r.h <= bar.y + bar.h, "button {i} sits in the bar");
            let (cx, cy) = (r.x + r.w / 2, r.y + r.h / 2);
            assert!(hit_test_at(W, H, false, cx, cy, q) == *want, "button {i} centre at {q}");
            // The target reaches past the drawn circle by the scaled pad.
            let pad = px(4, q);
            assert!(
                hit_test_at(W, H, false, r.x + r.w + pad - 1, cy, q) != DecorationHit::Titlebar
            );
        }
        // Past the buttons the bar moves the window, below it is content.
        let mid = bar.x + bar.w / 2;
        assert!(hit_test_at(W, H, false, mid, bar.y + bar.h - 1, q) == DecorationHit::Titlebar);
        assert!(hit_test_at(W, H, false, mid, bar.y + bar.h, q) == DecorationHit::None);
    }
}
