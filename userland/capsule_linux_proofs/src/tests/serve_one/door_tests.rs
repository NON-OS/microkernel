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

//! P.6: OPEN's door end is an ordinary loopback client's. It is queued once
//! on the guest's listener, which then polls readable, and one accept takes
//! it; no listener refuses it, and a full queue is EAGAIN.

use super::{listening, AT, GUEST};
use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED};
use crate::linux::net::sock::Socks;

const POLLIN: u16 = 0x001;

#[test]
fn open_queues_one_end_the_guest_accepts_and_the_listener_polls_readable() {
    let (mut t, l) = listening(8);
    assert_eq!(t.bits(l).unwrap() & POLLIN, 0);
    let door = t.open_door(AT).unwrap();
    assert_eq!(t.get(l).unwrap().pending.len(), 1, "one end queued");
    assert_ne!(t.bits(l).unwrap() & POLLIN, 0, "the listener polls readable");
    let (end, _) = t.accept(l, GUEST).unwrap();
    assert_eq!(t.accept(l, GUEST).map(|_| ()), Err(EAGAIN), "and gives one accept");
    assert_eq!(t.bits(l).unwrap() & POLLIN, 0);
    let guest = t.get(end).unwrap();
    assert_eq!((guest.peer, guest.holders.clone()), (Some(door), vec![GUEST]));
    let from = guest.remote.unwrap();
    assert_eq!(from.ip, [127, 0, 0, 1], "a loopback client");
    assert!(from.port >= 32_768, "on an ephemeral port: {}", from.port);
    assert!(t.get(door).unwrap().holders.is_empty(), "no guest holds the door end");
}

#[test]
fn open_with_no_listener_is_refused_and_with_a_full_queue_is_eagain() {
    let mut none = Socks::new();
    assert_eq!(none.open_door(AT), Err(ECONNREFUSED));
    assert_eq!(none.iter().count(), 0, "the refused door end is gone");
    let (mut t, _) = listening(0);
    t.open_door(AT).unwrap();
    assert_eq!(t.open_door(AT), Err(EAGAIN), "a backlog of 0 queues one");
    assert_eq!(t.iter().count(), 3, "the listener, the door end and its server end");
}
