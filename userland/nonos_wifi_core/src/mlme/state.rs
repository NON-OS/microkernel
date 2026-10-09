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

//! MLME state: the target network, the person's policy, the randomness the
//! join consumes, the BSS and security chosen from its beacon, and the SAE
//! exchange and handshake that follow.

use alloc::vec::Vec;

use super::failure::MlmeFailure;
use crate::dot11::ies::Rates;
use crate::rsn::{JoinPolicy, Selection};
use crate::sae::SaeStation;
use crate::wpa::akm::Akm;
use crate::wpa::supplicant::ie::Ie;
use crate::wpa::supplicant::Supplicant;

/// How far the association has progressed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MlmeState {
    /// Watching beacons for the target SSID.
    Scanning,
    /// A matching BSS was found; authentication (Open System or SAE) sent.
    Authenticating,
    /// Authenticated; association request sent.
    Associating,
    /// Associated; running the four-way handshake.
    FourWay,
    /// Connected: pairwise and group keys installed, link up.
    Connected,
    /// The join failed; `Mlme::failure` says where.
    Failed,
}

/// What one input produced: the frame to transmit next, if any. The link
/// state is read separately from `Mlme::state`.
pub struct MlmeOutput {
    pub tx: Option<Vec<u8>>,
}

impl MlmeOutput {
    pub(super) fn none() -> Self {
        Self { tx: None }
    }

    pub(super) fn send(frame: Vec<u8>) -> Self {
        Self { tx: Some(frame) }
    }
}

/// The bytes of fresh randomness SAE consumes: 32 for rand, 32 for mask and
/// 64 for the hunting-and-pecking stand-in password.
pub const SAE_ENTROPY: usize = 128;
/// The longest passphrase or SAE password held.
pub const SECRET_MAX: usize = 64;

/// Fresh randomness for one join. Never derived from anything sent on the air:
/// the SNonce goes out in message 2, so SAE's secrets are drawn separately.
#[derive(Clone, Copy)]
pub struct Entropy {
    pub snonce: [u8; 32],
    pub sae: [u8; SAE_ENTROPY],
}

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
    pub(super) state: MlmeState,
    pub(super) our_mac: [u8; 6],
    pub(super) ssid: [u8; 32],
    pub(super) ssid_len: usize,
    pub(super) secret: [u8; SECRET_MAX],
    pub(super) secret_len: usize,
    /// The passphrase did not fit `secret`; any join that needs it fails.
    pub(super) secret_too_long: bool,
    pub(super) policy: JoinPolicy,
    pub(super) snonce: [u8; 32],
    pub(super) sae_entropy: Option<[u8; SAE_ENTROPY]>,
    pub(super) bssid: [u8; 6],
    pub(super) channel: u8,
    pub(super) seq: u16,
    pub(super) selection: Option<Selection>,
    pub(super) ap_rsne: Ie,
    pub(super) ap_rsnxe: Ie,
    pub(super) rates: Option<Rates>,
    pub(super) sae: Option<SaeStation>,
    pub(super) pmk: [u8; 32],
    pub(super) supplicant: Option<Supplicant>,
    pub(super) failure: Option<MlmeFailure>,
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

    pub fn state(&self) -> MlmeState {
        self.state
    }

    /// Why the join failed, once `Failed`.
    pub fn failure(&self) -> Option<MlmeFailure> {
        self.failure
    }

    /// The security chosen from the beacon, once a BSS was selected.
    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    /// The AKM the join runs, once a BSS was selected.
    pub fn akm(&self) -> Option<Akm> {
        self.selection.map(|s| s.akm)
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// The supplicant, once connected: the data path keeps answering the
    /// access point's group key handshakes with it.
    pub fn supplicant(&self) -> Option<Supplicant> {
        self.supplicant.filter(|_| self.state == MlmeState::Connected)
    }

    /// The pairwise temporal key, valid once Connected.
    pub fn tk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.tk())
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.gtk())
    }

    /// The group key's index, valid once Connected.
    pub fn gtk_id(&self) -> Option<u8> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.gtk_id())
    }

    /// The station MAC and the joined AP's BSSID, needed by the data path to
    /// address encrypted frames. Meaningful once a BSS has been selected.
    pub fn our_mac(&self) -> [u8; 6] {
        self.our_mac
    }

    pub fn bssid(&self) -> [u8; 6] {
        self.bssid
    }

    pub(super) fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }

    pub(super) fn secret(&self) -> &[u8] {
        &self.secret[..self.secret_len]
    }

    // Each transmitted management frame advances the sequence counter.
    pub(super) fn next_seq(&mut self) -> u16 {
        let s = self.seq;
        self.seq = self.seq.wrapping_add(1);
        s
    }

    pub(super) fn fail(&mut self, why: MlmeFailure) -> MlmeOutput {
        self.state = MlmeState::Failed;
        self.failure = Some(why);
        MlmeOutput::none()
    }
}
