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

//! What a turn forgets: the other way, a flick's excess, a reset.

use super::turns::notches;
use crate::launchpad_wheel::PageWheel;

#[test]
fn a_change_of_direction_drops_what_was_added_the_other_way() {
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 1, 4, -1, 2), 1);
    // Two notches toward the next page, then three back: the three turn back
    // a whole page, not one notch short.
    assert_eq!(notches(&mut w, 1, 4, 1, 3), 0);
}

#[test]
fn a_flick_is_capped_and_no_notch_changes_nothing() {
    let mut w = PageWheel::default();
    assert_eq!(w.turn(0, 100, i32::MIN), 3, "ten notches at most: three pages");
    let mut w = PageWheel::default();
    assert_eq!(w.turn(5, 100, i32::MAX), 2);
    assert_eq!(w.turn(4, 100, 0), 4);
    // A page past the end (the filter just shrank the grid) comes back to it.
    assert_eq!(w.turn(9, 3, 0), 2);
}

#[test]
fn reset_forgets_what_was_added() {
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 0, 4, -1, 2), 0);
    w.reset();
    assert_eq!(w.turn(0, 4, -1), 0, "the overlay was closed: start over");
}
