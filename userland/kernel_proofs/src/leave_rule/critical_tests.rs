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

//! The critical leave (a thermal zone or the battery past its grace,
//! zerostate terminate_critical): the same schedule within CRITICAL_TOTAL_MS,
//! so a disk that never answers holds the machine up 6 s, not 15.

use std::cell::Cell;

use super::rule::{run, Ended, Poll, CRITICAL_TOTAL_MS, TOTAL_MS};

#[test]
fn a_critical_leave_ends_every_leg_by_its_shorter_total() {
    let t = Cell::new(0u64);
    let clock = || {
        t.set(t.get() + 1);
        assert!(t.get() < 100_000, "the leave phase outlived every budget");
        Some(t.get())
    };
    let ended = run(&[14_500, 12_000], CRITICAL_TOTAL_MS, clock, |_| Poll::Waiting, || {});
    assert_eq!(ended, [Ended::Missed, Ended::Missed]);
    assert!(t.get() <= CRITICAL_TOTAL_MS + 2, "{} ms, past the critical total", t.get());
}

// Shorter than the ordinary total, and still room for a drive's typical
// normal shutdown and a working disk's flush.
const _: () = assert!(CRITICAL_TOTAL_MS < TOTAL_MS && CRITICAL_TOTAL_MS >= 5_000);
