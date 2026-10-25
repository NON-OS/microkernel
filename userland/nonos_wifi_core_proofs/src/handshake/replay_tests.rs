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

//! A forged or replayed message 3 is dropped without ending the handshake,
//! and a repeated message 3 gets message 4 again without reinstalling a key.

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use nonos_wifi_core::eapol::parse::parse;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

#[test]
fn a_forged_or_replayed_message3_is_dropped_without_ending_the_handshake() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 5, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let mut forged = message3(Akm::Psk, 6, &ANONCE, &ptk, &kd);
    forged[90] ^= 1; // a bit of the MIC
    assert!(sup.step(&forged).reply.is_none());
    assert_eq!(sup.state(), State::PtkDerived, "a forged frame does not end the join");
    let mut wrong_anonce = message3(Akm::Psk, 6, &[0xEE; 32], &ptk, &kd);
    assert!(sup.step(&wrong_anonce).reply.is_none(), "another ANonce is not message 3");
    wrong_anonce.clear();
    let good = message3(Akm::Psk, 6, &ANONCE, &ptk, &kd);
    assert!(sup.step(&good).reply.is_some());
    // The same message 3 again (same counter) is a replay and is dropped.
    assert!(sup.step(&good).reply.is_none(), "replayed counter dropped");
    assert_eq!(sup.state(), State::Connected);
}

#[test]
fn a_repeated_message3_gets_message4_again_without_reinstalling() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    // The AP did not get message 4 and repeats message 3 with the next counter.
    let mut kd2 = AP_RSNE_MIXED.to_vec();
    kd2.extend_from_slice(&gtk_kde(2, &[0x99; 16]));
    let out = sup.step(&message3(Akm::Psk, 3, &ANONCE, &ptk, &kd2));
    let m4 = out.reply.expect("message 4 again");
    assert_eq!(parse(&m4).unwrap().replay_counter, rc(3));
    assert!(!out.new_group_key, "nothing is reinstalled");
    assert_eq!(sup.gtk(), &GTK, "the group key in use is kept");
    assert_eq!(sup.gtk_id(), 1);
}
