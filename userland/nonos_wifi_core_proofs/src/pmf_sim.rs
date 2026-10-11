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

//! A station joined to an access point that requires management frame
//! protection, holding the keys that AP handed it.

use crate::ap_sim::{gtk_kde, igtk_kde, message1, message3, AA, ANONCE, GTK, SNONCE, SPA};
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

pub const IGTK: [u8; 16] = [0x33; 16];
/// The AP's RSNE: CCMP, PSK, MFPR and MFPC set.
pub const AP_RSNE_MFP: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 2, 0xcc, 0];

/// A joined station holding the IGTK (index 4, IPN 1), and the PTK.
pub fn joined_pmf() -> (LinkStation, [u8; 48]) {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    // The station advertises capable and required alike.
    let mut own = AP_RSNE_MFP;
    own[20] = 0xc0;
    let mut sup = Supplicant::configure(&Config {
        pmk: key,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm: Akm::Psk,
        own_rsne: &own,
        own_rsnxe: None,
        ap_rsne: &AP_RSNE_MFP,
        ap_rsnxe: None,
        pmf: true,
    });
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = Akm::Psk.derive_ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut kd = AP_RSNE_MFP.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    kd.extend_from_slice(&igtk_kde(&IGTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    assert!(sup.igtk().is_some(), "the join delivered the IGTK");
    let mut s = LinkStation::new(SPA);
    s.associate(AA, Ccmp::Software { tk: tk(&ptk) });
    s.set_supplicant(sup);
    (s, ptk)
}

pub fn tk(ptk: &[u8; 48]) -> [u8; 16] {
    ptk[32..48].try_into().unwrap()
}
