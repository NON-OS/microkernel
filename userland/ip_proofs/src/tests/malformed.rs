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

//! A malformed frame is dropped and the poll goes on to the next.

use crate::link::{arrive, frame, fresh, poll, LOCAL, REMOTE};

const UDP: u8 = 17;
const DATAGRAM: [u8; 8] = [0x30, 0x39, 0, 53, 0, 8, 0, 0];

/*
 * A frame with a bad IPv4 header (checksum, version, length) or one too
 * short to hold the Ethernet header ended the poll with E_BAD_PACKET, as if
 * the link had failed, and the good datagram queued behind it waited for the
 * next poll; a steady trickle of junk from the network held every reader of
 * net.ip to one datagram per poll at best.
 */
#[test]
fn a_bad_frame_is_dropped_and_the_next_one_delivered() {
    let bad: [fn() -> Vec<u8>; 4] = [
        || {
            let mut f = frame(REMOTE, LOCAL, UDP, &DATAGRAM);
            f[14 + 10] ^= 0xFF;
            f
        },
        || {
            let mut f = frame(REMOTE, LOCAL, UDP, &DATAGRAM);
            f[14] = 0x65;
            f
        },
        || {
            let mut f = frame(REMOTE, LOCAL, UDP, &DATAGRAM);
            f.truncate(14 + 10);
            f
        },
        || vec![0xFF; 9],
    ];
    for (i, make) in bad.into_iter().enumerate() {
        let _g = fresh();
        arrive(make());
        arrive(frame(REMOTE, LOCAL, UDP, &DATAGRAM));
        let (errno, got) = poll(UDP);
        assert_eq!(errno, 0, "case {i}: the poll did not fail");
        assert!(got.is_some(), "case {i}: the good datagram behind it was delivered");
    }
}

#[test]
fn a_fragment_is_dropped() {
    let _g = fresh();
    let mut f = frame(REMOTE, LOCAL, UDP, &DATAGRAM);
    f[14 + 6] = 0x20; // more fragments
    let ip = &mut f[14..34];
    ip[10] = 0;
    ip[11] = 0;
    let ck = crate::link::checksum(ip);
    ip[10..12].copy_from_slice(&ck.to_be_bytes());
    arrive(f);
    assert!(poll(UDP).1.is_none(), "this stack does not reassemble, and says so");
}
