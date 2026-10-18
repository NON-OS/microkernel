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

//! A protected supplicant, part way through its four-way handshake.

use crate::ap_sim::{message1, AA, ANONCE, SNONCE, SPA};
use crate::pmf_sim::AP_RSNE_MFP;
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

pub fn psk() -> [u8; 32] {
    pmk(b"ThisIsAPassword", b"ThisIsASSID")
}

/// A protected supplicant past message 1, and its PTK.
pub fn past_message1() -> (Supplicant, [u8; 48]) {
    let mut own = AP_RSNE_MFP;
    own[20] = 0xc0;
    let (ap_rsne, own_rsne) = (&AP_RSNE_MFP[..], &own[..]);
    let (pmk, aa, spa, snonce, akm) = (psk(), AA, SPA, SNONCE, Akm::Psk);
    let c = Config {
        pmk,
        aa,
        spa,
        snonce,
        akm,
        own_rsne,
        own_rsnxe: None,
        ap_rsne,
        ap_rsnxe: None,
        pmf: true,
    };
    let mut sup = Supplicant::configure(&c);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    (sup, Akm::Psk.derive_ptk(&psk(), &AA, &SPA, &ANONCE, &SNONCE))
}
