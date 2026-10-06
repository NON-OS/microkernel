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

//! The three frames the supplicant sends (IEEE Std 802.11-2020, 12.7.6.3,
//! 12.7.6.5 and 12.7.7.3). Message 2: Pairwise and MIC, the SNonce, and the
//! RSNE (and RSNXE) of the association request as key data. Message 4:
//! Pairwise, MIC and Secure, a zero nonce and no key data. Group message 2:
//! MIC and Secure, no key data. Each echoes the replay counter of the message
//! it answers and carries the key descriptor version and MIC of the AKM.

use alloc::vec::Vec;

use super::ie::IE_MAX;
use super::state::Supplicant;
use crate::eapol::build::{build_key_frame_kind, KeyFrame};
use crate::eapol::parse::{HEADER_LEN, KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_SECURE};

const ZERO_NONCE: [u8; 32] = [0u8; 32];

impl Supplicant {
    pub(super) fn message2(&self, replay: &[u8; 8]) -> Option<Vec<u8>> {
        let mut kd = [0u8; 2 * IE_MAX];
        let rsne = self.own_rsne.as_slice();
        let rsnxe = self.own_rsnxe.as_slice();
        let n = rsne.len() + rsnxe.len();
        kd[..rsne.len()].copy_from_slice(rsne);
        kd[rsne.len()..n].copy_from_slice(rsnxe);
        let info = self.akm.key_version() | KEY_INFO_PAIRWISE | KEY_INFO_MIC;
        self.frame(info, replay, &self.snonce, &kd[..n])
    }

    pub(super) fn message4(&self, replay: &[u8; 8]) -> Option<Vec<u8>> {
        let info = self.akm.key_version() | KEY_INFO_PAIRWISE | KEY_INFO_MIC | KEY_INFO_SECURE;
        self.frame(info, replay, &ZERO_NONCE, &[])
    }

    pub(super) fn group2(&self, replay: &[u8; 8]) -> Option<Vec<u8>> {
        let info = self.akm.key_version() | KEY_INFO_MIC | KEY_INFO_SECURE;
        self.frame(info, replay, &ZERO_NONCE, &[])
    }

    fn frame(&self, key_info: u16, replay: &[u8; 8], nonce: &[u8; 32], kd: &[u8]) -> Option<Vec<u8>> {
        let mut out = alloc::vec![0u8; HEADER_LEN + kd.len()];
        let f = KeyFrame { key_info, replay_counter: replay, nonce, key_data: kd };
        let n = build_key_frame_kind(&mut out, self.eapol_version, self.akm.mic_kind(), &f, self.kck())?;
        out.truncate(n);
        Some(out)
    }
}
