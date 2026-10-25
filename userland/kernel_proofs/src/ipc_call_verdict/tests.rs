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


use super::verdict::{no_answer, stale_reply, wait_ms, NoAnswer, DEFAULT_MS};

const D: u64 = 5000;

#[test]
fn a_poll_whose_own_short_deadline_ran_out_is_late() {
    // net.core's 8 ms NIC poll under TCG.
    assert_eq!(no_answer(true, 8, D, 9), NoAnswer::Late);
    assert_eq!(no_answer(true, 24, D, 31), NoAnswer::Late);
}

#[test]
fn a_call_that_waited_the_default_is_starved() {
    assert_eq!(no_answer(true, 0, D, D), NoAnswer::Starved);
    assert_eq!(no_answer(true, 0, D, D + 3), NoAnswer::Starved);
    assert_eq!(no_answer(true, 10_000, D, 10_000), NoAnswer::Starved);
}

#[test]
fn equality_with_the_default_is_starved() {
    assert_eq!(no_answer(true, D, D, D), NoAnswer::Starved);
    assert_eq!(no_answer(true, D - 1, D, D - 1), NoAnswer::Late);
    // A short caller scheduled late enough to reach the default.
    assert_eq!(no_answer(true, D - 1, D, D), NoAnswer::Starved);
    assert_eq!(no_answer(true, D, D, D - 1), NoAnswer::Starved);
}

#[test]
fn an_early_failure_without_a_short_deadline_is_not_late() {
    assert_eq!(no_answer(true, 0, D, 30), NoAnswer::Starved);
    assert_eq!(no_answer(true, 10_000, D, 30), NoAnswer::Starved);
}

#[test]
fn a_failure_that_did_not_time_out_is_never_late() {
    // No inbox, or a reply buffer the copy faulted on, under a short bound.
    assert_eq!(no_answer(false, 8, D, 0), NoAnswer::Starved);
    assert_eq!(no_answer(false, 24, D, 3), NoAnswer::Starved);
}

#[test]
fn the_bound_is_the_callers_or_the_default() {
    assert_eq!(DEFAULT_MS, 5000);
    assert_eq!(wait_ms(0, D), D);
    assert_eq!(wait_ms(8, D), 8);
    assert_eq!(wait_ms(10_000, D), 10_000);
}

#[test]
fn only_a_nonzero_stale_token_counts_as_a_late_reply() {
    assert!(stale_reply(6, 7));
    assert!(!stale_reply(0, 7), "a forgery stamped 0 is not a late reply");
    assert!(!stale_reply(7, 7));
}
