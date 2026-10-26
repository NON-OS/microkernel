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

//! must_send against the test it replaced, the one pending.rs made inline:
//! `held > 0 && (start.checked_add(held) != Some(at) || held == cap)`.

use super::pending_rule::must_send;

fn old(start: u64, held: u64, at: u64, cap: u64) -> bool {
    held > 0 && (start.checked_add(held) != Some(at) || held == cap)
}

#[test]
fn must_send_agrees_with_the_old_test_over_the_grid() {
    let starts = [0, 1, 7, 100, u64::MAX - 70, u64::MAX - 1, u64::MAX];
    let mut seen = [0u32; 2];
    for &start in &starts {
        for held in 0..=64u64 {
            let next = start.wrapping_add(held);
            for at in [0, 1, start, next, next.wrapping_add(1), u64::MAX] {
                let (new, was) = (must_send(start, held, at, 64), old(start, held, at, 64));
                assert_eq!(new, was, "start {start} held {held} at {at}");
                seen[usize::from(new)] += 1;
            }
        }
    }
    assert!(seen[0] > 0 && seen[1] > 0, "the grid reaches both answers: {seen:?}");
}

#[test]
fn a_run_that_would_pass_the_last_lba_never_grows() {
    assert!(must_send(u64::MAX, 1, 0, 64));
    assert!(must_send(u64::MAX - 1, 2, 0, 64));
    assert!(!must_send(u64::MAX - 2, 1, u64::MAX - 1, 64));
}

#[test]
fn an_empty_run_takes_any_sector_and_a_full_one_none() {
    assert!(!must_send(5, 0, 900, 64));
    assert!(must_send(5, 64, 69, 64));
    assert!(!must_send(5, 63, 68, 64));
}
