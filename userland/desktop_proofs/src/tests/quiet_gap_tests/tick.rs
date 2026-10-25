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

//! An answer, and the tick's own clock.

use crate::quiet_gap::QuietGap;
use crate::shell_tick::{tick_due, TICK_MS};

#[test]
fn an_answer_resets_the_gap() {
    let mut gap = QuietGap::new();
    gap.missed(0);
    gap.missed(1_000);
    gap.answered();
    assert!(gap.due(1_001));
    gap.missed(2_000);
    assert!(gap.due(2_000 + QuietGap::FIRST_MS), "back to the first gap");
}

#[test]
fn the_tick_runs_once_a_second() {
    assert!(!tick_due(500, 0));
    assert!(tick_due(TICK_MS, 0));
    assert!(tick_due(5_000, 3_999));
}

/* A clock read behind the last tick (a wall clock stepped back by NTP was
 * what the tick used to be timed on) runs the tick instead of stalling it
 * until the clock catches up. */
#[test]
fn a_clock_behind_the_last_tick_does_not_stall_it() {
    assert!(tick_due(1_000, 3_600_000));
    assert!(tick_due(-61, 0), "an error reading still ticks");
}
