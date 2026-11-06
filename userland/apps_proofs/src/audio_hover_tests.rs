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
//! The audio player's hover, fed as the window feeds it: a move inside one
//! control repaints nothing and one onto the next repaints, the leave names
//! and lights nothing, and a press lights only its own control.

use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::widgets::State as Look;

use crate::audio_check::{feed, points, window, windows};
use crate::audio_ui::hover::{Hover, Lit};
use crate::audio_ui::targets::targets;
use crate::audio_ui::view::View;

#[test]
fn a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints() {
    for s in windows() {
        let ts: Vec<_> = targets(&s).into_iter().filter(|(_, r)| r.w > 0 && r.h > 0).collect();
        let mut h = Hover::new();
        for pair in ts.windows(2) {
            let ((a, ra), (b, rb)) = (pair[0], pair[1]);
            let [mid, corner, ..] = points(ra);
            feed(&mut h, &s, PointerAbs, corner);
            assert!(!feed(&mut h, &s, PointerAbs, mid), "{a:?} in {:?}", s.view);
            assert!(feed(&mut h, &s, PointerAbs, points(rb)[0]), "{b:?} in {:?}", s.view);
            assert!(Lit(&h, &s).of(b).hovered && Lit(&h, &s).of(a) == Look::default());
        }
    }
}

#[test]
fn the_leave_names_nothing_and_lights_nothing() {
    let (s, mut h) = (window(View::Library), Hover::new());
    let ts = targets(&s);
    let (t, r) = ts[0];
    assert!(feed(&mut h, &s, PointerAbs, points(r)[0]) && Lit(&h, &s).of(t).hovered);
    assert!(feed(&mut h, &s, PointerAbs, (-1, -1)));
    assert!(h.over.is_none() && h.pointer.position().is_none());
    assert!(ts.iter().all(|&(t, _)| Lit(&h, &s).of(t) == Look::default()));
    assert!(!feed(&mut h, &s, PointerAbs, (-1, -1)));
}

#[test]
fn a_press_lights_only_its_control_and_its_release_repaints() {
    let (s, mut h) = (window(View::Settings), Hover::new());
    let ts = targets(&s);
    let ((first, r0), (next, r1)) = (ts[0], ts[1]);
    let (a, b) = (points(r0)[0], points(r1)[0]);
    h.pointer.pressed(a.0, a.1);
    assert!(h.settle(&s) && Lit(&h, &s).of(first).pressed);
    assert!(feed(&mut h, &s, PointerAbs, b));
    assert!(Lit(&h, &s).of(next).hovered && !Lit(&h, &s).of(next).pressed);
    assert!(feed(&mut h, &s, ButtonUp, b));
    assert!(!feed(&mut h, &s, ButtonDown, b) && !feed(&mut h, &s, Touch, (0, 0)));
}
