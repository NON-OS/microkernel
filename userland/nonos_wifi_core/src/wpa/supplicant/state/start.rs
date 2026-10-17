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

//! A handshake begun from its configuration.

use super::super::ie::Ie;
use super::kinds::{Config, State};
use super::Supplicant;
use crate::eapol::build::EAPOL_VERSION_DEFAULT;

impl Supplicant {
    /// Begin a handshake for the negotiated AKM, with the elements message 2
    /// repeats and the beacon elements message 3 must match.
    pub fn configure(c: &Config) -> Self {
        Self {
            state: State::Start,
            pmk: c.pmk,
            aa: c.aa,
            spa: c.spa,
            snonce: c.snonce,
            anonce: [0u8; 32],
            ptk: [0u8; 48],
            gtk: [0u8; 32],
            gtk_len: 0,
            gtk_id: 0,
            gtk_rsc: 0,
            akm: c.akm,
            own_rsne: Ie::from_slice(c.own_rsne),
            own_rsnxe: Ie::from_option(c.own_rsnxe),
            ap_rsne: Ie::from_slice(c.ap_rsne),
            ap_rsnxe: Ie::from_option(c.ap_rsnxe),
            pmf: c.pmf,
            rx_replay: None,
            igtk: [0u8; 16],
            igtk_id: 0,
            igtk_ipn: [0u8; 6],
            igtk_set: false,
            eapol_version: EAPOL_VERSION_DEFAULT,
            transition_disable: 0,
            failure: None,
        }
    }
}
