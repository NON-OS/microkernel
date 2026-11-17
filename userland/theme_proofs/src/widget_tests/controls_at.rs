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

//! Controls: `at` names exactly what `state` lights, the box pushed last wins
//! where boxes overlap, and a touch clicks on lift without hovering.

use nonos_toolkit::widgets::{Controls, Rect, State, Step};

use super::controls_hover::{two, A};

#[test]
fn at_names_exactly_the_control_whose_state_is_hovered() {
    let mut c = two();
    for (x, y) in [(0, 0), (10, 10), (49, 29), (50, 15), (60, 10), (99, 29), (-1, -1)] {
        c.moved(x, y);
        for a in ['a', 'b'] {
            assert_eq!(c.state(a).hovered, c.at(x, y) == Some(a), "{a} at {x},{y}");
        }
    }
}

#[test]
fn the_box_pushed_last_is_the_one_under_an_overlap() {
    let mut c = Controls::new();
    c.set([(Rect { x: 0, y: 0, w: 100, h: 100 }, 'p'), (A, 'a')]);
    assert_eq!((c.at(20, 15), c.at(80, 80)), (Some('a'), Some('p')));
    c.moved(20, 15);
    assert_eq!((c.state('a').hovered, c.state('p')), (true, State::default()));
}

#[test]
fn a_touch_acts_on_lift_and_hovers_nothing() {
    let mut c = two();
    assert_eq!(c.touched(20, 15), Step::Repaint);
    assert_eq!((c.state('a').hovered, c.state('a').pressed), (false, true));
    assert_eq!(c.released(), Step::Act('a'));
    assert_eq!(c.lit(), None);
}
