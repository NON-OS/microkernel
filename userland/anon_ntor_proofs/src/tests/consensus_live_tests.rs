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
//! Consensus live.

use super::consensus_live_fixture::{live, weights_of};
use crate::directory::consensus::parse;
use crate::path::{candidates, Position, Relay, Taken, Weights};
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

#[test]
fn every_relay_in_a_live_consensus_reads_back() {
    let Some(body) = live() else { return };
    let doc = parse(&body).expect("a live consensus parses");
    assert!(doc.entries.len() > 1000, "a real consensus carries thousands of relays");
    for entry in doc.entries.iter() {
        assert_ne!(entry.address, [0, 0, 0, 0], "every entry has an address");
        assert_ne!(entry.or_port, 0, "every entry has an ORPort");
        assert!(entry.rsa_identity.iter().any(|b| *b != 0), "every entry has an identity");
        assert!(
            entry.microdesc_digest.iter().any(|b| *b != 0),
            "every entry names a microdescriptor, or its ntor key can never be checked"
        );
    }
    let ids: BTreeSet<[u8; 20]> = doc.entries.iter().map(|e| e.rsa_identity).collect();
    assert_eq!(ids.len(), doc.entries.len(), "no identity appears twice");
}
#[test]
fn a_live_consensus_declares_a_window_and_enough_signatures() {
    let Some(body) = live() else { return };
    let doc = parse(&body).expect("parses");
    assert!(doc.valid_after < doc.fresh_until, "fresh-until follows valid-after");
    assert!(doc.fresh_until < doc.valid_until, "valid-until follows fresh-until");
    assert!(doc.signatures.len() >= 4, "fewer than the quorum would never be believed");
    let (from, to) = doc.signed;
    assert!(to > from && to <= body.len(), "the signed span lies inside the document");
}
