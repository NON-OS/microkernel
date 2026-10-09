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

//! Which echo requests are answered.

use crate::link::{arrive, checksum, echo_request, frame, fresh, poll, sent, LOCAL, REMOTE};

const ICMP: u8 = 1;
const UDP: u8 = 17;

#[test]
fn an_echo_request_to_us_is_answered_with_its_data() {
    let _g = fresh();
    arrive(frame(REMOTE, LOCAL, ICMP, &echo_request(7, 1, b"ping data")));
    let _ = poll(UDP);
    let out = sent();
    assert_eq!(out.len(), 1, "one echo reply");
    let ip = &out[0][14..];
    assert_eq!((&ip[12..16], &ip[16..20]), (&LOCAL[..], &REMOTE[..]));
    let icmp = &ip[20..];
    assert_eq!((icmp[0], icmp[1]), (0, 0), "an echo reply");
    assert_eq!(&icmp[4..8], &[0, 7, 0, 1], "identifier and sequence echoed");
    assert_eq!(&icmp[8..], b"ping data");
    assert_eq!(checksum(icmp), 0, "the reply's checksum verifies");
}

/*
 * net.ip accepts datagrams sent to 255.255.255.255. An echo request there was
 * answered, so one forged request to the broadcast address drew a reply from
 * every host on the segment at whatever source it named: a smurf amplifier.
 * RFC 1122 3.2.2.6 lets a host stay silent, and every current stack does.
 */
#[test]
fn an_echo_request_to_the_broadcast_address_is_not_answered() {
    let _g = fresh();
    arrive(frame(REMOTE, [255, 255, 255, 255], ICMP, &echo_request(1, 1, b"x")));
    let _ = poll(UDP);
    assert!(sent().is_empty(), "no reply to a broadcast echo request");
}

#[test]
fn an_echo_request_from_a_group_or_broadcast_source_is_not_answered() {
    let _g = fresh();
    for src in [[224, 0, 0, 1], [255, 255, 255, 255], [0, 0, 0, 0]] {
        arrive(frame(src, LOCAL, ICMP, &echo_request(1, 1, b"x")));
    }
    let _ = poll(UDP);
    assert!(sent().is_empty(), "a reply would go to a whole group or nowhere");
}

#[test]
fn an_echo_request_with_a_bad_checksum_is_not_answered() {
    let _g = fresh();
    let mut m = echo_request(1, 1, b"x");
    m[8] ^= 0xFF;
    arrive(frame(REMOTE, LOCAL, ICMP, &m));
    let _ = poll(UDP);
    assert!(sent().is_empty());
}
