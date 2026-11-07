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

//! The policy store's settings read a step a pass: never in a pass that
//! routed input, three steps a round, a round every ROUND_MS, and a round
//! in which the store did not answer ended at once, so a slow store holds
//! the router's loop for one call's timeout at most, not for every read.

use crate::state::policy_turn::{PolicyTurn, Step, ROUND_MS};

#[test]
fn a_pass_that_routed_input_reads_nothing() {
    let turn = PolicyTurn::new();
    assert_eq!(turn.take(0, true), None);
    assert_eq!(turn.take(0, false), Some(Step::Sensitivity));
}

#[test]
fn a_round_reads_each_setting_once_a_pass_then_waits_a_round() {
    let mut turn = PolicyTurn::new();
    let mut read = Vec::new();
    for now in 0..6 {
        if let Some(step) = turn.take(now, false) {
            read.push(step);
            turn.read(now, true);
        }
    }
    assert_eq!(read, vec![Step::Sensitivity, Step::ScreenTimeout, Step::AutoLock]);
    assert_eq!(turn.take(2 + ROUND_MS - 1, false), None, "not before the round is due");
    assert_eq!(turn.take(2 + ROUND_MS, false), Some(Step::Sensitivity));
}

#[test]
fn a_store_that_does_not_answer_ends_the_round_at_once() {
    let mut turn = PolicyTurn::new();
    assert_eq!(turn.take(10, false), Some(Step::Sensitivity));
    turn.read(10, false);
    assert_eq!(turn.take(11, false), None, "no second call to a store that did not answer");
    assert_eq!(turn.take(10 + ROUND_MS, false), Some(Step::Sensitivity), "asked again next round");
}

#[test]
fn reads_put_off_by_input_come_in_the_first_quiet_pass() {
    let mut turn = PolicyTurn::new();
    turn.read(0, true);
    for now in 1..500 {
        assert_eq!(turn.take(now, true), None);
    }
    assert_eq!(turn.take(500, false), Some(Step::ScreenTimeout), "the round goes on where it was");
}
