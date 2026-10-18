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

//! The lease question the Wi-Fi and Network pages ask on the window's thread:
//! at most once a second, and never a long wait, so a DHCP client busy
//! getting an address does not hold the window.

use crate::net_poll::{net_poll_due, LEASE_TIMEOUT_MS, NET_POLL_MS};

#[test]
fn the_first_look_asks_at_once() {
    assert!(net_poll_due(None, 0));
    assert!(net_poll_due(None, 123_456));
}

/* A join ticks the page every 100 ms; the lease is asked on one tick in ten. */
#[test]
fn ticks_100_ms_apart_ask_once_a_second() {
    let mut last = None;
    let mut asked = 0;
    for tick in 0..30i64 {
        let now = 5_000 + tick * 100;
        if net_poll_due(last, now) {
            last = Some(now);
            asked += 1;
        }
    }
    assert_eq!(asked, 3);
}

#[test]
fn a_clock_behind_the_last_question_asks_again() {
    assert!(net_poll_due(Some(10_000), 9_000));
}

/* The old wait was 2.5 s on every tick; one question may not hold the window
 * for more than a quarter of the gap between questions. */
const _: () = assert!(LEASE_TIMEOUT_MS as i64 * 4 <= NET_POLL_MS);
