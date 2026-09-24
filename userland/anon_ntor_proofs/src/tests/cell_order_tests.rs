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
//! Cell order.

use super::onion_fixture::{chain, message, HOPS};
use crate::cell::{put_digest, PAYLOAD_BYTES};
use crate::circuit::{open, Hop};

/*
 * The inbound direction, through `open`, which is the path a cell arriving on the
 * link takes. `digest_chain_tests` pins the same property outbound through
 * `seal`; the two chains are separate state and neither covers the other.
 */

fn from_exit(relays: &mut [Hop], text: &[u8]) -> [u8; PAYLOAD_BYTES] {
    let origin = HOPS - 1;
    let mut payload = message(2, 7, text);
    relays[origin].backward_digest.update(&payload[..]);
    let digest = relays[origin].backward_digest.peek();
    put_digest(&mut payload, &digest);
    for index in (0..=origin).rev() {
        relays[index].backward.apply(&mut payload[..]);
    }
    payload
}

#[test]
fn a_run_of_cells_verifies_in_order() {
    let mut exit = chain();
    let mut client = chain();
    for n in 0..8u8 {
        let mut cell = from_exit(&mut exit, &[n; 16]);
        let opened = open(&mut client, &mut cell).expect("recognised at the exit hop");
        assert_eq!(opened.hop, HOPS - 1, "attributed to the exit that sent it");
    }
}
#[test]
fn one_skipped_cell_breaks_every_cell_after_it() {
    let mut exit = chain();
    let mut client = chain();

    let mut first = from_exit(&mut exit, b"one");
    assert!(open(&mut client, &mut first).is_some(), "the first cell verifies");

    // The exit sends this one and the client never applies it.
    let _skipped = from_exit(&mut exit, b"two");

    let mut third = from_exit(&mut exit, b"three");
    assert!(
        open(&mut client, &mut third).is_none(),
        "with a gap in the chain the next cell cannot be recognised"
    );
}
