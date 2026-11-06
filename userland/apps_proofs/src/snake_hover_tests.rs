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
//! Snake's hover: a move inside one control asks for no repaint and one onto
//! the next asks for one, the router's leave names and lights nothing, and a
//! press lights only the control it began on.

use nonos_app_skeleton::EventOutcome::{self, Idle, Repaint};
use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonDown, ButtonUp, PointerAbs};
use nonos_toolkit::widgets::{Rect, State as Look};

use crate::snake::input::hover::Hover;
use crate::snake::state::Screen;
use crate::snake_controls_tests::{drawn, SCREENS};

fn feed(h: &mut Hover, s: Screen, kind: InputKind, (x, y): (i32, i32)) -> Option<EventOutcome> {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    h.on_pointer(s, &InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns })
}

fn mid(r: Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

#[test]
fn a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints() {
    for s in SCREENS {
        let mut h = Hover::new();
        for pair in drawn(s).windows(2) {
            let ((a, ra), (b, rb)) = (pair[0], pair[1]);
            feed(&mut h, s, PointerAbs, (ra.x as i32, ra.y as i32));
            assert!(feed(&mut h, s, PointerAbs, mid(ra)) == Some(Idle), "{a:?}");
            assert!(feed(&mut h, s, PointerAbs, mid(rb)) == Some(Repaint), "{b:?}");
            assert!(h.look(s, b).hovered && h.look(s, a) == Look::default());
        }
    }
}

#[test]
fn the_leave_names_nothing_and_lights_nothing() {
    let (s, mut h) = (Screen::Home, Hover::new());
    feed(&mut h, s, PointerAbs, mid(drawn(s)[0].1));
    assert!(h.look(s, drawn(s)[0].0).hovered);
    assert!(feed(&mut h, s, PointerAbs, (-1, -1)) == Some(Repaint));
    assert!(h.target.is_none() && h.pointer.position().is_none());
    assert!(drawn(s).iter().all(|&(t, _)| h.look(s, t) == Look::default()));
    assert!(feed(&mut h, s, PointerAbs, (-1, -1)) == Some(Idle));
}

#[test]
fn a_press_lights_only_its_control_and_its_release_repaints() {
    let (s, mut h) = (Screen::Pause, Hover::new());
    let ((first, r0), (next, r1)) = (drawn(s)[0], drawn(s)[1]);
    h.pointer.pressed(mid(r0).0, mid(r0).1);
    assert!(h.rehover(s) && h.look(s, first).pressed);
    assert!(feed(&mut h, s, PointerAbs, mid(r1)) == Some(Repaint));
    assert!(h.look(s, next).hovered && !h.look(s, next).pressed);
    assert!(feed(&mut h, s, ButtonUp, mid(r1)) == Some(Repaint));
    assert!(feed(&mut h, s, ButtonDown, mid(r1)).is_none());
}
