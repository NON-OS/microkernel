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

//! A protected link holding a BIGTK (index 6, BIPN 3), and beacons for it.

use super::handshake::past_message1;
use crate::ap_sim::{bigtk_kde, gtk_kde, igtk_kde, message3, AA, ANONCE, GTK, SPA};
use crate::pmf_sim::{AP_RSNE_MFP, IGTK};
use crate::soft::protect::BIGTK;
use nonos_wifi_core::pmf::bip_protect_beacon;
use nonos_wifi_core::station::{Ccmp, LinkStation};
use nonos_wifi_core::wpa::akm::Akm;

/// Message 3's key data, its group keys followed by `extra`.
pub fn kd3(extra: &[u8]) -> Vec<u8> {
    [&AP_RSNE_MFP[..], &gtk_kde(1, &GTK), &igtk_kde(&IGTK), extra].concat()
}

/// A joined station whose message 3 carried `extra` after its group keys.
pub fn joined_with(extra: &[u8]) -> (LinkStation, [u8; 48]) {
    let (mut sup, ptk) = past_message1();
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd3(extra)));
    let mut s = LinkStation::new(SPA);
    let tk: [u8; 16] = ptk[32..48].try_into().unwrap();
    s.associate(AA, Ccmp::Software { tk });
    s.set_supplicant(sup);
    (s, ptk)
}

/// A joined station holding the BIGTK at index 6, BIPN 3.
pub fn joined_bp() -> (LinkStation, [u8; 48]) {
    joined_with(&bigtk_kde(6, 3, &BIGTK))
}

/// The AP's beacon, bare.
pub fn bare() -> Vec<u8> {
    crate::mlme::sim::beacon(AA, &[&AP_RSNE_MFP])
}

/// The AP's beacon under `key` at index `id`, BIPN `bipn`.
pub fn signed(id: u16, bipn: u64, key: &[u8; 16]) -> Vec<u8> {
    bip_protect_beacon(&bare(), id, bipn, key).unwrap()
}
