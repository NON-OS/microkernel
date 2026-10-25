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

//! The directory's wait after a sweep that reached no authority: it starts
//! at a second, doubles, stops at half a minute, and is said only as it grows.

use crate::cold_wait_rule::{seconds, worth_saying, FIRST_SECONDS, MOST_SECONDS};

#[test]
fn the_wait_starts_at_a_second_and_doubles_to_half_a_minute() {
    let waits: Vec<u64> = (1..=8).map(seconds).collect();
    assert_eq!(waits, [1, 2, 4, 8, 16, 30, 30, 30]);
    assert_eq!(seconds(1), FIRST_SECONDS);
}

#[test]
fn no_count_of_empty_sweeps_waits_past_the_ceiling_or_overflows() {
    for empty in (0..=200).chain([u32::MAX - 1, u32::MAX]) {
        let w = seconds(empty);
        assert!((FIRST_SECONDS..=MOST_SECONDS).contains(&w), "{empty}: {w}");
        assert!(seconds(empty.saturating_add(1)) >= w, "{empty}: the wait shrank");
    }
}

/* A boot with no network at all says so six times in its first minute and
 * then not again, however long it waits. */
#[test]
fn the_line_is_said_only_while_the_wait_grows() {
    let said: Vec<u32> = (1..=10_000).filter(|&n| worth_saying(n)).collect();
    assert_eq!(said, [1, 2, 3, 4, 5, 6]);
    let elapsed: u64 = (1..=6).map(seconds).sum();
    assert!(elapsed <= 61, "{elapsed}");
}
