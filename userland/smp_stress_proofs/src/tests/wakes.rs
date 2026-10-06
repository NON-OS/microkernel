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

//! What counts as a lost or late wake, and when a run passes.

use crate::stall::{is_late, is_stall, verdict, Counts, LATE_MS, STALL_MS, WAIT_TIMEOUT_MS};

#[test]
fn a_stall_is_a_wait_of_a_second_which_a_timed_out_wait_always_is() {
    assert!(!is_stall(STALL_MS - 1));
    assert!(is_stall(STALL_MS));
    assert!(is_stall(WAIT_TIMEOUT_MS));
}

#[test]
fn a_sleep_is_late_only_past_its_deadline_and_the_margin() {
    assert!(!is_late(7, 7 + LATE_MS - 1));
    assert!(is_late(7, 7 + LATE_MS));
    assert!(!is_late(u64::MAX, u64::MAX));
}

#[test]
fn a_pass_needs_every_kind_of_work_and_no_fault() {
    let good = Counts { futex: 1, ipc: 1, sleeps: 1, ..Counts::default() };
    assert!(verdict(&good));
    assert!(!verdict(&Counts { ipc: 0, ..good }));
    assert!(!verdict(&Counts { stalls: 1, ..good }));
    assert!(!verdict(&Counts { late: 1, ..good }));
    assert!(!verdict(&Counts { errors: 1, ..good }));
}
