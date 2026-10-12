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
//! it carries, message 1's ANonce, a MIC under the KCK) and taken.

use super::*;

impl Supplicant {
    // Whether `frame` is an authentic message 3 for this handshake: the bits
    // message 3 carries, the ANonce of message 1, and a MIC under the KCK. A
    // frame that is not is dropped without ending the handshake.
    pub(super) fn authentic_message3(&self, frame: &[u8], key: &EapolKey<'_>) -> bool {
        key.key_info & MESSAGE3_BITS == MESSAGE3_BITS
            && key.nonce == self.anonce
            && verify_mic_kind(self.akm.mic_kind(), self.kck(), frame)
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
        let mut plain = [0u8; UNWRAPPED_MAX];
        let out = self.accept_message3(key, &mut plain);
        wipe(&mut plain);
        out
    }
}
