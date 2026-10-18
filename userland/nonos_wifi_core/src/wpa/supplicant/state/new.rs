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

//! Begin a WPA2-PSK handshake.

use super::*;

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
}
