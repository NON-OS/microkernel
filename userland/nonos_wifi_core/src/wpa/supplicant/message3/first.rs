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

//! Message 3 of the four-way handshake checked for authenticity (the bits
//! it carries, a MIC under the KCK of the ANonce it repeats) and taken.

use super::*;

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
        let mut ptk = [0u8; PTK_MAX];
        let len = 32 + self.pairwise.tk_len();
        let (ids, nonces) = ((&self.aa, &self.spa), (&key.nonce, &self.snonce));
        let derived = self.akm.derive_ptk_into(&self.pmk, ids, nonces, &mut ptk[..len]);
        if !derived || !verify_mic_kind(self.akm.mic_kind(), &ptk[..16], frame) {
            wipe(&mut ptk);
            return false;
        }
        self.anonce = key.nonce;
        self.ptk = ptk;
        wipe(&mut ptk);
        true
    }

    pub(in crate::wpa::supplicant) fn on_message3(
        &mut self,
        frame: &[u8],
        key: &EapolKey<'_>,
    ) -> StepOutput {
        if !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        self.take_message3(key)
    }

    /// Take an authentic message 3's keys and answer it with message 4.
    pub(in crate::wpa::supplicant) fn take_message3(&mut self, key: &EapolKey<'_>) -> StepOutput {
        let mut plain = [0u8; UNWRAPPED_MAX];
        let out = self.accept_message3(key, &mut plain);
        wipe(&mut plain);
        out
    }
}
