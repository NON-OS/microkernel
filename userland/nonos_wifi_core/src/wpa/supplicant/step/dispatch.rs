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

//! Every EAPOL-Key frame enters here and is routed by message, after the
//! replay counter check.

use super::*;

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
}
