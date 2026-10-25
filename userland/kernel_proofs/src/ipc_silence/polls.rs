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

//! A poll that is meant to miss: net.core asks its NIC driver for 8 ms every
//! 10 ms. Answered in time now and then, it is never reported; answered
//! never, it is reported once the default has passed, then again a window on.

use super::silence::{Pair, Silence};

const W: u64 = 5000;
const POLL: Pair = Pair { caller: 24, port: 4410 };

fn run(answer_every: Option<u64>, polls: u64) -> Vec<u64> {
    let mut s = Silence::new(W);
    let mut reported = Vec::new();
    for i in 0..polls {
        let start = i * 10;
        if answer_every.is_some_and(|k| i % k == 0) {
            s.answered(POLL);
        } else if s.late(POLL, start, start + 8)[0].is_some() {
            reported.push(start + 8);
        }
    }
    reported
}

#[test]
fn a_poll_answered_in_time_now_and_then_is_never_reported() {
    for every in [2, 7, 100, 400] {
        assert_eq!(run(Some(every), 3000), [], "one in {every} answered");
    }
}

#[test]
fn a_poll_never_answered_is_reported_once_a_window() {
    assert_eq!(run(None, 1200), [5008, 10008]);
    // An answer every 6 s leaves silences longer than the window: each is said.
    assert_eq!(run(Some(600), 1200), [5018, 11018]);
}
