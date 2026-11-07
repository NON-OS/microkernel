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

//! The wait after wrong entries, and the count across a restart.

use nonos_lock::backoff::{wait_ms, BASE_MS, FREE_TRIES, MAX_WAIT_MS};
use nonos_lock::Gate;

#[test]
fn three_free_tries_then_doubling_to_the_cap() {
    assert_eq!(FREE_TRIES, 3);
    for n in 0..=3 {
        assert_eq!(wait_ms(n), 0, "{n}");
    }
    assert_eq!(wait_ms(4), BASE_MS);
    assert_eq!(wait_ms(5), 2 * BASE_MS);
    assert_eq!(wait_ms(6), 4 * BASE_MS);
    assert_eq!(wait_ms(12), 256 * BASE_MS);
    assert_eq!(wait_ms(13), MAX_WAIT_MS);
    assert_eq!(wait_ms(u32::MAX), MAX_WAIT_MS);
    let mut last = 0;
    for n in 0..200 {
        assert!(wait_ms(n) >= last);
        last = wait_ms(n);
    }
}

#[test]
fn each_wrong_entry_past_the_free_ones_holds_the_next() {
    let mut g = Gate::new(0, 1_000);
    for _ in 0..3 {
        assert_eq!(g.wait_left(1_000), 0);
        g.failed(1_000);
    }
    assert_eq!(g.wait_left(1_000), 0);
    g.failed(1_000);
    assert_eq!(g.failures(), 4);
    assert_eq!(g.wait_left(1_000), BASE_MS);
    assert_eq!(g.wait_left(1_000 + 1_999), 1);
    assert_eq!(g.wait_left(1_000 + 2_000), 0);
    g.failed(3_000);
    assert_eq!(g.wait_left(3_000), 2 * BASE_MS);
}

#[test]
fn a_restart_waits_the_whole_wait_again() {
    // Six wrong entries were kept; the machine restarted at uptime 500 ms.
    let g = Gate::new(6, 500);
    assert_eq!(g.failures(), 6);
    assert_eq!(g.wait_left(500), wait_ms(6));
    assert_eq!(g.wait_left(500 + wait_ms(6) as i64), 0);
    // A fresh boot with no count waits for nothing.
    assert_eq!(Gate::new(0, 500).wait_left(500), 0);
}

#[test]
fn the_right_entry_clears_the_count() {
    let mut g = Gate::new(9, 0);
    g.passed();
    assert_eq!(g.failures(), 0);
    assert_eq!(g.wait_left(0), 0);
}

#[test]
fn the_count_does_not_wrap() {
    let mut g = Gate::new(u32::MAX, i64::MAX - 5);
    g.failed(i64::MAX - 5);
    assert_eq!(g.failures(), u32::MAX);
    assert!(g.wait_left(i64::MAX - 5) > 0);
}
