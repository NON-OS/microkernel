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

//! Fast downloads, D: a window of GETs holds one reply at a time, its bytes
//! bounded (install/get/window_room.rs).

#[path = "../../../capsule_linux/src/linux/install/get/window_room.rs"]
mod window_room;
use window_room::{cap, room};

/// A heap with far more free than any window holds.
const ROOMY: usize = 4 << 30;

#[test]
fn one_get_at_a_time_while_a_proxy_keeps_one_stream_per_caller() {
    // A second stream from the installer ends the first at the proxy
    // (nonos_route_link tunnel.rs): three at once brought no package a byte.
    let sizes = [1 << 20; 5];
    assert!(room(&sizes, &[], 0, ROOMY), "the first, alone");
    assert!(!room(&sizes, &[0], 1, ROOMY), "never a second beside it");
    assert!(!room(&sizes, &[0, 1], 2, ROOMY));
}

#[test]
fn a_package_comes_alone_however_large_or_short_the_heap() {
    let sizes = [80 << 20, 1 << 20];
    assert!(room(&sizes, &[], 0, ROOMY), "alone, however large");
    assert!(room(&sizes, &[], 1, 1 << 20), "alone, however little is free");
    assert!(!room(&sizes, &[0], 1, ROOMY), "nothing beside it");
    // Monero's 24.7 MB: a roomy heap takes nothing beside it either.
    assert!(!room(&[24_698_520, 1 << 20], &[0], 1, 320 << 20));
}

#[test]
fn a_reply_may_bring_its_size_and_headers_and_no_more() {
    assert_eq!(cap(1000), 1000 + (16 << 10));
    assert_eq!(cap(0), 160 << 20);
    assert_eq!(cap(usize::MAX), 160 << 20);
}
