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

//! A title-bar drag at any pace moves the window to where the pointer let go
//! and never resizes it: the press, every motion and the release arriving in
//! one drain, against one frame (KVM pacing), as much as one to a frame. The
//! window's place is the one the app's drain applies last, as apply_drain
//! applies the last move it was asked for.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState, PointerAction};
use crate::input::InputKind;
use crate::press::Press;

/// A window as Files opens at 1080p, and the test's drag: from 120,32 on its
/// title bar, 300 right and 150 down.
const AT: (u32, u32) = (667, 501);
const SIZE: (u32, u32) = (842, 421);
const FROM: (u32, u32) = (AT.0 + 120, AT.1 + 32);
const BY: (u32, u32) = (300, 150);

/// The drag with `steps` motions between press and release; `per_frame`
/// applies each move before the next event (one event a frame), else all of
/// them are drained against the frame the press saw. Where the window ends,
/// and whether anything asked to resize it.
fn drag(steps: u32, per_frame: bool) -> ((u32, u32), bool) {
    // The router's press grab: the origin fixed at the press, from the
    // window manager's hit test (route_to_press).
    let grab = Press::arm(1, FROM.0, FROM.1, FROM.0 - AT.0, FROM.1 - AT.1);
    let mut state = DragState::new();
    let mut at = AT;
    let mut frame = AT;
    let mut resized = false;
    let mut send =
        |kind: InputKind, sx: u32, sy: u32, at: &mut (u32, u32), frame: &mut (u32, u32)| {
            let (lx, ly) = grab.local(sx, sy);
            match drag::handle(&mut state, SIZE, *frame, false, &event(kind, lx, ly)) {
                PointerAction::MoveTo(x, y) => *at = (x, y),
                PointerAction::ResizeTo(..) | PointerAction::Place(_) => resized = true,
                _ => {}
            }
            if per_frame {
                *frame = *at;
            }
        };
    send(InputKind::ButtonDown, FROM.0, FROM.1, &mut at, &mut frame);
    for i in 1..=steps {
        let x = FROM.0 + BY.0 * i / steps.max(1);
        let y = FROM.1 + BY.1 * i / steps.max(1);
        send(InputKind::PointerAbs, x, y, &mut at, &mut frame);
    }
    let end = if steps == 0 { FROM } else { (FROM.0 + BY.0, FROM.1 + BY.1) };
    send(InputKind::ButtonUp, end.0, end.1, &mut at, &mut frame);
    (at, resized)
}

#[test]
fn a_drag_in_one_drain_ends_where_the_pointer_let_go() {
    for steps in [1, 2, 3, 12, 64] {
        let (at, resized) = drag(steps, false);
        assert_eq!(at, (AT.0 + BY.0, AT.1 + BY.1), "{steps} motions in one frame");
        assert!(!resized, "{steps} motions in one frame resized the window");
    }
}

#[test]
fn a_drag_one_event_a_frame_ends_where_the_pointer_let_go() {
    for steps in [1, 2, 12] {
        let (at, resized) = drag(steps, true);
        assert_eq!(at, (AT.0 + BY.0, AT.1 + BY.1), "{steps} motions, a frame each");
        assert!(!resized);
    }
}

#[test]
fn a_press_and_release_with_no_motion_leaves_the_window() {
    assert_eq!(drag(0, false), (AT, false));
}
