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

//! `until` and `hold`, the clock every wait in the driver runs on. Each
//! step's refusal is `until` returning false, so these carry the paths a
//! concurrent model cannot hold a bit for: queue enables that never read
//! back, and a semaphore another agent never releases.

use std::cell::Cell;
use std::time::Duration;

use crate::init::wait::{hold, until};

use super::model::timed;

#[test]
fn a_condition_already_true_returns_at_once() {
    let (ok, took) = timed(|| until(1000, || true));
    assert!(ok);
    assert!(took < Duration::from_millis(500));
}

#[test]
fn a_condition_never_true_fails_only_after_the_bound() {
    for ms in [1u64, 20, 100] {
        let (ok, took) = timed(|| until(ms, || false));
        assert!(!ok);
        assert!(took >= Duration::from_millis(ms), "{ms} ms bound, gave up after {took:?}");
    }
}

#[test]
fn a_condition_that_turns_true_late_is_seen() {
    let asked = Cell::new(0u32);
    let ok = until(1000, || {
        asked.set(asked.get() + 1);
        asked.get() >= 5
    });
    assert!(ok);
    assert_eq!(asked.get(), 5, "no further look once it held");
}

#[test]
fn the_last_look_after_the_deadline_still_counts() {
    let asked = Cell::new(0u32);
    let ok = until(0, || {
        asked.set(asked.get() + 1);
        asked.get() == 2
    });
    assert!(ok, "a part that answered during the last sleep is not called silent");
}

#[test]
fn hold_waits_at_least_its_milliseconds() {
    for ms in [0u64, 1, 10] {
        let (_, took) = timed(|| hold(ms));
        assert!(took >= Duration::from_millis(ms));
    }
}
