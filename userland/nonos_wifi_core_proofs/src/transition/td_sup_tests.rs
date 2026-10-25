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

//! The Transition Disable bitmap reaches the supplicant, and the station
//! the drivers read it from, from message 3 only.

use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA};
use nonos_wifi_core::eapol::kde::TD_WPA3_PERSONAL;
use nonos_wifi_core::station::LinkStation;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, State, Supplicant};

/// A WPA2 handshake whose message 3 carries `extra` after the RSNE and GTK.
pub(super) fn handshake(extra: &[u8]) -> Supplicant {
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
        pmf: false,
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    assert_eq!(sup.transition_disable(), None, "nothing before message 3");
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    kd.extend_from_slice(extra);
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    sup
}

#[test]
fn message3_s_transition_disable_reaches_the_supplicant_and_the_station() {
    let sup = handshake(&super::td_kde_tests::TD);
    assert_eq!((sup.state(), sup.transition_disable()), (State::Connected, Some(TD_WPA3_PERSONAL)));
    let mut station = LinkStation::new(SPA);
    assert_eq!(station.transition_disable(), 0, "no supplicant, no bitmap");
    station.set_supplicant(sup);
    assert_eq!(station.transition_disable(), TD_WPA3_PERSONAL);
    let none = handshake(&[]);
    assert_eq!((none.state(), none.transition_disable()), (State::Connected, None));
    let other = handshake(&[0xDD, 5, 0x50, 0x6F, 0x9A, 0x20, 0x0E]);
    let bits = other.transition_disable().unwrap_or(0);
    assert_eq!(bits & TD_WPA3_PERSONAL, 0, "bit 0 clear pins nothing");
}
