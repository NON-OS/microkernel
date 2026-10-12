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

//! How a pass's outcome moves autojoin on, and what leaves it alone.

use crate::autojoin_machine::{Due, Machine, Step, EMPTY_PASSES_MAX, NOT_YET_MS, RESCAN_MS};

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
