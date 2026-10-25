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

//! The WPA2-only join, for drivers that have no SAE randomness.

use super::*;

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
            entropy: Entropy { snonce, sae: [0u8; SAE_ENTROPY], rekey: [0u8; 32] },
        });
        m.sae_entropy = None;
        m
    }
}
