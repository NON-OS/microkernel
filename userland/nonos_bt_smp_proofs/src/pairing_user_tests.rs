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

//! A user who says no, gives up, or mistypes: pairing ends, and the side
//! that notices says why.

use crate::pairing_fail_tests::{failed_here, run};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::{Outcome, State};
use nonos_bt_smp::pdu::*;

#[test]
fn a_user_who_sees_different_numbers_ends_pairing() {
    let mut r = run(IoCap::DisplayYesNo, true, |_| ());
    r.agree = Some(false);
    failed_here(r, NUMERIC_COMPARISON_FAILED);
}

#[test]
fn a_cancelled_or_impossible_passkey_ends_pairing() {
    for typed in [None, Some(1_000_000)] {
        let mut r = run(IoCap::DisplayOnly, true, |_| ());
        r.typed = Some(typed);
        let r = failed_here(r, PASSKEY_ENTRY_FAILED);
        assert_eq!(r.sent_with(PAIRING_CONFIRM), 0);
    }
}

#[test]
fn a_mistyped_passkey_is_caught_by_the_responder() {
    let mut r = run(IoCap::DisplayOnly, true, |_| ());
    r.typed = Some(Some(r.peer.passkey ^ 1));
    let reason = CONFIRM_VALUE_FAILED;
    assert_eq!(r.go(), Some(Outcome::Failed { reason, by_peer: true }));
    assert_eq!(r.init.state(), State::Failed);
}
