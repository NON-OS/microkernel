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

//! Responses this host will not pair with, PDUs out of place,
//! and identity keys in the wrong order.

use crate::sim::{Peer, Rng, Run, A, B};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::{Initiator, Outcome, State, Step, REQUEST};
use nonos_bt_smp::pdu::*;

fn refused(set: impl FnOnce(&mut Peer), reason: u8) {
    let mut peer = Peer::new(IoCap::KeyboardDisplay, true);
    set(&mut peer);
    let mut run = Run::new(peer);
    assert_eq!(run.go(), Some(Outcome::Failed { reason, by_peer: false }));
    // Nothing of ours, not even the public key, follows the request.
    assert_eq!(run.sent, [run.sent[0].clone(), vec![PAIRING_FAILED, reason]]);
}

#[test]
fn legacy_only_short_keys_and_oob_are_refused() {
    refused(|p| p.ours.auth_req &= !auth::SC, AUTHENTICATION_REQUIREMENTS);
    refused(|p| p.ours.max_key_size = 15, ENCRYPTION_KEY_SIZE);
    refused(|p| p.ours.oob = true, OOB_NOT_AVAILABLE);
}

pub fn fresh() -> Initiator {
    Initiator::start(REQUEST, A, B, &mut Rng(7), 0).unwrap().0
}

fn sends(step: Step, reason: u8) {
    assert_eq!(step.send, [vec![PAIRING_FAILED, reason]]);
}

#[test]
fn pdus_out_of_place_unknown_or_malformed_end_pairing() {
    sends(fresh().on_pdu(&[PAIRING_DHKEY_CHECK; 17], &mut Rng(1), 1), UNSPECIFIED_REASON);
    sends(fresh().on_pdu(&[0x0f], &mut Rng(1), 1), COMMAND_NOT_SUPPORTED);
    sends(fresh().on_pdu(&[PAIRING_RESPONSE, 3, 0], &mut Rng(1), 1), INVALID_PARAMETERS);
    let mut i = fresh();
    assert_eq!(i.on_pdu(&[SECURITY_REQUEST, 0x0d], &mut Rng(1), 1), Step::default());
    assert_eq!(i.state(), State::AwaitResponse);
}

#[test]
fn identity_keys_out_of_order_end_pairing() {
    let mut peer = Peer::new(IoCap::NoInputNoOutput, false);
    peer.faults.keys_reversed = true;
    let mut run = Run::new(peer);
    let reason = UNSPECIFIED_REASON;
    assert_eq!(run.go(), Some(Outcome::Failed { reason, by_peer: false }));
}
