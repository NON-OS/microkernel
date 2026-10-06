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

//! The host controller lookup is a bounded search: it stops at the first
//! answer, pauses only between tries, and gives up after its last try. It
//! used to retry without end.

use std::cell::Cell;

use crate::find_within::find_within;

#[test]
fn a_controller_that_is_there_is_found_at_once_without_a_pause() {
    let pauses = Cell::new(0u32);
    let got = find_within(100, || Some(7u32), || pauses.set(pauses.get() + 1));
    assert_eq!((got, pauses.get()), (Some(7), 0));
}

#[test]
fn a_late_answer_is_taken_with_one_pause_per_miss_before_it() {
    let (tries, pauses) = (Cell::new(0u32), Cell::new(0u32));
    let got = find_within(
        100,
        || {
            tries.set(tries.get() + 1);
            (tries.get() == 3).then_some(9u32)
        },
        || pauses.set(pauses.get() + 1),
    );
    assert_eq!((got, tries.get(), pauses.get()), (Some(9), 3, 2));
}

#[test]
fn no_controller_is_given_up_on_after_the_last_try_with_no_pause_after_it() {
    let (tries, pauses) = (Cell::new(0u32), Cell::new(0u32));
    let got: Option<u32> = find_within(
        100,
        || {
            tries.set(tries.get() + 1);
            None
        },
        || pauses.set(pauses.get() + 1),
    );
    assert_eq!((got, tries.get(), pauses.get()), (None, 100, 99));
}

#[test]
fn no_tries_means_no_lookup_and_no_pause() {
    let got: Option<u32> = find_within(0, || panic!("looked up"), || panic!("paused"));
    assert_eq!(got, None);
}
