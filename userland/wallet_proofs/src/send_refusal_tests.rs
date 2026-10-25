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

//! What a node's refusal of a broadcast is said as: each known refusal its
//! own sentence, one already held as on its way, anything else as unnamed.

use nonos_eth_rpc::{broadcast_error, NodeSaid, UNNAMED};

fn node_error(message: &str) -> Vec<u8> {
    format!(r#"{{"jsonrpc":"2.0","id":5,"error":{{"code":-32000,"message":"{message}"}}}}"#)
        .into_bytes()
}

#[test]
fn a_node_that_already_holds_the_transaction_has_it_on_its_way() {
    assert_eq!(broadcast_error(&node_error("already known")), NodeSaid::AlreadyKnown);
    assert_eq!(broadcast_error(&node_error("Known transaction: 0xab")), NodeSaid::AlreadyKnown);
}

#[test]
fn each_refusal_a_node_gives_is_its_own_sentence() {
    let errors = [
        "insufficient funds for gas * price + value",
        "nonce too low: next nonce 7, tx nonce 6",
        "nonce too high",
        "replacement transaction underpriced",
        "max fee per gas less than block base fee",
        "intrinsic gas too low",
    ];
    let said: Vec<&str> = errors
        .iter()
        .map(|e| match broadcast_error(&node_error(e)) {
            NodeSaid::Refused(why) => why,
            NodeSaid::AlreadyKnown => panic!("{e} is a refusal"),
        })
        .collect();
    for (i, a) in said.iter().enumerate() {
        assert_ne!(*a, UNNAMED, "{}", errors[i]);
        for b in &said[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn a_refusal_the_wallet_does_not_know_is_said_as_that() {
    let unknown = broadcast_error(&node_error("the moon is in the wrong phase"));
    assert_eq!(unknown, NodeSaid::Refused(UNNAMED));
    assert_eq!(broadcast_error(b"not json at all"), unknown);
}
