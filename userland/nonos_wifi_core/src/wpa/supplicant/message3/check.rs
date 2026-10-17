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

//! Whether a message 3 is authentic for this handshake.

use super::super::state::Supplicant;
use crate::eapol::mic::verify_mic_kind;
use crate::eapol::parse::{EapolKey, KEY_INFO_ENCRYPTED, KEY_INFO_INSTALL, KEY_INFO_SECURE};
use crate::wipe::wipe;

/// The Key Information bits every message 3 of an RSN handshake carries.
const MESSAGE3_BITS: u16 = KEY_INFO_INSTALL | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;

/// Whether `key` carries the bits every message 3 does.
pub(in crate::wpa::supplicant) fn is_message3(key: &EapolKey<'_>) -> bool {
    key.key_info & MESSAGE3_BITS == MESSAGE3_BITS
}

impl Supplicant {
    // Whether `frame` is an authentic message 3 for this handshake: the bits
    // message 3 carries and a MIC under the KCK of the ANonce it repeats. A
    // message 1 carries no MIC, so one seen since the AP's own may be forged;
    // its ANonce is not trusted over the one message 3 is keyed with, whose
    // PTK is taken only once the MIC verifies. A frame that is not authentic
    // is dropped without ending the handshake.
    pub(super) fn authentic_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> bool {
        if !is_message3(key) {
            return false;
        }
        if key.nonce == self.anonce {
            return verify_mic_kind(self.akm.mic_kind(), self.kck(), frame);
        }
        let mut ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, &key.nonce, &self.snonce);
        if !verify_mic_kind(self.akm.mic_kind(), &ptk[..16], frame) {
            wipe(&mut ptk);
            return false;
        }
        self.anonce = key.nonce;
        self.ptk = ptk;
        wipe(&mut ptk);
        true
    }
}
