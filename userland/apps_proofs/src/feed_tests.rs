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

//! The skeleton's `feed`: each pointer event reaches the toolkit's Controls as
//! the call it means, a move to (-1, -1) is the pointer leaving, and keys and
//! wheel turns ask for nothing.

use nonos_app_skeleton::InputKind::{
    self, ButtonDown, ButtonUp, KeyDown, PointerAbs, Touch, Wheel,
};
use nonos_app_skeleton::{feed, InputEvent};
use nonos_toolkit::widgets::{Controls, Rect, Step};

const A: Rect = Rect { x: 10, y: 10, w: 40, h: 20 };

pub(crate) fn ev(kind: InputKind, x: i32, y: i32) -> InputEvent {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns }
}

fn one() -> Controls<u8> {
    let mut c = Controls::new();
    c.push(A, 1);
    c
}

#[test]
fn a_move_down_and_up_on_a_control_hover_press_and_click_it() {
    let mut c = one();
    assert_eq!(feed(&mut c, &ev(PointerAbs, 20, 15)), Step::Repaint);
    assert!(c.state(1).hovered);
    assert_eq!(feed(&mut c, &ev(ButtonDown, 20, 15)), Step::Repaint);
    assert!(c.state(1).pressed);
    assert_eq!(feed(&mut c, &ev(ButtonUp, 20, 15)), Step::Act(1));
}

#[test]
fn a_move_to_minus_one_is_the_pointer_leaving_and_drops_a_held_press() {
    let mut c = one();
    feed(&mut c, &ev(ButtonDown, 20, 15));
    assert_eq!(feed(&mut c, &ev(PointerAbs, -1, -1)), Step::Repaint);
    assert_eq!((c.position(), c.state(1).hovered), (None, false));
    assert_eq!(feed(&mut c, &ev(ButtonUp, 20, 15)), Step::Idle);
}

#[test]
fn a_touch_clicks_on_lift_without_hovering() {
    let mut c = one();
    assert_eq!(feed(&mut c, &ev(Touch, 20, 15)), Step::Repaint);
    assert!(!c.state(1).hovered);
    assert_eq!(feed(&mut c, &ev(ButtonUp, 0, 0)), Step::Act(1));
}

#[test]
fn keys_and_wheel_turns_ask_nothing_of_the_controls() {
    let mut c = one();
    feed(&mut c, &ev(PointerAbs, 20, 15));
    for kind in [KeyDown, Wheel] {
        assert_eq!(feed(&mut c, &ev(kind, 20, 15)), Step::Idle);
    }
    assert!(c.state(1).hovered && !c.state(1).pressed);
}
