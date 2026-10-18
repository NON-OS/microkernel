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

//! The display log's line for each driver the lookup finds: one when a
//! driver is found anew, none while the same one is found again or none is
//! found, and one again for a driver that takes over.

use core::sync::atomic::AtomicUsize;

use crate::discover_order::news;
use crate::displays::DISPLAYS;

#[test]
fn each_driver_found_anew_is_named_once() {
    let said = AtomicUsize::new(usize::MAX);
    let bga = DISPLAYS.iter().position(|&d| d == "driver.bga0").expect("bga listed");
    assert_eq!(news(&said, None), None, "none found: still waiting, nothing said");
    assert_eq!(news(&said, Some(bga)), Some("driver.bga0"));
    assert_eq!(news(&said, Some(bga)), None, "the same driver again");
    assert_eq!(news(&said, Some(0)), Some("driver.virtio_gpu0"), "virtio-gpu takes over");
    assert_eq!(news(&said, None), None);
    assert_eq!(news(&said, Some(0)), Some("driver.virtio_gpu0"), "back after a gap");
}

#[test]
fn a_rank_past_the_list_is_never_named() {
    let said = AtomicUsize::new(usize::MAX);
    assert_eq!(news(&said, Some(DISPLAYS.len() + 3)), None);
}
