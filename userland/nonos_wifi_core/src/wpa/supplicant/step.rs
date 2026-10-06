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

//! Drive one EAPOL-Key frame through the handshake (IEEE Std 802.11-2020,
//! 12.7.6 and 12.7.7). Every frame must be an RSN descriptor with Key Ack set,
//! no Request or Error, the key descriptor version of the negotiated AKM, and
//! a replay counter above the last one whose MIC verified; anything else is
//! dropped unanswered. Message 1 (no MIC) yields message 2, also when the AP
//! repeats message 1 because message 2 was lost. Message 3 must carry Install,
//! Secure and Encrypted Key Data, verify under the KCK, repeat the ANonce, and
//! hold the beacon's RSNE; it yields message 4. Once connected, a repeated
//! message 3 is answered with message 4 again without reinstalling any key,
//! and a group message 1 delivers the next GTK and yields group message 2.
//!
//! A frame that fails its MIC is dropped, not treated as the end of the
//! handshake: an unauthenticated frame must not be able to tear the join down.
//! Only an authentic message that is unacceptable (a downgraded RSNE, key data
//! without a valid group key) fails it.

use alloc::vec::Vec;

use super::state::{State, Supplicant};
use crate::eapol::parse::{
    parse, EapolKey, DESCRIPTOR_RSN, KEY_INFO_ACK, KEY_INFO_ERROR, KEY_INFO_INDEX_MASK,
    KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_REQUEST, KEY_INFO_VERSION_MASK,
};

/// What `step` produced: a frame to transmit, and whether the group key
/// changed. A frame is returned for messages 2 and 4 and for group message 2;
/// `None` means the input was not a message the supplicant acts on, or it was
/// dropped (check `Supplicant::state` for `Failed`).
pub struct StepOutput {
    pub reply: Option<Vec<u8>>,
    /// A group key handshake delivered a new group key: the caller installs
    /// `Supplicant::gtk` at `Supplicant::gtk_id`.
    pub new_group_key: bool,
}

impl StepOutput {
    pub(super) fn none() -> Self {
        Self { reply: None, new_group_key: false }
    }

    pub(super) fn reply(frame: Option<Vec<u8>>) -> Self {
        Self { reply: frame, new_group_key: false }
    }
}

impl Supplicant {
    pub fn step(&mut self, frame: &[u8]) -> StepOutput {
        if self.state == State::Failed {
            return StepOutput::none();
        }
        let Some(key) = parse(frame) else {
            return StepOutput::none();
        };
        let info = key.key_info;
        if key.descriptor_type != DESCRIPTOR_RSN
            || info & KEY_INFO_ACK == 0
            || info & (KEY_INFO_REQUEST | KEY_INFO_ERROR) != 0
            || info & KEY_INFO_VERSION_MASK != self.akm.key_version()
            || !self.replay_is_fresh(&key.replay_counter)
        {
            return StepOutput::none();
        }
        // Reply in the 802.1X version the authenticator spoke.
        self.eapol_version = match frame[0] {
            v @ 1..=3 => v,
            _ => crate::eapol::build::EAPOL_VERSION_DEFAULT,
        };
        let has_mic = info & KEY_INFO_MIC != 0;
        if info & KEY_INFO_PAIRWISE != 0 {
            if info & KEY_INFO_INDEX_MASK != 0 {
                return StepOutput::none();
            }
            return match (self.state, has_mic) {
                (State::Start | State::PtkDerived, false) => self.on_message1(&key),
                (State::PtkDerived, true) => self.on_message3(frame, &key),
                (State::Connected, true) => self.on_message3_again(frame, &key),
                _ => StepOutput::none(),
            };
        }
        if has_mic && self.state == State::Connected {
            return self.on_group1(frame, &key);
        }
        StepOutput::none()
    }

    // A replay counter must exceed the last one a verified MIC vouched for.
    // Message 1 carries no MIC, so it never raises the floor; it is held only
    // to the floor already set.
    fn replay_is_fresh(&self, rc: &[u8; 8]) -> bool {
        match &self.rx_replay {
            Some(last) => u64::from_be_bytes(*rc) > u64::from_be_bytes(*last),
            None => true,
        }
    }

    // Message 1 -> derive the PTK, answer with message 2 (SNonce and the
    // station's RSNE, MIC keyed by the fresh KCK). A repeated message 1 before
    // message 3 is answered the same way, with the ANonce it carries: the AP
    // repeats it when message 2 was lost, and its replay counter advanced, so
    // the earlier message 2 would be discarded.
    fn on_message1(&mut self, key: &EapolKey<'_>) -> StepOutput {
        self.anonce = key.nonce;
        self.ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, &self.anonce, &self.snonce);
        match self.message2(&key.replay_counter) {
            Some(frame) => {
                self.state = State::PtkDerived;
                StepOutput::reply(Some(frame))
            }
            None => {
                self.fail(super::state::Failure::Internal);
                StepOutput::none()
            }
        }
    }
}
