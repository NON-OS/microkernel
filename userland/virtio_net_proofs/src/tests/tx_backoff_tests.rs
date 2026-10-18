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

//! A device that keeps stalling is not reset, released,
//! claimed and logged every two seconds for as long as its backend holds
//! the frames. The first stall is reset at once; a stall again with nothing
//! completed waits twice as long as the last, up to the cap, and only a
//! longer wait is logged. A completion between two stalls starts over.

use super::stall_model::stalls;
use crate::constants::{RESET_BACKOFF_MAX_MS, RESET_BACKOFF_MS, TX_STALL_MS};
use crate::server::backoff::{Backoff, Step};

#[test]
fn a_device_that_keeps_stalling_is_reset_ever_more_rarely_and_logged_once_a_step() {
    let resets = stalls(600_000, 0);
    assert_eq!(resets[0], (TX_STALL_MS, true), "the first stall is reset at once");
    let (mut wait, mut steps) = (RESET_BACKOFF_MS, 1);
    for (pair, n) in resets.windows(2).zip(1..) {
        let gap = pair[1].0 - pair[0].0;
        assert!(gap >= wait && gap <= wait.max(TX_STALL_MS) + 200, "reset {n} after {gap} ms");
        assert_eq!(pair[1].1, wait < RESET_BACKOFF_MAX_MS, "reset {n}: logged once a step");
        steps += usize::from(wait < RESET_BACKOFF_MAX_MS);
        wait = (2 * wait).min(RESET_BACKOFF_MAX_MS);
    }
    assert_eq!(resets.iter().filter(|r| r.1).count(), steps);
    assert!(resets.len() > steps + 4, "still reset at the cap: {} resets", resets.len());
}

#[test]
fn a_completion_between_two_stalls_starts_the_count_over() {
    let resets = stalls(120_000, 4);
    let gaps: Vec<u64> = resets.windows(2).map(|p| p[1].0 - p[0].0).collect();
    assert!(gaps[2] >= 4 * RESET_BACKOFF_MS, "the fourth reset waited");
    assert!(resets[4].1, "a new stall is logged");
    assert!(gaps[3] < RESET_BACKOFF_MS + TX_STALL_MS, "the completion did not restart it");
    assert!(gaps[4] >= RESET_BACKOFF_MS && gaps[4] < 2 * RESET_BACKOFF_MS + TX_STALL_MS);
}

#[test]
fn the_backoff_holds_at_the_ends_of_the_clock() {
    let mut b = Backoff::default();
    assert_eq!(b.step(u64::MAX - 1, false), Step::Reset { log: true });
    assert_eq!(b.step(u64::MAX - 1, false), Step::Wait);
    assert_eq!(b.step(u64::MAX, true), Step::Reset { log: true });
}
