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

//! The shutdown wait: a controller that never finishes costs the budget and
//! no more, and a clock that cannot be read bounds nothing and says so.

use std::cell::Cell;

use super::OCCURRING;
use crate::admin::{await_shutdown, SHUTDOWN_CAP_MS};
use crate::error::NvmeError;

#[test]
fn a_controller_that_never_finishes_ends_at_its_budget() {
    // A millisecond a clock read; the guard turns a missing bound into a
    // failure instead of a hang.
    let (now, reads) = (Cell::new(0u64), Cell::new(0u32));
    let clock = || {
        now.set(now.get() + 1);
        assert!(now.get() < 50_000, "the wait outlived every budget");
        Some(now.get())
    };
    let csts = || {
        reads.set(reads.get() + 1);
        OCCURRING | 1
    };
    assert_eq!(await_shutdown(SHUTDOWN_CAP_MS, csts, clock), Err(NvmeError::ShutdownTimeout));
    assert!(now.get() >= SHUTDOWN_CAP_MS && now.get() <= SHUTDOWN_CAP_MS + 2, "at the budget");
    assert!(reads.get() as u64 <= SHUTDOWN_CAP_MS + 1);
}

#[test]
fn an_unreadable_clock_bounds_nothing_and_says_so() {
    assert_eq!(await_shutdown(10, || OCCURRING, || None), Err(NvmeError::ClockFailed));
}
