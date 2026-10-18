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

//! The station's side of the handshake proofs: a supplicant configured for an
//! AKM and the RSNEs of both ends, and the PTK the access point derives.

use crate::ap_sim::{AA, SNONCE, SPA};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Supplicant};

pub const PMK_SAE: [u8; 32] = [0x77; 32];

pub fn psk() -> [u8; 32] {
    pmk(b"ThisIsAPassword", b"ThisIsASSID")
}

pub fn configured(akm: Akm, pmk: [u8; 32], own_rsne: &[u8], ap_rsne: &[u8]) -> Supplicant {
    Supplicant::configure(&Config {
        pmk,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm,
        own_rsne,
        own_rsnxe: None,
        ap_rsne,
        ap_rsnxe: None,
        pmf: akm == Akm::Sae,
    })
}

pub fn ptk_for(akm: Akm, pmk: &[u8; 32], anonce: &[u8; 32]) -> [u8; 48] {
    akm.derive_ptk(pmk, &AA, &SPA, anonce, &SNONCE)
}
