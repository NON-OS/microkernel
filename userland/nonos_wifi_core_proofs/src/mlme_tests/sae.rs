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

//! A WPA3-SAE join run to the four-way handshake, with the keys the AP derives.

use nonos_wifi_core::dot11::auth::parse_auth;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::mlme::Mlme;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeStation, SaeStep};

use super::assoc::{assoc_elements, assoc_resp};
use super::base::{join, AP, PASS, SSID, STA};
use super::frames::{beacon, sae_from_ap};
use crate::ap_sim::AP_RSNE_MIXED;

// Run a WPA3 join against an AP built from the same SAE code.
pub(super) fn sae_join(h2e: bool) -> (Mlme, [u8; 32]) {
    let mut els: Vec<Vec<u8>> = vec![AP_RSNE_MIXED.to_vec()];
    if h2e {
        els.push(vec![244, 1, 0x20]);
    }
    let refs: Vec<&[u8]> = els.iter().map(|e| e.as_slice()).collect();
    let mut m = join(JoinPolicy::ANY);
    let commit = m.on_mgmt(&beacon(AP, &refs)).tx.expect("an SAE commit");
    let pwe = if h2e {
        pwe_from_pt(&derive_pt(SSID, PASS, None).unwrap(), &AP, &STA).unwrap()
    } else {
        derive_pwe(PASS, &vec![9u8; PASS.len()], &AP, &STA).unwrap()
    };
    let mut ap = SaeStation::start(pwe, h2e, &[0x33; 32], &[0x44; 32]).unwrap();
    // The station's commit, as the AP reads it.
    let sta = parse_auth(&commit, &AP, &STA).is_none();
    assert!(sta);
    let body = &commit[MAC_HEADER_LEN..];
    assert_eq!(u16::from_le_bytes([body[0], body[1]]), 3, "algorithm SAE");
    let status = u16::from_le_bytes([body[4], body[5]]);
    assert_eq!(status, if h2e { 126 } else { 0 });
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, status, &body[6..]) else {
        panic!("the AP accepts the station's commit");
    };
    let (_, ap_status, ap_commit) = ap.commit_frame();
    let confirm = m.on_mgmt(&sae_from_ap(1, ap_status, &ap_commit)).tx.expect("the confirm");
    let cb = &confirm[MAC_HEADER_LEN..];
    assert!(matches!(ap.on_frame(2, 0, &cb[6..]), SaeStep::Accepted), "the AP verifies it");
    let req = m.on_mgmt(&sae_from_ap(2, 0, &ap_confirm)).tx.expect("association");
    let els = assoc_elements(&req);
    let rsn = els.iter().find(|e| e.0 == 48).unwrap().1.clone();
    assert_eq!(rsn[17], 8, "AKM SAE");
    assert_eq!(rsn[18] & 0x80, 0x80, "MFPC");
    assert_eq!(els.iter().any(|e| e.0 == 244), h2e, "the RSNXE rides with H2E");
    m.on_mgmt(&assoc_resp(0));
    (m, ap.pmk().unwrap().0)
}
