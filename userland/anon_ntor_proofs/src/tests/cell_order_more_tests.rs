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
//! Cell order, continued.

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
fn cells_applied_out_of_order_are_not_recognised() {
    let mut exit = chain();
    let mut client = chain();
    let first = from_exit(&mut exit, b"one");
    let second = from_exit(&mut exit, b"two");

    let mut later = second;
    assert!(open(&mut client, &mut later).is_none(), "the second cell is not the next one owed");
    let mut earlier = first;
    assert!(
        open(&mut client, &mut earlier).is_none(),
        "and the chain does not recover by going back for the one it missed"
    );
}
