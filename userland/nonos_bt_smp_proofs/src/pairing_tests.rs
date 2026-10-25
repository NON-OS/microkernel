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

//! Whole pairings against the simulated responder, one per method: both
//! sides must reach the same LTK, and the bond must carry the responder's
//! identity.

use crate::sim::{Peer, Run};
use nonos_bt_smp::method::{IoCap, Method};
use nonos_bt_smp::pairing::{Bond, Outcome, State, User};
use nonos_bt_smp::pdu::*;

pub fn paired(run: &mut Run) -> Bond {
    let Some(Outcome::Paired(bond)) = run.go() else { panic!("not paired: {:?}", run.outcomes) };
    assert_eq!(run.init.state(), State::Done);
    assert_eq!(bond.ltk, run.peer.ltk);
    assert_ne!(bond.ltk, [0; 16]);
    assert_eq!(run.outcomes[0], Outcome::Encrypt(bond.ltk));
    assert_eq!(bond.irk, Some(run.peer.irk));
    assert_eq!(bond.identity, Some((1, [0x10, 0x20, 0x30, 0x40, 0x50, 0xc0])));
    bond
}

#[test]
fn just_works_with_a_device_that_has_no_io() {
    let mut run = Run::new(Peer::new(IoCap::NoInputNoOutput, false));
    let bond = paired(&mut run);
    assert_eq!(run.init.method(), Method::JustWorks);
    assert!(!bond.authenticated);
    assert!(run.users.is_empty());
    // Request, public key, Na, Ea; with Just Works the initiator never commits.
    assert_eq!(run.sent.len(), 4);
    assert_eq!(run.sent_with(PAIRING_CONFIRM), 0);
}

#[test]
fn numeric_comparison_shows_the_number_the_responder_shows() {
    let mut run = Run::new(Peer::new(IoCap::DisplayYesNo, true));
    let bond = paired(&mut run);
    assert_eq!(run.init.method(), Method::NumericComparison);
    assert_eq!(run.users, [User::ConfirmNumber(run.peer.number())]);
    assert!(run.peer.number() < 1_000_000);
    assert!(bond.authenticated);
}

#[test]
fn a_responder_that_wants_no_identity_keys_pairs_at_encryption() {
    let mut peer = Peer::new(IoCap::NoInputNoOutput, false);
    peer.ours.resp_keys = 0;
    let mut run = Run::new(peer);
    let Some(Outcome::Paired(bond)) = run.go() else { panic!("{:?}", run.outcomes) };
    assert_eq!((bond.irk, bond.identity), (None, None));
}
