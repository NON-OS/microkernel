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

//! One file a family writes may fill its app's home and no more: a synced
//! wallet's cache passed the old 8 MiB and its save failed with nothing said
//! (docs/release-evidence/0.9.3/s1-g-kvm). The bound is the home quota
//! itself, and passing it is said once a run.

use crate::calls::loadavg::declared::HOME;

const TABLE: &str = include_str!("../../../../../capsule_linux/src/linux/file/held/cache/table.rs");
const TOLD: &str =
    include_str!("../../../../../capsule_linux/src/linux/file/held/cache/too_large.rs");

#[test]
fn one_file_may_fill_its_home() {
    assert_eq!(HOME, 16 << 20);
    assert!(TABLE.contains("pub const MAX_FILE: usize = HOME as usize;"), "tied to the home");
}

#[test]
fn a_write_past_it_is_said_once_a_run() {
    let change = include_str!("../../../../../capsule_linux/src/linux/file/held/cache/change.rs");
    assert_eq!(change.matches("ok_or_else(too_large)").count(), 2, "put and set_len");
    assert!(TOLD.contains("!TOLD.swap(true, Ordering::Relaxed)") && TOLD.contains("errno::EFBIG"));
}
