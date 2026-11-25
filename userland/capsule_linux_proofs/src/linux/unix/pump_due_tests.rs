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

//! When the display's inbox is read: when the wait said a message came or
//! timed out, else once EVERY_MS has passed since the last read.

use super::pump_due::{due, EVERY_MS};

#[test]
fn told_reads_at_once() {
    assert!(due(true, 100, 100));
}

#[test]
fn untold_waits_for_the_interval() {
    assert!(!due(false, 100, 100));
    assert!(!due(false, 100 + EVERY_MS - 1, 100));
    assert!(due(false, 100 + EVERY_MS, 100));
}

#[test]
fn a_clock_behind_the_last_read_does_not_read() {
    assert!(!due(false, 50, 100));
}
