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

//! The calculator's hover: a move inside one control repaints nothing and one
//! onto the next repaints, the leave lights nothing, and a press lights only its own.

use nonos_app_skeleton::EventOutcome::{self, Idle, Repaint};
use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::widgets::{Rect, State as Look};

use crate::calc::event::hover::{look, on_pointer, rehover};
use crate::calc::mode::Mode;
use crate::calc::state::State;
use crate::calc::ui::target::targets;
use crate::calc_controls_tests::{window, windows};

fn feed(s: &mut State, kind: InputKind, (x, y): (i32, i32)) -> Option<EventOutcome> {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    on_pointer(s, &InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns })
}

pub fn mid(r: Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

#[test]
fn a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints() {
    for mut s in windows() {
        for pair in targets(&s).windows(2) {
            let ((a, ra), (b, rb)) = (pair[0], pair[1]);
            feed(&mut s, PointerAbs, (ra.x as i32, ra.y as i32));
            assert!(feed(&mut s, PointerAbs, mid(ra)) == Some(Idle), "{ra:?}");
            assert!(feed(&mut s, PointerAbs, mid(rb)) == Some(Repaint), "{rb:?}");
            assert!(look(&s, b).hovered && look(&s, a) == Look::default());
        }
    }
}

#[test]
fn the_leave_names_nothing_and_lights_nothing() {
    let mut s = window(Mode::Basic, 0, 0);
    let ts = targets(&s);
    let (key, r) = *ts.last().unwrap();
    assert!(feed(&mut s, PointerAbs, mid(r)) == Some(Repaint) && look(&s, key).hovered);
    assert!(feed(&mut s, PointerAbs, (-1, -1)) == Some(Repaint));
    assert!(s.hover.is_none() && s.pointer.position().is_none());
    assert!(ts.iter().all(|&(t, _)| look(&s, t) == Look::default()));
    assert!(feed(&mut s, PointerAbs, (-1, -1)) == Some(Idle));
}

#[test]
fn a_press_lights_only_its_control_and_its_release_repaints() {
    let mut s = window(Mode::Programmer, 0, 0);
    let (&(first, r0), &(next, r1)) = (&targets(&s)[0], &targets(&s)[1]);
    s.pointer.pressed(mid(r0).0, mid(r0).1);
    assert!(rehover(&mut s) && look(&s, first).pressed);
    assert!(feed(&mut s, PointerAbs, mid(r1)) == Some(Repaint));
    assert!(look(&s, next).hovered && !look(&s, next).pressed);
    assert!(feed(&mut s, ButtonUp, mid(r1)) == Some(Repaint));
    assert!(feed(&mut s, ButtonDown, mid(r1)).is_none() && feed(&mut s, Touch, (0, 0)).is_none());
}
