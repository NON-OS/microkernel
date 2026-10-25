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

//! Legs that end without an answer (rule.rs): a driver gone ended at once,
//! and a clock that cannot be read named on every leg still running.

use std::cell::Cell;

use super::rule::{run, Ended, Poll, TOTAL_MS};
use super::test_clock::clock;

#[test]
fn a_gone_driver_ends_at_once_and_is_not_asked_again() {
    let t = Cell::new(0);
    let asked = Cell::new(0);
    let ended = run(
        &[5_000, 5_000],
        TOTAL_MS,
        clock(&t),
        |i| {
            if i == 0 {
                asked.set(asked.get() + 1);
                Poll::Gone
            } else if t.get() > 100 {
                Poll::Answered(0)
            } else {
                Poll::Waiting
            }
        },
        || {},
    );
    assert_eq!(ended, [Ended::Gone, Ended::Answered(0)]);
    assert_eq!(asked.get(), 1);
}

#[test]
fn an_unreadable_clock_ends_every_leg_still_running_named() {
    let reads = Cell::new(0);
    let now = || {
        reads.set(reads.get() + 1);
        (reads.get() < 3).then_some(reads.get())
    };
    let ended = run(
        &[5_000, 5_000],
        TOTAL_MS,
        now,
        |i| if i == 0 { Poll::Answered(0) } else { Poll::Waiting },
        || {},
    );
    assert_eq!(ended, [Ended::Answered(0), Ended::NoClock]);
    assert_eq!(run(&[1], TOTAL_MS, || None, |_| Poll::Answered(0), || {}), [Ended::NoClock]);
}
