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
//! OP_START in both drivers (driver.usb_mic0 mounts indicator.rs too): the
//! indicator shown before any capture begins, no capture when it is not,
//! and the indicator taken down again when capture could not begin.

use std::cell::RefCell;

use crate::indicator::shown_first;

/// What `shown_first` did, in order, given the compositor's answer and
/// the device's.
fn run(shown: bool, begun: Result<u32, i32>) -> (Result<u32, i32>, Vec<&'static str>) {
    let log = RefCell::new(Vec::new());
    let r = shown_first(
        -1,
        || {
            log.borrow_mut().push("show");
            shown
        },
        || {
            log.borrow_mut().push("begin");
            begun
        },
        || log.borrow_mut().push("hide"),
    );
    (r, log.into_inner())
}

#[test]
fn capture_begins_only_after_the_indicator_is_shown() {
    assert_eq!(run(true, Ok(7)), (Ok(7), vec!["show", "begin"]));
}

#[test]
fn no_indicator_no_capture() {
    assert_eq!(run(false, Ok(7)), (Err(-1), vec!["show"]));
}

#[test]
fn a_capture_that_fails_takes_the_indicator_down() {
    assert_eq!(run(true, Err(-19)), (Err(-19), vec!["show", "begin", "hide"]));
}
