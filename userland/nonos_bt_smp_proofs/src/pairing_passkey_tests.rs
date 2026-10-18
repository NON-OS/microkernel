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

//! Passkey entry against the simulated responder, the passkey shown on
//! either side: twenty rounds, one per passkey bit (Vol 3, Part H,
//! 2.3.5.6.3).

use crate::pairing_tests::paired;
use crate::sim::{Peer, Run};
use nonos_bt_smp::method::{IoCap, Method};
use nonos_bt_smp::pairing::{User, ROUNDS};
use nonos_bt_smp::pdu::*;

#[test]
fn passkey_shown_here_runs_twenty_rounds() {
    let mut run = Run::new(Peer::new(IoCap::KeyboardOnly, true));
    let bond = paired(&mut run);
    assert_eq!(run.init.method(), Method::PasskeyInitiatorDisplays);
    let [User::ShowPasskey(k)] = run.users[..] else { panic!("{:?}", run.users) };
    assert!(k < 1_000_000);
    assert_eq!(run.sent_with(PAIRING_CONFIRM), ROUNDS as usize);
    assert_eq!(run.sent_with(PAIRING_RANDOM), ROUNDS as usize);
    assert!(bond.authenticated);
}

#[test]
fn passkey_shown_there_is_typed_here() {
    let mut run = Run::new(Peer::new(IoCap::DisplayOnly, true));
    let bond = paired(&mut run);
    assert_eq!(run.init.method(), Method::PasskeyResponderDisplays);
    assert_eq!(run.users, [User::EnterPasskey]);
    assert_eq!(run.sent_with(PAIRING_CONFIRM), ROUNDS as usize);
    assert!(bond.authenticated);
}
