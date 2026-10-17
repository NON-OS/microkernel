// NONOS Operating System (AGPL-3.0-or-later)

//! One access point as heard: a BSSID with one RSNE and RSNXE on one channel.
//! The same BSSID heard with other elements is another candidate, so a frame
//! forged under a real AP's BSSID never replaces what the AP itself sent.

use alloc::vec::Vec;

use crate::wpa::akm::Akm;

pub struct Candidate {
    pub(super) frame: Vec<u8>,
    pub(super) channel: u8,
    pub(super) bssid: [u8; 6],
    pub(super) ies: Vec<u8>,
    pub(super) rank: u8,
    pub(super) signal: u8,
    pub(super) heard: u16,
}

impl Candidate {
    /// The latest beacon or probe response heard from it, for `Mlme::on_mgmt`.
    pub fn frame(&self) -> &[u8] {
        &self.frame
    }

    /// The channel to join it on.
    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn bssid(&self) -> [u8; 6] {
        self.bssid
    }

    /// Better security first, then the stronger signal, then the one heard
    /// more often.
    pub(super) fn key(&self) -> (u8, u8, u16) {
        (self.rank, self.signal, self.heard)
    }

    pub(super) fn is(&self, bssid: [u8; 6], channel: u8, ies: &[u8]) -> bool {
        self.bssid == bssid && self.channel == channel && self.ies == ies
    }
}

/// How strong the AKM a join would run is: SAE, then PSK-SHA256, then PSK.
pub(super) fn rank(akm: Akm) -> u8 {
    match akm {
        Akm::Sae => 3,
        Akm::PskSha256 => 2,
        Akm::Psk => 1,
    }
}
