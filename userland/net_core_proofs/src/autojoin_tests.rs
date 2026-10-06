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

//! When autojoin acts. The serve loop runs a tick once a second; a join is
//! handed to the driver and watched, never waited for, and each saved network
//! is tried once.

use crate::autojoin_machine::{
    Due, Machine, Step, EMPTY_PASSES_MAX, JOIN_WATCH_MS, NOT_YET_MS, RESCAN_MS, WATCH_EVERY_MS,
    CALL_MS,
};

// Every call autojoin makes from the serve loop waits at most 25 ms.
const _: () = assert!(CALL_MS <= 25);

#[test]
fn not_yet_waits_and_tries_again() {
    let mut m = Machine::new();
    assert_eq!(m.due(0), Due::Attempt);
    m.after_attempt(0, Step::NotYet);
    assert_eq!(m.due(NOT_YET_MS - 1), Due::Nothing);
    assert_eq!(m.due(NOT_YET_MS), Due::Attempt);
}

// A join the driver has not answered within CALL_MS runs on in the driver; the
// loop looks at the link once a second and moves on as soon as it is up.
#[test]
fn a_started_join_is_watched_until_the_link_is_up() {
    let mut m = Machine::new();
    m.after_attempt(0, Step::Started(1));
    assert_eq!(m.tried(), 0b10, "tried once, never replayed");
    assert_eq!(m.due(WATCH_EVERY_MS - 1), Due::Nothing);
    assert_eq!(m.due(WATCH_EVERY_MS), Due::Watch);
    // A driver busy joining does not answer the link in time: keep watching.
    assert_eq!(m.after_watch(WATCH_EVERY_MS, None), None);
    assert_eq!(m.after_watch(2 * WATCH_EVERY_MS, Some(false)), None);
    assert_eq!(m.due(3 * WATCH_EVERY_MS), Due::Watch);
    assert_eq!(m.after_watch(3 * WATCH_EVERY_MS, Some(true)), Some((1, true)));
    assert!(m.done());
    assert_eq!(m.due(i64::MAX), Due::Nothing);
}

#[test]
fn a_join_that_never_comes_up_counts_as_tried_and_the_next_is_due() {
    let mut m = Machine::new();
    m.after_attempt(0, Step::Started(0));
    let mut t = 0;
    while t < JOIN_WATCH_MS {
        t += WATCH_EVERY_MS;
        let ended = m.after_watch(t, Some(false));
        if t < JOIN_WATCH_MS {
            assert_eq!(ended, None);
        } else {
            assert_eq!(ended, Some((0, false)));
        }
    }
    assert!(!m.done());
    assert_eq!(m.tried(), 0b1);
    assert_eq!(m.due(t + NOT_YET_MS), Due::Attempt, "the next saved network gets its pass");
}

#[test]
fn an_answered_join_ends_or_moves_on() {
    let mut m = Machine::new();
    m.after_attempt(0, Step::Answered { index: 2, joined: false });
    assert_eq!(m.tried(), 0b100);
    assert_eq!(m.due(NOT_YET_MS), Due::Attempt);
    m.after_attempt(NOT_YET_MS, Step::Answered { index: 0, joined: true });
    assert!(m.done());
}

#[test]
fn nothing_in_range_rescans_then_gives_up() {
    let mut m = Machine::new();
    let mut t = 0;
    for pass in 1..=EMPTY_PASSES_MAX {
        assert_eq!(m.due(t), Due::Attempt);
        m.after_attempt(t, Step::NoneInRange);
        if pass < EMPTY_PASSES_MAX {
            assert!(!m.done());
            t += RESCAN_MS;
        }
    }
    assert!(m.done());
}

#[test]
fn a_bound_link_is_left_alone() {
    let mut m = Machine::new();
    m.bound(0);
    assert_eq!(m.due(NOT_YET_MS - 1), Due::Nothing);
    assert_eq!(m.due(NOT_YET_MS), Due::Attempt);
    assert_eq!(m.tried(), 0);
}

#[test]
fn a_watch_outside_a_join_changes_nothing() {
    let mut m = Machine::new();
    assert_eq!(m.after_watch(0, Some(true)), None);
    assert!(!m.done());
}
