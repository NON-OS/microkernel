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

//! The wheel turns the Launchpad's pages: three notches a page, a notch
//! toward the user (negative, as the drivers post it) on to the next page,
//! a notch away back toward the first, and never past the first or last.

use crate::launchpad_wheel::{PageWheel, NOTCHES_PER_PAGE};

/// Feed single notches, as a mouse wheel or a two-finger swipe posts them.
fn notches(w: &mut PageWheel, mut page: usize, pages: usize, delta: i32, n: usize) -> usize {
    for _ in 0..n {
        page = w.turn(page, pages, delta);
    }
    page
}

#[test]
fn three_notches_toward_the_user_turn_to_the_next_page() {
    let mut w = PageWheel::default();
    assert_eq!(NOTCHES_PER_PAGE, 3);
    assert_eq!(notches(&mut w, 0, 4, -1, 2), 0, "two notches do not turn yet");
    assert_eq!(w.turn(0, 4, -1), 1, "the third does");
    assert_eq!(notches(&mut w, 1, 4, -1, 3), 2);
}

#[test]
fn a_notch_away_goes_back_toward_the_first_page() {
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 3, 4, 1, 3), 2);
    assert_eq!(w.turn(2, 4, 6), 0, "a step of six notches turns two pages");
}

#[test]
fn the_first_and_last_pages_hold_and_the_way_back_starts_at_once() {
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 0, 3, 1, 30), 0, "nothing before the first page");
    assert_eq!(notches(&mut w, 0, 3, -1, 3), 1, "nothing banked against the top");
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 0, 3, -1, 60), 2, "nothing past the last page");
    assert_eq!(notches(&mut w, 2, 3, 1, 3), 1);
    let mut w = PageWheel::default();
    assert_eq!(notches(&mut w, 0, 1, -1, 9), 0, "a single page does not turn");
}

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
