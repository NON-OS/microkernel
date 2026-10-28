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

//! Fast downloads, D: a window of GETs holds few replies and bounded bytes
//! (install/get/window_room.rs).

#[path = "../../../capsule_linux/src/linux/install/get/window_room.rs"]
mod window_room;
use window_room::{cap, room};

/// A heap with far more free than any window holds.
const ROOMY: usize = 4 << 30;

#[test]
fn three_small_replies_at_once_and_never_a_fourth() {
    let sizes = [1 << 20; 5];
    assert!(room(&sizes, &[], 0, ROOMY));
    assert!(room(&sizes, &[0, 1], 2, ROOMY));
    assert!(!room(&sizes, &[0, 1, 2], 3, ROOMY));
}

#[test]
fn a_large_package_is_fetched_alone_and_bytes_stay_bounded() {
    let sizes = [80 << 20, 1 << 20, 40 << 20, 30 << 20];
    assert!(room(&sizes, &[], 0, ROOMY), "alone, however large");
    assert!(!room(&sizes, &[0], 1, ROOMY), "nothing beside it");
    assert!(!room(&sizes, &[1, 2], 3, ROOMY), "past 64 MiB together");
    assert!(room(&sizes, &[1], 2, ROOMY));
    assert!(!room(&[0, 1 << 20], &[1], 0, ROOMY), "a size the index lacks counts as the most");
}

#[test]
fn a_heap_short_of_room_takes_one_package_at_a_time() {
    // Monero's 24.7 MB beside 160 MiB of which 72 MiB is in use: its unpack
    // and pack need the rest, so nothing comes beside it.
    let sizes = [24_698_520, 1 << 20];
    assert!(room(&sizes, &[], 0, 88 << 20), "alone, always");
    assert!(!room(&sizes, &[0], 1, 88 << 20), "a sixth of 88 MiB is under 25 MB");
    assert!(room(&sizes, &[0], 1, 320 << 20), "a roomy heap takes both");
}

#[test]
fn a_reply_may_bring_its_size_and_headers_and_no_more() {
    assert_eq!(cap(1000), 1000 + (16 << 10));
    assert_eq!(cap(0), 160 << 20);
    assert_eq!(cap(usize::MAX), 160 << 20);
}
