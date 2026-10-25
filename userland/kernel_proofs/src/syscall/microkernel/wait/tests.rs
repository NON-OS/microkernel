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

use super::rule::{after_empty, Next, NO_WAIT};

#[test]
fn no_wait_looks_once_however_long_it_has_been() {
    assert_eq!(NO_WAIT, u64::MAX);
    assert_eq!(after_empty(NO_WAIT, 1_000, 0), Next::Again);
    assert_eq!(after_empty(NO_WAIT, 1_000, 5_000), Next::Again);
}

#[test]
fn zero_still_waits_for_ever() {
    assert_eq!(after_empty(0, 1_000, 0), Next::SleepUntil(u64::MAX));
    assert_eq!(after_empty(0, 1_000, u64::MAX - 1), Next::SleepUntil(u64::MAX));
}

#[test]
fn a_timeout_sleeps_to_its_deadline_then_runs_out() {
    assert_eq!(after_empty(250, 1_000, 0), Next::SleepUntil(1_250));
    assert_eq!(after_empty(250, 1_000, 249), Next::SleepUntil(1_250));
    assert_eq!(after_empty(250, 1_000, 250), Next::TimedOut);
    assert_eq!(after_empty(u64::MAX - 1, u64::MAX - 5, 0), Next::SleepUntil(u64::MAX));
}
