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

use std::cell::Cell;

use super::rule::{run, Ended, Poll, TOTAL_MS};

/// A clock that moves a millisecond a read and fails the test if a run
/// outlives every budget (a missing bound otherwise hangs).
fn clock(t: &Cell<u64>) -> impl FnMut() -> Option<u64> + '_ {
    move || {
        t.set(t.get() + 1);
        assert!(t.get() < 100_000, "the leave phase outlived every budget");
        Some(t.get())
    }
}

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
fn legs_are_waited_on_together_not_one_after_another() {
    // Each answers 4 s after it is first asked: four in turn would be 16 s,
    // past the total. Every leg is asked in the first round.
    let t = Cell::new(0);
    let first: [Cell<Option<u64>>; 4] = Default::default();
    let ended = run(
        &[5_000; 4],
        TOTAL_MS,
        clock(&t),
        |i| {
            let asked = *first[i].get().get_or_insert(t.get());
            first[i].set(Some(asked));
            if t.get() >= asked + 4_000 {
                Poll::Answered(0)
            } else {
                Poll::Waiting
            }
        },
        || {},
    );
    assert_eq!(ended, [Ended::Answered(0); 4]);
    assert!(first.iter().all(|f| f.get() == Some(2)), "every disk asked in the first round");
}

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

#[test]
fn no_disk_is_no_wait() {
    let t = Cell::new(0);
    assert!(run(&[], TOTAL_MS, clock(&t), |_| Poll::Waiting, || {}).is_empty());
    assert!(t.get() <= 2);
}
