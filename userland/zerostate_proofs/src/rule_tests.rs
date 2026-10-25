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

//! The keep rule alone: only an installed disk's store takes a write.

use crate::keep_rule::{disk_of, may_write, Disk};

#[test]
fn only_an_installed_disk_takes_a_write() {
    assert!(may_write(Disk::Installed));
    assert!(!may_write(Disk::Live));
    assert!(!may_write(Disk::Unread));
}

#[test]
fn each_plan_reading_names_its_disk() {
    assert_eq!(disk_of(Some(true)), Disk::Installed);
    assert_eq!(disk_of(Some(false)), Disk::Live);
    assert_eq!(disk_of(None), Disk::Unread);
}

/* A fault reading the plan must never be the way the store opens. */
#[test]
fn every_reading_but_a_volume_plan_keeps_the_store_shut() {
    for reading in [Some(false), None] {
        assert!(!may_write(disk_of(reading)), "{reading:?}");
    }
    assert!(may_write(disk_of(Some(true))));
}
