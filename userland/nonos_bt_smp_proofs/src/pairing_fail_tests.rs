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

//! A responder that cheats: pairing ends with the
//! reason Vol 3, Part H, 3.5.5 gives, never with an LTK to encrypt with.

use crate::sim::{Peer, Run};
use nonos_bt_smp::ecdh::{DEBUG_PUBLIC_X, DEBUG_PUBLIC_Y};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::{Outcome, State};
use nonos_bt_smp::pdu::*;

/// Pairing failed here with `reason`, which was sent last, and no LTK
/// was ever handed out.
pub fn failed_here(mut run: Run, reason: u8) -> Run {
    assert_eq!(run.go(), Some(Outcome::Failed { reason, by_peer: false }));
    assert_eq!(run.sent.last().unwrap(), &[PAIRING_FAILED, reason]);
    assert!(!run.outcomes.iter().any(|o| matches!(o, Outcome::Encrypt(_))));
    assert_eq!(run.init.state(), State::Failed);
    run
}

pub fn run(io: IoCap, mitm: bool, set: impl FnOnce(&mut Peer)) -> Run {
    let mut peer = Peer::new(io, mitm);
    set(&mut peer);
    Run::new(peer)
}

#[test]
fn a_confirm_the_random_does_not_open_fails() {
    failed_here(
        run(IoCap::NoInputNoOutput, false, |p| p.faults.bad_confirm = true),
        CONFIRM_VALUE_FAILED,
    );
    failed_here(
        run(IoCap::KeyboardOnly, true, |p| p.faults.bad_confirm = true),
        CONFIRM_VALUE_FAILED,
    );
}

#[test]
fn a_wrong_dhkey_check_fails_before_encryption() {
    failed_here(run(IoCap::DisplayYesNo, true, |p| p.faults.bad_check = true), DHKEY_CHECK_FAILED);
}

#[test]
fn the_debug_key_an_off_curve_key_and_our_own_key_are_refused() {
    let mut debug = [0u8; 64];
    debug[..32].copy_from_slice(&le(&DEBUG_PUBLIC_X));
    debug[32..].copy_from_slice(&le(&DEBUG_PUBLIC_Y));
    for key in [debug, [1; 64]] {
        let r = failed_here(
            run(IoCap::NoInputNoOutput, false, |p| p.faults.key = Some(key)),
            DHKEY_CHECK_FAILED,
        );
        assert_eq!(r.sent_with(PAIRING_RANDOM), 0);
    }
    failed_here(run(IoCap::KeyboardOnly, true, |p| p.faults.reflect = true), DHKEY_CHECK_FAILED);
}
