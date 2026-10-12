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

//! Frame bodies from the LLC/SNAP header on, and a station that finished a
//! WPA2 join with the group key at a known receive sequence counter.

use crate::ap_sim::{gtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA};
use crate::hex::h;
use crate::receive::frame::{AP, STA};
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

pub fn ip_body(payload: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa030000000800");
    b.extend_from_slice(payload);
    b
}

pub fn eapol_body(eapol: &[u8]) -> Vec<u8> {
    let mut b = h("aaaa0300000088 8e");
    b.extend_from_slice(eapol);
    b
}

// A station that finished a WPA2 join: its supplicant, the PTK the AP holds.
pub fn joined() -> (LinkStation, [u8; 48]) {
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
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let mut m3 = message3(Akm::Psk, 2, &ANONCE, &ptk, &kd);
    m3[65] = 9; // Key RSC: the AP's group frames are past PN 9
    let m3 = {
        // Recompute the MIC over the edited RSC.
        let mic = nonos_wifi_core::eapol::mic::compute_mic(&ptk[..16], &m3);
        let mut f = m3;
        f[81..97].copy_from_slice(&mic);
        f
    };
    sup.step(&m3);
    let mut tkb = [0u8; 16];
    tkb.copy_from_slice(&ptk[32..48]);
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: tkb });
    s.set_supplicant(sup);
    (s, ptk)
}
