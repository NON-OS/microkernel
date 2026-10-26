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

//! Begin an association under `req.policy`: SAE when the network offers it
//! and the policy allows, PSK otherwise (never for a WPA3-only policy).

use super::*;

impl Mlme {
    /// Begin an association under `req.policy`: SAE when the network offers it
    /// and the policy allows, PSK otherwise (never for a WPA3-only policy).
    pub fn join(req: &JoinRequest) -> Self {
        let mut ssid = [0u8; 32];
        let len = req.ssid.len().min(32);
        ssid[..len].copy_from_slice(&req.ssid[..len]);
        let secret_len = req.passphrase.len().min(SECRET_MAX);
        // The passphrase is copied straight into the machine, which wipes it
        // when it goes, never into a local that would outlive the copy.
        let mut m = Self {
            state: MlmeState::Scanning,
            our_mac: req.our_mac,
            ssid,
            ssid_len: len,
            secret: [0u8; SECRET_MAX],
            secret_len,
            secret_too_long: req.passphrase.len() > SECRET_MAX,
            policy: req.policy,
            snonce: req.entropy.snonce,
            sae_entropy: Some(req.entropy.sae),
            rekey_seed: req.entropy.rekey,
            bssid: [0u8; 6],
            channel: 0,
            domain: WORLD,
            seq: 0,
            selection: None,
            ap_rsne: Ie::NONE,
            ap_rsnxe: Ie::NONE,
            rates: None,
            sae: None,
            pmk: [0u8; 32],
            pmk_given: false,
            pt: None,
            pmkid: None,
            cached: None,
            cache_used: false,
            reassoc_from: None,
            verifies_beacons: false,
            supplicant: None,
            failure: None,
            step_since: None,
            expired: false,
        };
        m.secret[..secret_len].copy_from_slice(&req.passphrase[..secret_len]);
        m
    }
}
