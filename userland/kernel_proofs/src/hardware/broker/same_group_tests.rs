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

//! A device's holder is its process: the thread that claimed it, the
//! group leader and any other thread of the group all hold it; a thread of
//! another process does not, nor does pid 0.

use super::acpi::same_group::{group, same_holder};

/// Process 100 with threads 205 and 206; process 300 alone; 400 a pid
/// whose group reads 0; 900 no process at all.
fn table(pid: u32) -> Option<u32> {
    match pid {
        100 | 205 | 206 => Some(100),
        300 => Some(300),
        400 => Some(0),
        _ => None,
    }
}

#[test]
fn every_thread_of_the_holding_process_holds() {
    for (holder, asker) in [(100, 205), (205, 100), (205, 206), (206, 206), (100, 100)] {
        assert!(same_holder(holder, asker, table), "{holder} held, {asker} asks");
    }
}

#[test]
fn no_other_process_and_no_pid_zero() {
    for (holder, asker) in [(205, 300), (300, 206), (100, 400), (100, 900), (205, 0)] {
        assert!(!same_holder(holder, asker, table), "{holder} held, {asker} asks");
    }
    assert!(same_holder(400, 400, table), "a group of 0 stands for itself");
    assert!(same_holder(900, 900, table), "a pid naming no process stands for itself");
    assert_eq!((group(206, table), group(400, table), group(900, table)), (100, 400, 900));
}
