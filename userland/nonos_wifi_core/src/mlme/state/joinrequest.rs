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

//! One join: the station, the network, the secret and what the person
//! allowed.

use super::*;

/// One join: the station, the network, the secret and what the person allowed.
pub struct JoinRequest<'a> {
    pub our_mac: [u8; 6],
    pub ssid: &'a [u8],
    pub passphrase: &'a [u8],
    pub policy: JoinPolicy,
    pub entropy: Entropy,
}

#[derive(Clone, Copy)]
pub struct Mlme {
    pub(in crate::mlme) state: MlmeState,
    pub(in crate::mlme) our_mac: [u8; 6],
    pub(in crate::mlme) ssid: [u8; 32],
    pub(in crate::mlme) ssid_len: usize,
    pub(in crate::mlme) secret: [u8; SECRET_MAX],
    pub(in crate::mlme) secret_len: usize,
    /// The passphrase did not fit `secret`; any join that needs it fails.
    pub(in crate::mlme) secret_too_long: bool,
    pub(in crate::mlme) policy: JoinPolicy,
    pub(in crate::mlme) snonce: [u8; 32],
    pub(in crate::mlme) sae_entropy: Option<[u8; SAE_ENTROPY]>,
    pub(in crate::mlme) bssid: [u8; 6],
    pub(in crate::mlme) channel: u8,
    pub(in crate::mlme) domain: Domain,
    pub(in crate::mlme) seq: u16,
    pub(in crate::mlme) selection: Option<Selection>,
    pub(in crate::mlme) ap_rsne: Ie,
    pub(in crate::mlme) ap_rsnxe: Ie,
    pub(in crate::mlme) rates: Option<Rates>,
    pub(in crate::mlme) sae: Option<SaeStation>,
    pub(in crate::mlme) pmk: [u8; 32],
    pub(in crate::mlme) supplicant: Option<Supplicant>,
    pub(in crate::mlme) failure: Option<MlmeFailure>,
}

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
}
