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

//! The SMP timer, and a caller or random source that gets things wrong.

use crate::pairing_refuse_tests::fresh;
use crate::sim::{Rng, Stuck, A, B};
use nonos_bt_smp::pairing::{Initiator, Outcome, Step, REQUEST, TIMEOUT_MS};
use nonos_bt_smp::pdu::*;
use nonos_bt_smp::SmpError;

#[test]
fn thirty_silent_seconds_end_pairing_and_nothing_is_sent_after() {
    let mut i = fresh();
    assert_eq!(i.poll(TIMEOUT_MS - 1), Step::default());
    let response = frame(PAIRING_RESPONSE, &REQUEST.params()).unwrap();
    let step = i.on_pdu(&response, &mut Rng(1), TIMEOUT_MS);
    assert!(step.send.is_empty());
    let reason = UNSPECIFIED_REASON;
    assert_eq!(step.outcome, Some(Outcome::Failed { reason, by_peer: false }));
    assert_eq!(i.on_pdu(&response, &mut Rng(1), TIMEOUT_MS + 1), Step::default());
}

#[test]
fn caller_mistakes_and_a_broken_random_source_are_refused() {
    let mut i = fresh();
    assert_eq!(i.confirm(true, 1).err(), Some(SmpError::Refused));
    assert_eq!(i.encrypted(1).err(), Some(SmpError::Refused));
    assert_eq!(i.passkey(Some(1), &mut Rng(1), 1).err(), Some(SmpError::Refused));
    let legacy = Pairing { auth_req: auth::BONDING, ..REQUEST };
    assert_eq!(Initiator::start(legacy, A, B, &mut Rng(1), 0).err(), Some(SmpError::Refused));
    for b in [0x00, 0xff] {
        assert_eq!(
            Initiator::start(REQUEST, A, B, &mut Stuck(b), 0).err(),
            Some(SmpError::BadSecret)
        );
    }
}
