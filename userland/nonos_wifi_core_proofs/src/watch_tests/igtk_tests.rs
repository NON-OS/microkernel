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

//! Once management frame protection is negotiated, the IGTK is required.

use super::sim::IGTK;
use crate::ap_sim::{
    group1, gtk_kde, igtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA,
};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Failure, State, Supplicant};

fn handshake(pmf: bool, with_igtk: bool) -> (Supplicant, [u8; 48]) {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::configure(&Config {
        pmk: key,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &nonos_wifi_core::wpa::RSN_IE,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MIXED,
        ap_rsnxe: None,
        pmf,
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    if with_igtk {
        kd.extend_from_slice(&igtk_kde(&IGTK));
    }
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    (sup, ptk)
}

#[test]
fn message3_without_the_igtk_fails_when_pmf_was_negotiated() {
    let (sup, _) = handshake(true, false);
    assert_eq!(sup.state(), State::Failed);
    assert_eq!(sup.failure(), Some(Failure::BadKeyData));
    for (pmf, igtk) in [(false, false), (false, true), (true, true)] {
        assert_eq!(handshake(pmf, igtk).0.state(), State::Connected, "pmf {pmf} igtk {igtk}");
    }
}

#[test]
fn a_rekey_without_the_igtk_is_not_answered_when_pmf_was_negotiated() {
    let next = [0x5Au8; 16];
    let (mut sup, ptk) = handshake(true, true);
    let out = sup.step(&group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next)));
    assert!(out.reply.is_none() && !out.new_group_key);
    assert_eq!(sup.gtk(), GTK, "the key in use stays");
    let mut kd = gtk_kde(2, &next);
    kd.extend_from_slice(&igtk_kde(&IGTK));
    let out = sup.step(&group1(Akm::Psk, 4, &ptk, &kd));
    assert!(out.reply.is_some() && out.new_group_key);
    let (mut plain, ptk) = handshake(false, false);
    assert!(plain.step(&group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next))).reply.is_some());
}
