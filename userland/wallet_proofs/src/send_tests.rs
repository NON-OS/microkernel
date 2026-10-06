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

//! A payment's gas limit and fee, and what its broadcast came to: a plain
//! transfer at exactly its cost, a call with its margin and never wrapping,
//! no signing on a missing, zero or absurd fee, and a broadcast whose answer
//! was lost said as unknown and followed under its own hash, never as one
//! to send again.

use crate::wallet::rpc::{broadcast_error, NodeSaid};
use crate::wallet::send::gas::{fee_refusal, gas_limit, FEE_CEILING_WEI, PLAIN_GAS};
use crate::wallet::send::outcome::{judge, Broadcast, Carried, REFUSED};

const LOCAL: [u8; 32] = [0x11; 32];
const OTHER: [u8; 32] = [0x22; 32];

#[test]
fn a_plain_transfer_is_signed_at_exactly_its_cost() {
    assert_eq!(gas_limit(21_000, true), PLAIN_GAS);
    assert_eq!(gas_limit(20_000, true), PLAIN_GAS, "an estimate under the cost is not trusted");
}

#[test]
fn a_call_carries_its_margin() {
    assert_eq!(gas_limit(50_000, false), 60_000);
    assert_eq!(gas_limit(21_000, false), 25_200, "token data is a call whatever its estimate");
    /* A plain transfer the node says costs more, to a contract, gets the margin. */
    assert_eq!(gas_limit(30_000, true), 36_000);
}

#[test]
fn a_huge_estimate_saturates_instead_of_wrapping() {
    assert_eq!(gas_limit(u64::MAX, false), u64::MAX);
    assert!(gas_limit(u64::MAX / 2, false) >= u64::MAX / 2);
}

#[test]
fn no_payment_is_signed_on_a_fee_that_is_missing_zero_or_absurd() {
    assert!(fee_refusal(None).is_some());
    assert!(fee_refusal(Some(0)).is_some());
    assert!(fee_refusal(Some(FEE_CEILING_WEI + 1)).is_some());
    assert_eq!(fee_refusal(Some(FEE_CEILING_WEI)), None);
    assert_eq!(fee_refusal(Some(3_000_000_000)), None);
}

#[test]
fn a_taken_broadcast_is_followed_under_its_own_hash() {
    let b = judge(Carried::Answer(Some(LOCAL)), &LOCAL);
    assert_eq!(b, Broadcast::Sent(LOCAL));
    assert_eq!(b.follow(), Some(LOCAL));
    /* A node that names another hash cannot point the wallet elsewhere. */
    assert_eq!(judge(Carried::Answer(Some(OTHER)), &LOCAL).follow(), Some(LOCAL));
}

#[test]
fn a_broadcast_whose_answer_was_lost_is_unknown_and_followed_not_resent() {
    let b = judge(Carried::Unknown, &LOCAL);
    assert_eq!(b, Broadcast::Unknown(LOCAL));
    assert_eq!(b.follow(), Some(LOCAL), "its receipt is followed as a sent one's is");
    assert!(!matches!(b, Broadcast::NotSent(_)), "never read as one that did not go");
}

#[test]
fn a_refusal_and_a_route_that_carried_nothing_follow_nothing() {
    assert_eq!(judge(Carried::Answer(None), &LOCAL), Broadcast::Refused(REFUSED));
    assert_eq!(judge(Carried::Answer(None), &LOCAL).follow(), None);
    let funds = "not enough";
    assert_eq!(judge(Carried::Refused(funds), &LOCAL), Broadcast::Refused(funds));
    assert_eq!(judge(Carried::Refused(funds), &LOCAL).follow(), None);
    let why = "the Nym mixnet is not connected yet";
    assert_eq!(judge(Carried::NotSent(why), &LOCAL), Broadcast::NotSent(why));
    assert_eq!(judge(Carried::NotSent(why), &LOCAL).follow(), None);
}

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
        assert_ne!(*a, crate::wallet::rpc::broadcast_error::UNNAMED, "{}", errors[i]);
        for b in &said[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn a_refusal_the_wallet_does_not_know_is_said_as_that() {
    let unknown = broadcast_error(&node_error("the moon is in the wrong phase"));
    assert_eq!(unknown, NodeSaid::Refused(crate::wallet::rpc::broadcast_error::UNNAMED));
    assert_eq!(broadcast_error(b"not json at all"), unknown);
}
