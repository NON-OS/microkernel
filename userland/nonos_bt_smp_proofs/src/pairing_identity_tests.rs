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

//! This host's own identity in key distribution (Vol 3, Part H, 3.6.1):
//! sent after the responder's keys, and only when the response asks.

use crate::pairing_tests::paired;
use crate::sim::{Peer, Run, A, B};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::{Identity, Initiator, Outcome, REQUEST};
use nonos_bt_smp::pdu::*;
use nonos_bt_smp::SmpError;

const ME: Identity = Identity { irk: *b"this-hosts-irk!!", addr_type: 0, addr: [1, 2, 3, 4, 5, 6] };
const OFFER: Pairing = Pairing { init_keys: keys::ID, ..REQUEST };

fn asked() -> Peer {
    let mut peer = Peer::new(IoCap::NoInputNoOutput, false);
    peer.ours.init_keys = keys::ID;
    peer
}

#[test]
fn ours_follow_the_responders_in_order() {
    let mut run = Run::with(asked(), OFFER, Some(ME));
    paired(&mut run);
    let n = run.sent.len();
    assert_eq!(run.sent[n - 2], [&[IDENTITY_INFORMATION][..], &le(&ME.irk)].concat());
    assert_eq!(run.sent[n - 1], [IDENTITY_ADDRESS_INFORMATION, 0, 1, 2, 3, 4, 5, 6]);
    assert_eq!(run.sent_with(IDENTITY_INFORMATION), 1);
}

#[test]
fn a_response_that_does_not_ask_gets_none() {
    let mut run = Run::with(Peer::new(IoCap::NoInputNoOutput, false), OFFER, Some(ME));
    paired(&mut run);
    assert_eq!(run.sent_with(IDENTITY_INFORMATION), 0);
    assert_eq!(run.sent_with(IDENTITY_ADDRESS_INFORMATION), 0);
}

#[test]
fn a_responder_with_no_keys_of_its_own_gets_ours_at_encryption() {
    let mut peer = asked();
    peer.ours.resp_keys = 0;
    let mut run = Run::with(peer, OFFER, Some(ME));
    let Some(Outcome::Paired(_)) = run.go() else { panic!("{:?}", run.outcomes) };
    assert_eq!(run.sent_with(IDENTITY_ADDRESS_INFORMATION), 1);
}

#[test]
fn offering_keys_this_host_cannot_send_is_refused() {
    let mut rng = crate::sim::Rng(7);
    let mut start = |ours, local| Initiator::start_private(ours, local, A, B, &mut rng, 0).err();
    assert_eq!(start(OFFER, None), Some(SmpError::Refused));
    assert_eq!(start(REQUEST, Some(ME)), Some(SmpError::Refused));
    for extra in [keys::ENC, keys::SIGN] {
        let ours = Pairing { init_keys: keys::ID | extra, ..REQUEST };
        assert_eq!(start(ours, Some(ME)), Some(SmpError::Refused));
    }
}
