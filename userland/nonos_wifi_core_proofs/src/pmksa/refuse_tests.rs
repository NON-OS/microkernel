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

//! A PMKSA is offered only where it applies. One for another AP leaves the
//! join to SAE. A network the policy joins with PSK never sends a PMKID.
//! And an AP that refuses the association of a cached join fails it with
//! its status, which the soft-MAC answers by running SAE again.

use super::sim::{by_sae, offered};
use crate::ap_sim::STA_RSNE_PSK;
use crate::mlme::frames::{assoc_elements, assoc_resp, open_auth_ok};
use crate::mlme::sim::{beacon, join, AP, OTHER, STA};
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::mlme::{MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::softmac::join::{retry_with_sae, Counters, Outcome, Report};

#[test]
fn a_pmksa_for_another_ap_leaves_the_join_to_sae() {
    let (first, _) = by_sae();
    let p = first.pmksa().map(|mut p| {
        p.bssid = OTHER;
        p
    });
    let (m, auth, _) = offered(p.unwrap());
    assert_eq!(auth[MAC_HEADER_LEN..MAC_HEADER_LEN + 2], [3, 0], "an SAE commit");
    assert!(!m.used_pmksa());
}

#[test]
fn a_psk_join_never_sends_a_pmkid() {
    let (first, _) = by_sae();
    let mut m = join(JoinPolicy::PSK_ONLY);
    m.offer_pmksa(first.pmksa().unwrap());
    m.on_mgmt(&beacon(AP, &[&STA_RSNE_PSK]));
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("association");
    let rsne = assoc_elements(&req).into_iter().find(|e| e.0 == 48).unwrap().1;
    assert_eq!(rsne.len(), 20, "no PMKID list");
    assert!(!m.used_pmksa());
}

#[test]
fn an_ap_that_forgot_the_pmksa_refuses_and_sae_runs_again() {
    let (first, _) = by_sae();
    let (mut m, _, _) = offered(first.pmksa().unwrap());
    m.on_mgmt(&assoc_resp(53));
    assert_eq!(m.state(), MlmeState::Failed);
    assert_eq!(m.failure(), Some(MlmeFailure::AssocRejected(53)), "invalid PMKID");
    let report = |used_pmksa| Report {
        outcome: Outcome::Refused,
        counters: Counters::default(),
        state: 5,
        failure: m.failure(),
        akm: m.akm(),
        used_pmksa,
        pmksa: None,
    };
    assert!(retry_with_sae(&report(true)));
    assert!(!retry_with_sae(&report(false)), "a full SAE join that failed is not run again here");
}
