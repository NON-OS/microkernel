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

//! When net.core says the bound interface has no lease: once, fifteen seconds
//! after the bind or after the last lease, and never while a lease is held.

use crate::lease_wait::{LeaseWait, NO_LEASE_SAY_MS};

#[test]
fn a_bind_without_a_lease_is_said_once_after_the_wait() {
    let mut w = LeaseWait::new();
    assert!(!w.observe(1_000, 7, false));
    assert!(!w.observe(1_000 + NO_LEASE_SAY_MS - 1, 7, false));
    assert!(w.observe(1_000 + NO_LEASE_SAY_MS, 7, false));
    assert!(!w.observe(1_000 + 10 * NO_LEASE_SAY_MS, 7, false));
}

#[test]
fn a_held_lease_is_never_said() {
    let mut w = LeaseWait::new();
    for t in 0..100 {
        assert!(!w.observe(t * 1_000, 7, true));
    }
}

// A lease dropped (run out, NAK, or reset when the link came back) and not
// taken again is said the same way, counted from when it was last held.
#[test]
fn a_dropped_lease_not_taken_again_is_said_again() {
    let mut w = LeaseWait::new();
    w.observe(0, 7, false);
    assert!(w.observe(NO_LEASE_SAY_MS, 7, false));
    assert!(!w.observe(20_000, 7, true));
    assert!(!w.observe(20_000 + NO_LEASE_SAY_MS - 1, 7, false));
    assert!(w.observe(20_000 + NO_LEASE_SAY_MS, 7, false));
}

#[test]
fn nothing_is_said_before_a_bind() {
    let mut w = LeaseWait::new();
    assert!(!w.observe(10 * NO_LEASE_SAY_MS, 0, false));
}
