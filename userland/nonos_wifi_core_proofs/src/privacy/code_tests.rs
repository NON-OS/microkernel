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

//! Row 1.8: a failed roam reports its own code, never "wrong password",
//! and bars the candidate for the boot: a second look does not try it.

use super::msg3_tests::last_roam;
use crate::roam::two_aps::{run_until, sent, two_aps};
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::softmac::control::result::CODE_WRONG_PASSWORD;
use nonos_wifi_core::softmac::control::roam_code::*;
use nonos_wifi_core::softmac::join::{Counters, Outcome, Report};

fn report(accepted: bool, failure: MlmeFailure) -> Report {
    Report {
        outcome: Outcome::Refused,
        counters: Counters::default(),
        state: 5,
        failure: Some(failure),
        akm: None,
        used_pmksa: false,
        pmksa: None,
        selection: None,
        accepted,
        group_mgmt: None,
        transition_disable: None,
    }
}

#[test]
fn a_candidates_bad_confirm_is_a_refusal_not_a_wrong_password() {
    let bad = MlmeFailure::Sae(SaeFailure::BadConfirm);
    assert_eq!(roam_code(&report(false, bad)), CODE_CANDIDATE_REFUSED);
    assert_ne!(roam_code(&report(false, bad)), CODE_WRONG_PASSWORD);
    let left = MlmeFailure::Left(15);
    assert_eq!(roam_code(&report(true, left)), CODE_AP_UNPROVEN, "accepted, then unproven");
}

#[test]
fn a_refusing_candidate_is_reported_and_never_tried_again() {
    let mut st = two_aps(|b| b.refuse = true, [-85, -40]);
    assert!(run_until(&mut st, |s| !sent(s, 0x20).is_empty()), "tried once");
    assert_eq!(last_roam(&mut st), CODE_CANDIDATE_REFUSED);
    let looks = sent(&st, 0x48).len();
    run_until(&mut st, |_| false);
    run_until(&mut st, |_| false);
    assert!(sent(&st, 0x48).len() > looks, "it looked again");
    assert_eq!(sent(&st, 0x20).len(), 1, "but never tried the barred AP again");
}

#[test]
fn a_candidate_that_fails_the_handshake_is_unproven() {
    let mut st = two_aps(|b| b.pass = b"another password".to_vec(), [-85, -40]);
    assert!(run_until(&mut st, |s| !sent(s, 0x20).is_empty() && s.session().is_none()));
    assert_eq!(last_roam(&mut st), CODE_AP_UNPROVEN);
}
