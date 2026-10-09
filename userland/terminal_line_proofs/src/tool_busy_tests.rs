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


//! A second `linux` asked for while the first is only being torn down is
//! asked again, not refused (capsule_terminal builtin/tool_busy.rs), and a
//! refusal that stands says what to do.

use crate::tool_busy::{may_be_ending, running_said, ERRNO_EXIST, GAP_MS, RETRY_MS};

#[test]
fn a_linux_refused_as_running_is_asked_again() {
    assert!(may_be_ending(b"linux", ERRNO_EXIST));
}

#[test]
fn nothing_else_is_asked_again() {
    assert!(!may_be_ending(b"linux", -2), "not installed stays not installed");
    assert!(!may_be_ending(b"linux", -13), "a refusal stays a refusal");
    assert!(!may_be_ending(b"tokei", ERRNO_EXIST), "a crates.io tool has no teardown race");
    assert!(!may_be_ending(b"linux", 42), "a pid is a start");
}

#[test]
fn the_wait_is_short_and_asks_often() {
    assert!(RETRY_MS <= 1_000, "a terminal holds its input for at most a second");
    assert!(GAP_MS > 0 && RETRY_MS / GAP_MS >= 20);
}

#[test]
fn a_running_linux_says_one_runs_at_a_time_and_how_to_end_it() {
    let said = core::str::from_utf8(running_said(b"linux")).unwrap();
    assert!(said.contains("one runs at a time") && said.contains("Ctrl-C"), "{said}");
    assert_eq!(running_said(b"tokei"), b": one is already running");
}
