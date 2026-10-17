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

//! The gate against a controller driver that is gone, and the answer that
//! opens it again.

use crate::i2c_client::gate::rule::UNANSWERED_LIMIT;
use crate::i2c_client::gate::Gate;

const TIMED_OUT: i64 = -110;

#[test]
fn a_controller_driver_that_never_answers_is_asked_a_bounded_number_of_times() {
    // One call every 2 ms for ten minutes against a server that is gone: the
    // gate lets through a few calls per pause, not three hundred thousand.
    let mut g = Gate::default();
    let mut calls = 0u32;
    let mut now = 0u64;
    while now < 600_000 {
        if g.open(now) {
            calls += 1;
            g.record(TIMED_OUT, now);
        }
        now += 2;
    }
    assert!(calls <= 3 * 26, "{calls} calls reached the kernel");
}

#[test]
fn the_first_answer_opens_the_gate_and_forgets_the_pauses() {
    let mut g = Gate::default();
    for _ in 0..UNANSWERED_LIMIT * 3 {
        g.record(TIMED_OUT, 0);
    }
    assert!(g.pauses > 0);
    g.record(48, 10);
    assert_eq!(g, Gate::default());
    assert!(g.open(10));
}

#[test]
fn an_answer_between_misses_keeps_the_gate_open() {
    let mut g = Gate::default();
    for _ in 0..10 {
        g.record(TIMED_OUT, 0);
        g.record(TIMED_OUT, 0);
        g.record(32, 0);
    }
    assert!(g.open(0), "two misses in a row is under the limit");
}
