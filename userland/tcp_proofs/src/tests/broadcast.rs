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

//! TCP is between two unicast hosts. A segment to a broadcast or multicast
//! address, or from one, opens nothing and is never answered with a reset
//! (RFC 1122 4.2.3.10).

use crate::peer::{fresh, inject, request, sent, Seg, LOCAL, SYN};
use crate::protocol::OP_LISTEN;

const PORT: u16 = 8080;

fn syn(src: [u8; 4], dst: [u8; 4], dport: u16) -> Seg {
    let mut s = Seg::from_peer(dport, 5000, 0, SYN, &[]);
    s.src = src;
    s.dst = dst;
    s
}

fn connections() -> usize {
    crate::state::TABLE.lock().entries_mut().len()
}

/*
 * net.ip delivers segments sent to 255.255.255.255. A SYN there to a closed
 * port drew a reset sent from 255.255.255.255: one forged broadcast made
 * every listening host on the segment answer its victim.
 */
#[test]
fn a_syn_to_the_broadcast_address_draws_no_reset() {
    let _g = fresh();
    inject(syn([93, 184, 216, 34], [255, 255, 255, 255], 9));
    crate::peer::drain();
    assert!(sent().is_empty(), "nothing is sent from or for a broadcast");
}

#[test]
fn a_syn_to_the_broadcast_address_opens_nothing_on_a_listener() {
    let _g = fresh();
    assert_eq!(request(OP_LISTEN, &PORT.to_le_bytes()).0, 0);
    let before = connections();
    inject(syn([93, 184, 216, 34], [255, 255, 255, 255], PORT));
    crate::peer::drain();
    assert_eq!(connections(), before);
    assert!(sent().is_empty());
}

#[test]
fn segments_from_addresses_no_host_has_are_dropped() {
    let _g = fresh();
    assert_eq!(request(OP_LISTEN, &PORT.to_le_bytes()).0, 0);
    let before = connections();
    for src in [[224, 0, 0, 1], [255, 255, 255, 255], [0, 0, 0, 0], [127, 0, 0, 1], LOCAL] {
        inject(syn(src, LOCAL, PORT));
        inject(syn(src, LOCAL, 9));
    }
    crate::peer::drain();
    assert_eq!(connections(), before, "no connection from a non-unicast or forged source");
    assert!(sent().is_empty(), "and no SYN-ACK or reset sent to one");
}

#[test]
fn an_ordinary_syn_is_still_answered() {
    let _g = fresh();
    inject(syn([93, 184, 216, 34], LOCAL, 9));
    crate::peer::drain();
    assert_eq!(sent().len(), 1, "a closed port still refuses a unicast SYN");
}
