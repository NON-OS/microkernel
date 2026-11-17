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

//! Controls: hover follows the box under the pointer, a move repaints only on
//! crossing to another box, and leaving clears what was lit.

use nonos_toolkit::widgets::{Controls, Rect, Step};

pub const A: Rect = Rect { x: 10, y: 10, w: 40, h: 20 };
pub const B: Rect = Rect { x: 60, y: 10, w: 40, h: 20 };

pub fn two() -> Controls<char> {
    let mut c = Controls::new();
    c.push(A, 'a').push(B, 'b');
    c
}

fn look(c: &Controls<char>, a: char) -> (bool, bool) {
    let s = c.state(a);
    (s.hovered, s.pressed)
}

#[test]
fn a_move_repaints_only_when_the_lit_control_changes() {
    let mut c = two();
    assert_eq!(c.moved(0, 0), Step::Idle);
    assert_eq!(c.moved(20, 15), Step::Repaint);
    assert_eq!(c.moved(25, 18), Step::Idle);
    assert_eq!((look(&c, 'a'), look(&c, 'b')), ((true, false), (false, false)));
    assert_eq!(c.moved(70, 15), Step::Repaint);
    assert_eq!(c.moved(55, 15), Step::Repaint);
    assert_eq!(c.lit(), None);
}

#[test]
fn a_press_on_a_control_repaints_it_pressed_and_one_on_nothing_is_idle() {
    let mut c = two();
    assert_eq!(c.pressed(0, 0), Step::Idle);
    assert_eq!(c.released(), Step::Idle);
    assert_eq!(c.pressed(20, 15), Step::Repaint);
    assert_eq!(look(&c, 'a'), (true, true));
}

#[test]
fn leaving_clears_hover_and_press_and_repaints_once() {
    let mut c = two();
    c.pressed(20, 15);
    assert_eq!(c.left(), Step::Repaint);
    assert_eq!(look(&c, 'a'), (false, false));
    assert_eq!(c.left(), Step::Idle);
    assert_eq!(c.released(), Step::Idle);
}
