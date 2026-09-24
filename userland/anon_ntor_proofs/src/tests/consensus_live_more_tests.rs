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
//! A live consensus, continued.

use super::consensus_live_fixture::{live, weights_of};
use crate::directory::consensus::parse;
use crate::path::{candidates, Position, Relay, Taken, Weights};

#[test]
fn a_middle_survives_the_network_rule_on_a_live_consensus() {
    let Some(body) = live() else { return };
    let doc = parse(&body).expect("parses");
    let relays: Vec<Relay> = doc
        .entries
        .iter()
        .map(|e| Relay {
            address: e.address,
            or_port: e.or_port,
            rsa_identity: e.rsa_identity,
            ed25519_identity: [1u8; 32],
            ntor_onion_key: [1u8; 32],
            flags: e.flags,
            weight: e.weight,
            exits_web: true,
        })
        .collect();
    let w = weights_of(&doc);
    let (open, _) = candidates(&relays, &w, Position::Middle, &[]);
    assert!(!open.is_empty(), "the consensus offers middles at all");

    // Exclude the largest network in the document, which is the worst the rule
    // can do to the pool.
    let worst = busiest_network(&relays);
    let taken: Vec<Taken> = relays
        .iter()
        .filter(|r| r.address[0] == worst[0] && r.address[1] == worst[1])
        .take(1)
        .map(|r| Taken { identity: r.rsa_identity, address: r.address })
        .collect();
    let (left, weights) = candidates(&relays, &w, Position::Middle, &taken);
    assert!(!left.is_empty(), "excluding the busiest /16 still leaves middles");
    assert_eq!(weights.len(), left.len(), "the two vectors stay index aligned");
    assert!(left.len() < open.len(), "and the rule actually removed something");
}

/// The /16 holding the most relays, which is the worst case for the rule.
fn busiest_network(relays: &[Relay]) -> [u8; 2] {
    let mut counts: alloc::collections::BTreeMap<[u8; 2], usize> = Default::default();
    for relay in relays.iter() {
        *counts.entry([relay.address[0], relay.address[1]]).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|(_, n)| *n).map(|(net, _)| net).unwrap_or([0, 0])
}
