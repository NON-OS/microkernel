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


//! The descriptors a recvmsg takes, and those it leaves to be closed.

use crate::display_give::handed;

#[test]
fn a_block_is_handed_over_only_into_room_for_all_of_it() {
    assert!(handed(24, 0x1000, 24), "exactly the room");
    assert!(handed(24, 0x1000, 64));
    assert!(!handed(24, 0x1000, 20), "too little room: closed, MSG_CTRUNC");
    assert!(!handed(24, 0, 64), "no control buffer");
    assert!(!handed(0, 0x1000, 64), "nothing owed");
}

#[test]
fn the_display_is_the_path_whose_last_component_is_wayland_0() {
    use crate::display_path::is_display;
    assert!(is_display(b"/run/user/0/wayland-0"));
    assert!(is_display(b"wayland-0"), "relative, as WAYLAND_DISPLAY names it");
    assert!(!is_display(b"/run/user/0/xwayland-0"), "another socket");
    assert!(!is_display(b"/tmp/my-wayland-0"));
    assert!(!is_display(b"/run/user/0/wayland-0/x"));
}

#[test]
fn a_pollfd_array_is_answered_whole_in_one_pass() {
    use crate::poll_answer::answer;
    let entry = |fd: i32, events: u16| {
        let mut e = fd.to_le_bytes().to_vec();
        e.extend_from_slice(&events.to_le_bytes());
        e.extend_from_slice(&0xFFFFu16.to_le_bytes()); // stale revents the guest left
        e
    };
    // 3 readable, 4 not, -1 switched off, 9 closed (POLLNVAL).
    let mut raw: Vec<u8> = [entry(3, 1), entry(4, 1), entry(-1, 1), entry(9, 1)].concat();
    let hits = answer(&mut raw, |fd| match fd {
        3 => 0x001 | 0x004,
        9 => 0x020,
        _ => 0,
    });
    let revents = |i: usize| u16::from_le_bytes([raw[i * 8 + 6], raw[i * 8 + 7]]);
    assert_eq!(hits, 2);
    assert_eq!(revents(0), 0x001, "only what was asked");
    assert_eq!(revents(1), 0, "nothing ready is written as nothing");
    assert_eq!(revents(2), 0, "switched off");
    assert_eq!(revents(3), 0x020, "closed is said unasked");
}
