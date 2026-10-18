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

//! E-71: what the code does today — the host distributes its identity (the
//! IRK and its public address) to any bond that asked for identity keys,
//! authenticated or not. Whether it should be withheld from an
//! unauthenticated bond is the owner's open ruling; this records the
//! present behaviour unchanged, so the ruling has a before to move.

use crate::pairing_tests::paired;
use crate::sim::{Peer, Run};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::{Identity, REQUEST};
use nonos_bt_smp::pdu::*;

const ME: Identity = Identity { irk: *b"this-hosts-irk!!", addr_type: 0, addr: [1, 2, 3, 4, 5, 6] };
const OFFER: Pairing = Pairing { init_keys: keys::ID, ..REQUEST };

/// A responder that asks this host for its identity key.
fn asks(io: IoCap, mitm: bool) -> Peer {
    let mut peer = Peer::new(io, mitm);
    peer.ours.init_keys = keys::ID;
    peer
}

#[test]
fn the_host_sends_its_identity_whether_or_not_the_bond_is_authenticated() {
    // Just Works: the bond is not authenticated, yet the IRK and identity
    // address still go out.
    let mut jw = Run::with(asks(IoCap::NoInputNoOutput, false), OFFER, Some(ME));
    let bond = paired(&mut jw);
    assert!(!bond.authenticated, "Just Works is unauthenticated");
    assert_eq!(jw.sent_with(IDENTITY_INFORMATION), 1, "IRK sent to an unauthenticated bond");
    assert_eq!(jw.sent_with(IDENTITY_ADDRESS_INFORMATION), 1);
    // Numeric comparison: the bond is authenticated, and the same identity
    // goes out — distribution does not turn on the authentication level.
    let mut nc = Run::with(asks(IoCap::DisplayYesNo, true), OFFER, Some(ME));
    let bond = paired(&mut nc);
    assert!(bond.authenticated, "numeric comparison is authenticated");
    assert_eq!(nc.sent_with(IDENTITY_INFORMATION), 1);
    assert_eq!(nc.sent_with(IDENTITY_ADDRESS_INFORMATION), 1);
}
