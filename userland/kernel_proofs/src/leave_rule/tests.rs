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

//! The leave schedule's bounds (rule.rs): a leg ends answered, at its own
//! budget, or at the total, and no disk is no wait.

use std::cell::Cell;

use super::rule::{run, Ended, Poll, TOTAL_MS};
use super::test_clock::clock;

#[test]
fn every_leg_that_answers_ends_answered() {
    let t = Cell::new(0);
    let ended = run(&[5_000, 5_000], TOTAL_MS, clock(&t), |i| Poll::Answered(-(i as i32)), || {});
    assert_eq!(ended, [Ended::Answered(0), Ended::Answered(-1)]);
}

#[test]
fn a_leg_that_never_answers_ends_at_its_own_budget() {
    let t = Cell::new(0);
    let (first_last, second_last) = (Cell::new(0), Cell::new(0));
    let ended = run(
        &[3_000, 12_000],
        TOTAL_MS,
        clock(&t),
        |i| {
            if i == 0 {
                first_last.set(t.get());
                Poll::Waiting
            } else {
                second_last.set(t.get());
                if t.get() >= 5_000 {
                    Poll::Answered(0)
                } else {
                    Poll::Waiting
                }
            }
        },
        || {},
    );
    assert_eq!(ended, [Ended::Missed, Ended::Answered(0)]);
    assert!(first_last.get() <= 3_001, "asked at {} ms, past its 3 s budget", first_last.get());
    assert!(second_last.get() >= 5_000, "the second disk was given up with the first");
}

#[test]
fn the_total_bounds_every_leg_however_long_its_own_budget() {
    let t = Cell::new(0);
    let ended =
        run(&[60_000, 60_000, 60_000, 60_000], TOTAL_MS, clock(&t), |_| Poll::Waiting, || {});
    assert_eq!(ended, [Ended::Missed; 4]);
    assert!(t.get() <= TOTAL_MS + 8, "four disks took {} ms, not the total", t.get());
}

#[test]
fn no_disk_is_no_wait() {
    let t = Cell::new(0);
    assert!(run(&[], TOTAL_MS, clock(&t), |_| Poll::Waiting, || {}).is_empty());
    assert!(t.get() <= 2);
}
