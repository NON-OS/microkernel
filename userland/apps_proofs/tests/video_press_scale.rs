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

//! The video player's press at 1x, 1.25x, 1.5x and 2x, fed as the window feeds
//! it: a press lights only its own control and its release repaints. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::ui::hover::{Hover, Lit};
use apps_proofs::ui::screen::Route;
use apps_proofs::ui::targets::targets;
use apps_proofs::video_check::{feed, points, window};
use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::design::scale::set_scale;

fn a_press_lights_only_its_control_and_its_release_repaints(q: u32) {
    let (s, mut h) = (window(Route::Player), Hover::new());
    let ts = targets(&s);
    let ((first, r0), (next, r1)) = (ts[0], ts[1]);
    let (a, b) = (points(r0)[0], points(r1)[0]);
    h.pointer.pressed(a.0, a.1);
    assert!(h.settle(&s) && Lit(&h, &s).of(first).pressed, "q={q}");
    assert!(feed(&mut h, &s, PointerAbs, b), "q={q}");
    assert!(Lit(&h, &s).of(next).hovered && !Lit(&h, &s).of(next).pressed, "q={q}");
    assert!(feed(&mut h, &s, ButtonUp, b), "q={q}");
    assert!(!feed(&mut h, &s, ButtonDown, b) && !feed(&mut h, &s, Touch, (0, 0)), "q={q}");
}

#[test]
fn a_press_lights_only_its_control_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        a_press_lights_only_its_control_and_its_release_repaints(q);
    }
    set_scale(4);
}
