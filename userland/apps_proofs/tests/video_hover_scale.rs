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

//! The video player's hover at 1x, 1.25x, 1.5x and 2x, fed as the window
//! feeds it: a move inside one control repaints nothing and one onto the next
//! repaints, and the leave names and lights nothing. Its own binary, so
//! setting the scale races no other test.

use apps_proofs::ui::hover::{Hover, Lit};
use apps_proofs::ui::screen::Route;
use apps_proofs::ui::targets::targets;
use apps_proofs::video_check::{feed, points, window, windows};
use nonos_app_skeleton::InputKind::PointerAbs;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::State as Look;

fn a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints(q: u32) {
    for s in windows() {
        let ts: Vec<_> = targets(&s).into_iter().filter(|(_, r)| r.w > 0 && r.h > 0).collect();
        let mut h = Hover::new();
        for pair in ts.windows(2) {
            let ((a, ra), (b, rb)) = (pair[0], pair[1]);
            let [mid, corner, ..] = points(ra);
            feed(&mut h, &s, PointerAbs, corner);
            assert!(!feed(&mut h, &s, PointerAbs, mid), "q={q}: {a:?} in {s:?}");
            assert!(feed(&mut h, &s, PointerAbs, points(rb)[0]), "q={q}: {b:?} in {s:?}");
            assert!(Lit(&h, &s).of(b).hovered && Lit(&h, &s).of(a) == Look::default(), "q={q}");
        }
    }
}

fn the_leave_names_nothing_and_lights_nothing(q: u32) {
    for route in [Route::Library, Route::Player] {
        let (s, mut h) = (window(route), Hover::new());
        let ts = targets(&s);
        let (t, r) = ts[0];
        assert!(feed(&mut h, &s, PointerAbs, points(r)[0]) && Lit(&h, &s).of(t).hovered, "q={q}");
        assert!(feed(&mut h, &s, PointerAbs, (-1, -1)), "q={q}");
        assert!(h.over.is_none() && h.pointer.position().is_none(), "q={q}");
        assert!(ts.iter().all(|&(t, _)| Lit(&h, &s).of(t) == Look::default()), "q={q}");
        assert!(!feed(&mut h, &s, PointerAbs, (-1, -1)), "q={q}");
    }
}

#[test]
fn the_hover_follows_the_drawn_controls_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints(q);
        the_leave_names_nothing_and_lights_nothing(q);
    }
    set_scale(4);
}
