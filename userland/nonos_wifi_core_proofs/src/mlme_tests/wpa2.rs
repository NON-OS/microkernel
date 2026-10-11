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

//! A WPA2 join, run to Connected.

use nonos_wifi_core::dot11::auth::parse_auth;
use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::mlme::MlmeState;
use nonos_wifi_core::rsn::JoinPolicy;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::RSN_IE;

use super::assoc::{assoc_elements, assoc_resp};
use super::base::{join, AP, PASS, SNONCE, SSID, STA};
use super::frames::{beacon, open_auth_ok, rsne};
use crate::ap_sim::{gtk_kde, message1, message3, GTK};

#[test]
fn a_wpa2_join_runs_to_connected() {
    let psk_rsne = rsne(&[2], 0, 4);
    let mut m = join(JoinPolicy::ANY);
    let auth = m.on_mgmt(&beacon(AP, &[&psk_rsne])).tx.expect("open authentication");
    let a = parse_auth(&auth, &AP, &STA);
    assert!(a.is_none(), "the request is to the AP, not from it");
    assert_eq!(&auth[4..10], &AP, "addressed to the AP");
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("association request");
    let els = assoc_elements(&req);
    assert_eq!(els[0], (0, SSID.to_vec()));
    let rsn: Vec<u8> = els.iter().find(|e| e.0 == 48).map(|e| e.1.clone()).unwrap();
    assert_eq!(rsn, RSN_IE[2..].to_vec(), "the WPA2-PSK RSNE");
    assert!(m.on_mgmt(&assoc_resp(0)).tx.is_none());
    assert_eq!(m.state(), MlmeState::FourWay);

    let key = pmk(PASS, SSID);
    let anonce = [0xA1; 32];
    let m2 = m.on_eapol(&message1(Akm::Psk, 1, &anonce)).tx.expect("message 2");
    let ptk = Akm::Psk.derive_ptk(&key, &AP, &STA, &anonce, &SNONCE);
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2));
    let mut kd = psk_rsne.clone();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(m.on_eapol(&message3(Akm::Psk, 2, &anonce, &ptk, &kd)).tx.is_some());
    assert_eq!(m.state(), MlmeState::Connected);
    assert_eq!(m.tk(), Some(&ptk[32..48]));
    assert_eq!(m.akm(), Some(Akm::Psk));
    assert!(m.supplicant().is_some(), "the data path takes the supplicant over");
}
