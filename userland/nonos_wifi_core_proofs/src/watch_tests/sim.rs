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

//! A joined station, with or without management frame protection. The keys
//! are fixed so the pinned vectors (computed apart from this crate) fit it.

use crate::ap_sim::{
    gtk_kde, igtk_kde, message1, message3, AA, ANONCE, AP_RSNE_MIXED, GTK, SNONCE, SPA,
};
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

pub const AP: [u8; 6] = AA;
pub const STA: [u8; 6] = SPA;
pub const TK: [u8; 16] = [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
pub const IGTK: [u8; 16] = [64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79];

pub fn h(s: &str) -> Vec<u8> {
    let s: String = s.split_whitespace().collect();
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

/// A station that finished a WPA2 join, with management frame protection or
/// without; with it, message 3 delivered `IGTK` (index 4, IPN 1).
pub fn joined(pmf: bool) -> LinkStation {
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
    if pmf {
        kd.extend_from_slice(&igtk_kde(&IGTK));
    }
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    assert_eq!(sup.igtk().is_some(), pmf);
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Software { tk: TK });
    s.set_supplicant(sup);
    s
}
