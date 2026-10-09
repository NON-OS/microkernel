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

//! A driver's start: an absent device is never tried, a present one is
//! brought up once through the schedule, and each way out has its own code.

use std::cell::Cell;

use crate::bringup_policy::{decide, EXIT_ABSENT, EXIT_GAVE_UP};

#[test]
fn an_absent_device_leaves_at_once_without_a_single_attempt() {
    let said = Cell::new(0u32);
    let tried = Cell::new(0u32);
    let out: Result<(), i32> = decide(
        None::<u64>,
        || said.set(said.get() + 1),
        |_| {
            tried.set(tried.get() + 1);
            Ok(())
        },
    );
    assert_eq!(out, Err(EXIT_ABSENT));
    assert_eq!(said.get(), 1, "absence is said exactly once");
    assert_eq!(tried.get(), 0, "nothing is claimed on a machine without the device");
}

#[test]
fn a_present_device_that_never_comes_up_gives_up_with_its_own_code() {
    let said = Cell::new(false);
    let out: Result<(), i32> = decide(Some(7u64), || said.set(true), |_| Err("reset timeout"));
    assert_eq!(out, Err(EXIT_GAVE_UP));
    assert!(!said.get(), "a present device is not reported absent");
}

#[test]
fn a_present_device_that_comes_up_is_served_with_what_discovery_found() {
    let seen = Cell::new(0u64);
    let out = decide(
        Some(41u64),
        || panic!("present, not absent"),
        |id| {
            seen.set(id);
            Ok(id * 2)
        },
    );
    assert_eq!(out, Ok(82));
    assert_eq!(seen.get(), 41, "bring-up runs against the device discovery returned");
}

#[test]
fn bring_up_is_handed_the_device_exactly_once() {
    let calls = Cell::new(0u32);
    let _ = decide(
        Some(()),
        || {},
        |()| -> Result<(), &'static str> {
            calls.set(calls.get() + 1);
            Err("x")
        },
    );
    // The retries live inside `up` (the bounded schedule), never around it.
    assert_eq!(calls.get(), 1);
}
