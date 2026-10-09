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

//! The clock without a system time: no reading is not midnight, Apply with no
//! date is refused in words rather than sent and dropped, and every answer the
//! kernel gives a time correction is shown.

use crate::clock_civil::to_unix_ms;
use crate::clock_says::{adjust_outcome, wall_ms, DONE, NOT_SET, NO_DATE};

/// `CLOCK_FLOOR_MS` in the kernel's sys_time_adjust: 2025-01-01.
const KERNEL_FLOOR_MS: u64 = 1_735_689_600_000;

#[test]
fn a_clock_not_yet_set_gives_no_reading() {
    // sys_time_millis answers -61 until the clock is ready.
    assert_eq!(wall_ms(-61), None);
    assert_eq!(wall_ms(-1), None);
    assert_eq!(wall_ms(1_800_000_000_000), Some(1_800_000_000_000));
}

#[test]
fn apply_with_no_reading_would_send_a_time_the_kernel_refuses() {
    // With no reading the date is all zeros; the time built on it falls below
    // the kernel's floor, so it was refused and the refusal never shown.
    assert!(to_unix_ms(0, 0, 0, 12, 30, 0) < KERNEL_FLOOR_MS);
    assert!(NO_DATE.starts_with(b"Not set"));
}

#[test]
fn every_answer_to_a_time_correction_is_shown() {
    assert_eq!(adjust_outcome(0), DONE);
    assert_eq!(adjust_outcome(-22), b"Not set: the time must fall between 2025 and 2100");
    assert_eq!(adjust_outcome(-1), b"Not set: this app may not change the clock");
    assert_eq!(adjust_outcome(-13), b"Not set: this app may not change the clock");
    assert_eq!(adjust_outcome(-5), b"Not set: the clock refused the change");
    for rc in [-22, -13, -5, -1] {
        assert_ne!(adjust_outcome(rc), DONE);
    }
}

#[test]
fn the_clock_tab_names_a_missing_time() {
    assert_eq!(NOT_SET, b"The system clock is not set");
}
