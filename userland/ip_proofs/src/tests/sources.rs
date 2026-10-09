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

//! Which sources a datagram off the wire can have (RFC 1122 3.2.1.3).

use crate::link::{arrive, frame, fresh, poll, LOCAL, REMOTE};

const UDP: u8 = 17;

/*
 * No host has a broadcast, multicast, unspecified or loopback address, and
 * none but this one has this host's address, so a datagram claiming one of
 * them as its source is forged or misrouted. RFC 1122 3.2.1.3 has a host
 * discard it; they were all delivered, to be answered at that address.
 */
#[test]
fn datagrams_from_addresses_no_host_has_are_dropped() {
    let _g = fresh();
    for src in [[255, 255, 255, 255], [224, 0, 0, 251], [239, 1, 2, 3], [0, 0, 0, 0], [127, 0, 0, 1], LOCAL] {
        arrive(frame(src, LOCAL, UDP, &[0x30, 0x39, 0, 53, 0, 8, 0, 0]));
    }
    let (_, got) = poll(UDP);
    assert!(got.is_none(), "delivered from {:?}", got.map(|g| g.0));
}

#[test]
fn a_datagram_from_a_neighbour_is_delivered() {
    let _g = fresh();
    arrive(frame(REMOTE, LOCAL, UDP, &[0x30, 0x39, 0, 53, 0, 8, 0, 0]));
    let (errno, got) = poll(UDP);
    assert_eq!(errno, 0);
    let (src, dst, payload) = got.expect("delivered");
    assert_eq!((src, dst, payload.len()), (REMOTE, LOCAL, 8));
}
