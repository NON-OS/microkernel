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

//! Three hops of layering, outbound and inbound.

extern crate alloc;

use super::onion_fixture::{chain, message, HOPS};
use crate::cell::{body, put_digest, unpack};
use crate::circuit::{open, seal};

#[test]
fn outbound_layers_peel_in_order_and_the_digest_lands() {
    let mut client = chain();
    let mut relays = chain();
    let text = b"GET /cdn-cgi/trace HTTP/1.0\r\n\r\n";
    let mut payload = message(2, 1, text);
    seal(&mut client, HOPS - 1, &mut payload).expect("target hop exists");

    // Each relay in turn removes exactly one layer.
    for hop in relays.iter_mut() {
        hop.forward.apply(&mut payload[..]);
    }

    let header = unpack(&payload);
    assert_eq!(header.recognized, 0, "the exit did not recognise its own cell");
    assert_eq!(header.command, 2);
    assert_eq!(header.stream, 1);
    assert_eq!(body(&payload).expect("length fits"), text);
}

#[test]
fn an_inbound_cell_is_attributed_to_the_hop_that_sent_it() {
    for origin in 0..HOPS {
        let mut client = chain();
        let mut relays = chain();
        let mut payload = message(2, 7, b"answer");

        relays[origin].backward_digest.update(&payload[..]);
        let digest = relays[origin].backward_digest.peek();
        put_digest(&mut payload, &digest);
        for index in (0..=origin).rev() {
            relays[index].backward.apply(&mut payload[..]);
        }

        let opened = open(&mut client, &mut payload).expect("a hop should claim it");
        assert_eq!(opened.hop, origin, "cell attributed to the wrong hop");
        assert_eq!(body(&payload).expect("length fits"), b"answer");
    }
}
