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

//! Whose process a reset's waits sleep: the named caller, only while it is
//! the process running; anyone else, or no one named, and the waits spin.

use super::reset_wait::sleeper;

#[test]
fn the_caller_in_its_own_syscall_sleeps() {
    assert_eq!(sleeper(Some(7), Some(7)), Some(7));
}

#[test]
fn a_named_caller_that_is_not_running_is_not_put_to_sleep() {
    assert_eq!(sleeper(Some(7), Some(9)), None, "another process is on this CPU");
    assert_eq!(sleeper(Some(7), None), None, "no process is running");
}

#[test]
fn exit_teardown_names_no_one_and_spins() {
    assert_eq!(sleeper(None, Some(7)), None);
    assert_eq!(sleeper(None, None), None);
}
