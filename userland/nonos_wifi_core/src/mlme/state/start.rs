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

//! A join begun: from a passphrase alone, or from a driver's request.

use super::kinds::MlmeState;
use super::request::{Entropy, JoinRequest, SAE_ENTROPY, SECRET_MAX};
use super::Mlme;
use crate::rsn::JoinPolicy;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    /// Begin a WPA2-PSK association to `ssid` with `passphrase`. `snonce` must
    /// be a fresh random nonce for the eventual handshake; it is passed in so
    /// the state machine stays pure and host-testable. Without SAE randomness
    /// this joins WPA2 only; use [`Mlme::join`] for WPA3.
    pub fn new(our_mac: [u8; 6], ssid: &[u8], passphrase: &[u8], snonce: [u8; 32]) -> Self {
        let mut m = Self::join(&JoinRequest {
            our_mac,
            ssid,
            passphrase,
            policy: JoinPolicy::PSK_ONLY,
            entropy: Entropy { snonce, sae: [0u8; SAE_ENTROPY] },
        });
        m.sae_entropy = None;
        m
    }

    /// Begin an association under `req.policy`: SAE when the network offers it
    /// and the policy allows, PSK otherwise (never for a WPA3-only policy).
    pub fn join(req: &JoinRequest) -> Self {
        let mut ssid = [0u8; 32];
        let len = req.ssid.len().min(32);
        ssid[..len].copy_from_slice(&req.ssid[..len]);
        let mut secret = [0u8; SECRET_MAX];
        let secret_len = req.passphrase.len().min(SECRET_MAX);
        secret[..secret_len].copy_from_slice(&req.passphrase[..secret_len]);
        Self {
            state: MlmeState::Scanning,
            our_mac: req.our_mac,
            ssid,
            ssid_len: len,
            secret,
            secret_len,
            secret_too_long: req.passphrase.len() > SECRET_MAX,
            policy: req.policy,
            snonce: req.entropy.snonce,
            sae_entropy: Some(req.entropy.sae),
            bssid: [0u8; 6],
            channel: 0,
            seq: 0,
            selection: None,
            ap_rsne: Ie::NONE,
            ap_rsnxe: Ie::NONE,
            rates: None,
            sae: None,
            pmk: [0u8; 32],
            supplicant: None,
            failure: None,
        }
    }
}
