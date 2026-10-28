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

use crate::calls::loadavg::declared::{HOME, HOME_MOST};

const TABLE: &str = include_str!("../../../../../capsule_linux/src/linux/file/held/cache/table.rs");
const TOLD: &str =
    include_str!("../../../../../capsule_linux/src/linux/file/held/cache/too_large.rs");

#[test]
fn one_file_may_fill_its_home() {
    assert_eq!((HOME, HOME_MOST), (16 << 20, 64 << 20));
    assert!(TABLE.contains("pub const MAX_FILE: usize = HOME_MOST as usize;"), "the largest home");
}

#[test]
fn a_write_past_it_is_said_once_a_run() {
    let change = include_str!("../../../../../capsule_linux/src/linux/file/held/cache/change.rs");
    assert_eq!(change.matches("ok_or_else(too_large)").count(), 2, "put and set_len");
    assert!(TOLD.contains("!TOLD.swap(true, Ordering::Relaxed)") && TOLD.contains("errno::EFBIG"));
}

#[test]
fn the_home_is_sized_for_its_listing_before_it_opens() {
    let start = include_str!("../../../../../capsule_linux/src/linux/start_home.rs");
    let chose = start.find("file::choose_home(&guest.app.policy.listing_id);").expect("chosen");
    assert!(chose < start.find("super::file::open_home(key)").expect("opened"));
    let room =
        include_str!("../../../../../capsule_linux/src/linux/file/system/space/home_room.rs");
    assert_eq!(room.matches(", most),").count(), 2, "both arms held to the run's home");
    let statfs = include_str!("../../../../../capsule_linux/src/linux/file/system/space/room.rs");
    assert!(statfs.contains("blocks: home_bytes() / BSIZE,"), "statfs says the run's home");
}
