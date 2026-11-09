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

use capsule_browser_proofs::browser::start::badge::paint as badge;
use capsule_browser_proofs::browser::ui::rect::Rect;

use super::kit::{at_scale, ink_box, inside, paint, W};

const HERO: Rect = Rect { x: 0, y: 0, w: W, h: 400 };

#[test]
fn the_badge_sits_in_the_heros_corner_clear_of_the_greeting() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let blank = paint(|_| {});
        let greet = W / 2;
        let drawn = ink_box(&blank, &paint(|fb| badge(fb, HERO, greet))).expect("no badge");
        assert!(inside(drawn, HERO), "{drawn:?} at {q}");
        assert!(drawn.x > greet && drawn.right() > W - 40 && drawn.y < 40, "{drawn:?} at {q}");
    }
}

#[test]
fn the_badge_is_left_out_when_it_would_meet_the_greeting() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let blank = paint(|_| {});
        assert_eq!(ink_box(&blank, &paint(|fb| badge(fb, HERO, W - 200))), None, "{q}");
        let up = Rect { y: -30, ..HERO };
        assert_eq!(ink_box(&blank, &paint(|fb| badge(fb, up, 0))), None, "{q}");
    }
}
