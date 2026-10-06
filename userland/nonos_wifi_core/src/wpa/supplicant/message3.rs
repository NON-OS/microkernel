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

//! Message 3 of the four-way handshake (IEEE Std 802.11-2020, 12.7.6.4), the
//! first time and repeated. The first verifies, checks the RSNE (and RSNXE)
//! against the beacon, takes the group keys, and answers with message 4. A
//! repeat after that (the AP did not get message 4) is answered with message 4
//! again and changes nothing else: reinstalling the pairwise key on a repeat is
//! the key-reinstallation attack, so the keys stay as they are.

use super::keys::{parse_group_keys, rsc_pn, wipe, UNWRAPPED_MAX};
use super::state::{Failure, State, Supplicant};
use super::step::StepOutput;
use crate::eapol::mic::verify_mic_kind;
use crate::eapol::parse::{EapolKey, KEY_INFO_ENCRYPTED, KEY_INFO_INSTALL, KEY_INFO_SECURE};

/// The Key Information bits every message 3 of an RSN handshake carries.
const MESSAGE3_BITS: u16 = KEY_INFO_INSTALL | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;

impl Supplicant {
    // Whether `frame` is an authentic message 3 for this handshake: the bits
    // message 3 carries, the ANonce of message 1, and a MIC under the KCK. A
    // frame that is not is dropped without ending the handshake.
    fn authentic_message3(&self, frame: &[u8], key: &EapolKey<'_>) -> bool {
        key.key_info & MESSAGE3_BITS == MESSAGE3_BITS
            && key.nonce == self.anonce
            && verify_mic_kind(self.akm.mic_kind(), self.kck(), frame)
    }

    pub(super) fn on_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        if !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        let mut plain = [0u8; UNWRAPPED_MAX];
        let out = self.accept_message3(key, &mut plain);
        wipe(&mut plain);
        out
    }

    fn accept_message3(&mut self, key: &EapolKey<'_>, plain: &mut [u8; UNWRAPPED_MAX]) -> StepOutput {
        let Some(n) = self.unwrap_key_data(key.key_data, plain) else {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        };
        let Some((kd, mut keys)) = parse_group_keys(&plain[..n]) else {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        };
        // The downgrade check: the elements the AP signs into message 3 must be
        // the ones its beacon showed before any key existed.
        if self.ap_known && (!self.ap_rsne.matches(kd.rsne) || !self.ap_rsnxe.matches(kd.rsnxe)) {
            self.fail(Failure::IeMismatch);
            return StepOutput::none();
        }
        keys.rsc = rsc_pn(&key.key_rsc);
        self.store_group_keys(&keys);
        match self.message4(&key.replay_counter) {
            Some(frame) => {
                self.state = State::Connected;
                StepOutput::reply(Some(frame))
            }
            None => {
                self.fail(Failure::Internal);
                StepOutput::none()
            }
        }
    }

    pub(super) fn on_message3_again(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        if !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        StepOutput::reply(self.message4(&key.replay_counter))
    }
}
