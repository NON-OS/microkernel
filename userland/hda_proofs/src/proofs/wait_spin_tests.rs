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

//! A wait answered at once is not slept on.
//!
//! The clock counts whole milliseconds, so a wait begun late in one reads a
//! millisecond in after a few microseconds. The wait used to spin only while
//! it read under one, so a part that answered just past that tick still cost
//! a sleep, a whole scheduler tick, for an answer a microsecond away. It now
//! spins while the clock reads under two, at least a whole millisecond, and
//! sleeps between polls only after that. The clock is set by hand here, so
//! where the tick lands is the test's, not the scheduler's.

use crate::clock::{set_ms, sleeps};
use crate::controller::verb::VERB_MS;
use crate::controller::wait::until;

/// The sleeps a wait takes on a part that answers at its second poll, the
/// clock having moved `ticks` milliseconds on from where the wait began.
fn sleeps_answered_after(ticks: u64) -> u64 {
    set_ms(1_000);
    let (before, mut polls) = (sleeps(), 0);
    let answered = until(VERB_MS, || {
        polls += 1;
        set_ms(1_000 + ticks);
        polls > 1
    });
    assert!(answered, "the wait gave up on a part that answered");
    sleeps() - before
}

#[test]
fn a_part_that_answers_just_past_the_first_tick_is_not_slept_on() {
    assert_eq!(sleeps_answered_after(1), 0, "slept on an answer microseconds away");
}

#[test]
fn a_part_still_silent_two_ticks_in_is_slept_on_between_polls() {
    assert_eq!(sleeps_answered_after(2), 1, "spun a whole millisecond past the first");
}
