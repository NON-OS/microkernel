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

//! Supplicant state: the identities, nonces and elements fixed at the start of
//! a handshake, and the keys derived as it proceeds.

use super::ie::Ie;
use crate::eapol::build::EAPOL_VERSION_DEFAULT;
use crate::wpa::akm::Akm;
use crate::wpa::RSN_IE;

/// Where the handshake stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Waiting for message 1 (the AP's ANonce).
    Start,
    /// Message 1 seen, PTK derived, message 2 sent; awaiting message 3.
    PtkDerived,
    /// Message 3 verified, group key installed, message 4 sent. Connected; the
    /// group key handshake and message 3 retransmissions are still answered.
    Connected,
    /// Message 3 was authentic but unacceptable (its RSNE differs from the
    /// beacon's, or its key data does not hold a valid group key); the
    /// handshake is abandoned rather than trusting it.
    Failed,
}

/// Why an authentic handshake message was refused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Failure {
    /// Message 3's RSNE or RSNXE differs from the beacon's: a downgrade attempt
    /// (IEEE Std 802.11-2020, 12.7.6.4).
    IeMismatch,
    /// The key data would not unwrap, or held no group key of the right size.
    BadKeyData,
    /// A reply could not be framed.
    Internal,
}

/// How a handshake is set up: the keys and addresses, the AKM, the elements
/// the station sent in its association request (message 2 repeats them) and
/// the access point's from its beacon (message 3 is checked against them).
pub struct Config<'a> {
    pub pmk: [u8; 32],
    pub aa: [u8; 6],
    pub spa: [u8; 6],
    pub snonce: [u8; 32],
    pub akm: Akm,
    pub own_rsne: &'a [u8],
    pub own_rsnxe: Option<&'a [u8]>,
    pub ap_rsne: &'a [u8],
    pub ap_rsnxe: Option<&'a [u8]>,
    /// Management frame protection was negotiated, so message 3 delivers the
    /// IGTK as well.
    pub pmf: bool,
}

#[derive(Clone, Copy)]
pub struct Supplicant {
    pub(super) state: State,
    /// Pairwise master key from passphrase + SSID, or from SAE.
    pub(super) pmk: [u8; 32],
    /// Authenticator (AP) MAC.
    pub(super) aa: [u8; 6],
    /// Supplicant (our) MAC.
    pub(super) spa: [u8; 6],
    /// Our nonce, chosen once per handshake.
    pub(super) snonce: [u8; 32],
    /// AP nonce, learned from message 1.
    pub(super) anonce: [u8; 32],
    /// Pairwise transient key (KCK||KEK||TK), derived on message 1.
    pub(super) ptk: [u8; 48],
    /// Group temporal key, unwrapped from message 3 or a group message 1.
    pub(super) gtk: [u8; 32],
    pub(super) gtk_len: usize,
    /// The group key's index from its KDE.
    pub(super) gtk_id: u8,
    /// The group key's receive sequence counter from the Key RSC field.
    pub(super) gtk_rsc: u64,
    pub(super) akm: Akm,
    pub(super) own_rsne: Ie,
    pub(super) own_rsnxe: Ie,
    pub(super) ap_rsne: Ie,
    pub(super) ap_rsnxe: Ie,
    /// Whether `ap_rsne`/`ap_rsnxe` came from the beacon. A supplicant built
    /// with `new` has no beacon to compare message 3 with.
    pub(super) ap_known: bool,
    pub(super) pmf: bool,
    /// The replay counter of the last frame whose MIC verified. Any frame not
    /// above it is dropped (12.7.2).
    pub(super) rx_replay: Option<[u8; 8]>,
    /// The integrity group key from message 3 or a group message 1, with its
    /// index and the IGTK packet number the AP announced.
    pub(super) igtk: [u8; 16],
    pub(super) igtk_id: u16,
    pub(super) igtk_ipn: [u8; 6],
    pub(super) igtk_set: bool,
    /// The 802.1X version the AP spoke, mirrored in replies.
    pub(super) eapol_version: u8,
    pub(super) failure: Option<Failure>,
}

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
            ap_known: true,
            pmf: c.pmf,
            rx_replay: None,
            igtk: [0u8; 16],
            igtk_id: 0,
            igtk_ipn: [0u8; 6],
            igtk_set: false,
            eapol_version: EAPOL_VERSION_DEFAULT,
            failure: None,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// Why the handshake failed, once `Failed`.
    pub fn failure(&self) -> Option<Failure> {
        self.failure
    }

    /// The AKM this handshake runs.
    pub fn akm(&self) -> Akm {
        self.akm
    }

    /// Whether management frame protection was negotiated for this link.
    pub fn pmf(&self) -> bool {
        self.pmf
    }

    /// The pairwise temporal key (CCMP encryption key), valid once Connected.
    pub fn tk(&self) -> &[u8] {
        &self.ptk[32..48]
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> &[u8] {
        &self.gtk[..self.gtk_len]
    }

    /// The group key's index (1-3), valid once Connected. Group-addressed
    /// frames name their key by it, and an AP moves it between 1 and 2 on
    /// every rekey, so it has to be installed where the AP says.
    pub fn gtk_id(&self) -> u8 {
        self.gtk_id
    }

    /// The packet number the AP last sent under the group key, from the Key
    /// RSC of the message that delivered it: group frames must exceed it.
    pub fn gtk_rsc(&self) -> u64 {
        self.gtk_rsc
    }

    /// The integrity group key, its index and the AP's IGTK packet number,
    /// when management frame protection delivered one.
    pub fn igtk(&self) -> Option<(&[u8; 16], u16, [u8; 6])> {
        self.igtk_set.then_some((&self.igtk, self.igtk_id, self.igtk_ipn))
    }

    // KCK: signs EAPOL MICs. KEK: unwraps the group key. Both are slices of the
    // PTK, named here so the handshake code reads like the standard.
    pub(super) fn kck(&self) -> &[u8] {
        &self.ptk[0..16]
    }

    pub(super) fn kek(&self) -> [u8; 16] {
        let mut k = [0u8; 16];
        k.copy_from_slice(&self.ptk[16..32]);
        k
    }

    pub(super) fn fail(&mut self, why: Failure) {
        self.state = State::Failed;
        self.failure = Some(why);
    }
}
