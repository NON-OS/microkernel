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

//! A start with no beacon elements, and the supplicant's own helpers: its
//! key parts and its failure.

use super::kinds::{Config, Failure, State};
use super::Supplicant;
use crate::wpa::akm::Akm;
use crate::wpa::RSN_IE;

impl Supplicant {
    /// Begin a WPA2-PSK handshake. `snonce` must be freshly random per session;
    /// it is supplied by the caller so this stays pure and host-testable. With
    /// no beacon elements to hold message 3 to, prefer [`Supplicant::configure`].
    pub fn new(pmk: [u8; 32], aa: [u8; 6], spa: [u8; 6], snonce: [u8; 32]) -> Self {
        let mut s = Self::configure(&Config {
            pmk,
            aa,
            spa,
            snonce,
            akm: Akm::Psk,
            own_rsne: &RSN_IE,
            own_rsnxe: None,
            ap_rsne: &[],
            ap_rsnxe: None,
            pmf: false,
        });
        s.ap_known = false;
        s
    }

    // KCK: signs EAPOL MICs. KEK: unwraps the group key. Both are slices of the
    // PTK, named here so the handshake code reads like the standard.
    pub(in crate::wpa::supplicant) fn kck(&self) -> &[u8] {
        &self.ptk[0..16]
    }

    pub(in crate::wpa::supplicant) fn kek(&self) -> [u8; 16] {
        let mut k = [0u8; 16];
        k.copy_from_slice(&self.ptk[16..32]);
        k
    }

    pub(in crate::wpa::supplicant) fn fail(&mut self, why: Failure) {
        self.state = State::Failed;
        self.failure = Some(why);
    }
}
