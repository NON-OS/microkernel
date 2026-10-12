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

//! Pairwise master key from passphrase + SSID, or from SAE.

use super::*;

/// KCK and KEK then the longest TK (GCMP-256's).
pub(in crate::wpa::supplicant) const PTK_MAX: usize = 64;

#[derive(Clone, Copy)]
pub struct Supplicant {
    pub(in crate::wpa::supplicant) state: State,
    /// Pairwise master key from passphrase + SSID, or from SAE.
    pub(in crate::wpa::supplicant) pmk: [u8; 32],
    /// Authenticator (AP) MAC.
    pub(in crate::wpa::supplicant) aa: [u8; 6],
    /// Supplicant (our) MAC.
    pub(in crate::wpa::supplicant) spa: [u8; 6],
    /// Our nonce, chosen once per handshake.
    pub(in crate::wpa::supplicant) snonce: [u8; 32],
    /// AP nonce, learned from message 1.
    pub(in crate::wpa::supplicant) anonce: [u8; 32],
    /// Pairwise transient key (KCK||KEK||TK), derived on message 1; the
    /// pairwise cipher says how much of it is TK.
    pub(in crate::wpa::supplicant) ptk: [u8; PTK_MAX],
    pub(in crate::wpa::supplicant) pairwise: Cipher,
    pub(in crate::wpa::supplicant) group: Cipher,
    /// Group temporal key, unwrapped from message 3 or a group message 1.
    pub(in crate::wpa::supplicant) gtk: [u8; 32],
    pub(in crate::wpa::supplicant) gtk_len: usize,
    /// The group key's index from its KDE.
    pub(in crate::wpa::supplicant) gtk_id: u8,
    /// The group key's receive sequence counter from the Key RSC field.
    pub(in crate::wpa::supplicant) gtk_rsc: u64,
    pub(in crate::wpa::supplicant) akm: Akm,
    pub(in crate::wpa::supplicant) own_rsne: Ie,
    pub(in crate::wpa::supplicant) own_rsnxe: Ie,
    pub(in crate::wpa::supplicant) ap_rsne: Ie,
    pub(in crate::wpa::supplicant) ap_rsnxe: Ie,
    /// Whether `ap_rsne`/`ap_rsnxe` came from the beacon. A supplicant built
    /// with `new` has no beacon to compare message 3 with.
    pub(in crate::wpa::supplicant) ap_known: bool,
    pub(in crate::wpa::supplicant) pmf: bool,
    /// The replay counter of the last frame whose MIC verified. Any frame not
    /// above it is dropped (12.7.2).
    pub(in crate::wpa::supplicant) rx_replay: Option<[u8; 8]>,
    /// The integrity group key from message 3 or a group message 1, with its
    /// index and the IGTK packet number the AP announced.
    pub(in crate::wpa::supplicant) igtk: [u8; 16],
    pub(in crate::wpa::supplicant) igtk_id: u16,
    pub(in crate::wpa::supplicant) igtk_ipn: [u8; 6],
    pub(in crate::wpa::supplicant) igtk_set: bool,
    /// The 802.1X version the AP spoke, mirrored in replies.
    pub(in crate::wpa::supplicant) eapol_version: u8,
    pub(in crate::wpa::supplicant) failure: Option<Failure>,
}
