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

//! The warm circuit: drawn only while the network is up and something else
//! serves the rest, never past the cap; taken by installs only; gone at its
//! first failure or after ten minutes.

use crate::protocol::{CIRCUIT_DIRTY_SECONDS, CIRCUIT_FAILURES_MAX, CIRCUIT_MAX};
use crate::warm_rule::{charged, draw_warm, failures_max, may_take, spent, WARM_FAILURES_MAX};

#[test]
fn it_is_drawn_only_while_the_network_is_up_with_room_and_none_held() {
    assert!(draw_warm(true, 1, false, 1));
    assert!(!draw_warm(false, 1, false, 1), "a network that is not up draws nothing");
    assert!(!draw_warm(true, 1, true, 1), "one warm circuit at a time");
    assert!(!draw_warm(true, 0, false, 0), "browsing gets the first circuit");
    for general in 0..=CIRCUIT_MAX + 2 {
        for others in 0..=general {
            let drawn = draw_warm(true, general, false, others);
            assert!(!drawn || general < CIRCUIT_MAX, "{general}: past the cap");
        }
    }
}

#[test]
fn only_an_install_takes_the_warm_circuit() {
    assert!(may_take(true, true));
    assert!(!may_take(false, true), "no other stream goes down it");
    assert!(may_take(true, false) && may_take(false, false), "a general one is for all");
}

#[test]
fn it_goes_at_its_first_failure_where_a_general_one_gets_two() {
    assert_eq!((WARM_FAILURES_MAX, failures_max(true)), (1, 1));
    assert_eq!(failures_max(false), CIRCUIT_FAILURES_MAX);
    assert!(!spent(true, 0, 100, 100) && spent(true, 1, 100, 100));
    assert!(!spent(false, 1, 100, 100) && spent(false, 2, 100, 100));
    assert!(charged(true, true) && charged(true, false), "every failure counts on it");
    assert!(!charged(false, true) && charged(false, false));
}

#[test]
fn it_takes_streams_for_ten_minutes_from_opening_and_no_longer() {
    assert_eq!(CIRCUIT_DIRTY_SECONDS, 600);
    let opened = 1_000;
    assert!(!spent(true, 0, opened, opened + 599));
    assert!(spent(true, 0, opened, opened + 600));
    assert!(spent(true, 0, u64::MAX - 1, u64::MAX), "an opening near the end of time");
}
