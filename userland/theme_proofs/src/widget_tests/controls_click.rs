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

//! Controls: a click acts on release over the control it began on, the
//! press-acting variant acts on landing, and a disabled control never acts.

use nonos_toolkit::widgets::{Controls, Rect, Step};

use super::controls_hover::{two, A, B};

#[test]
fn a_click_acts_on_release_over_the_control_it_began_on() {
    let mut c = two();
    assert_eq!(c.pressed(20, 15), Step::Repaint);
    assert_eq!(c.released(), Step::Act('a'));
    assert_eq!(c.released(), Step::Idle);
}

#[test]
fn a_press_dragged_off_or_onto_another_control_acts_on_nothing() {
    let mut c = two();
    c.pressed(20, 15);
    c.moved(70, 15);
    assert!(!c.state('b').pressed);
    assert_eq!(c.released(), Step::Repaint);
    c.pressed(20, 15);
    c.moved(0, 0);
    assert_eq!(c.released(), Step::Repaint);
}

#[test]
fn a_press_dragged_off_and_back_still_acts() {
    let mut c = two();
    c.pressed(20, 15);
    c.moved(0, 0);
    c.moved(30, 20);
    assert_eq!(c.released(), Step::Act('a'));
}

#[test]
fn press_acting_controls_act_on_landing_and_not_again_on_release() {
    let mut c = Controls::acting_on_press();
    c.push(A, 'a').push(B, 'b');
    assert_eq!(c.pressed(70, 15), Step::Act('b'));
    assert_eq!(c.released(), Step::Repaint);
    assert_eq!(c.pressed(0, 0), Step::Repaint);
    assert_eq!(c.pressed(0, 0), Step::Idle);
}

#[test]
fn a_disabled_control_never_lights_or_acts_and_covers_what_is_beneath() {
    for mut c in [Controls::new(), Controls::acting_on_press()] {
        c.push(Rect { x: 0, y: 0, w: 100, h: 100 }, 'p').push_disabled(A, 'a');
        assert_eq!(c.moved(20, 15), Step::Idle);
        assert_eq!(c.pressed(20, 15), Step::Idle);
        assert_eq!(c.released(), Step::Idle);
        let s = c.state('a');
        assert!(s.disabled && !s.hovered && !s.pressed);
        assert_eq!((c.at(20, 15), c.state('p').hovered), (None, false));
    }
}
