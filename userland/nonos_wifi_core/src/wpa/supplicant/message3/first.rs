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

//! The first message 3: checked, keyed, and answered with message 4.

use super::super::keys::{parse_group_keys, rsc_pn, wipe, UNWRAPPED_MAX};
use super::super::state::{Failure, State, Supplicant};
use super::super::step::StepOutput;
use crate::eapol::parse::EapolKey;

impl Supplicant {
    pub(in super::super) fn on_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
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
        // With management frame protection the IGTK must come too: without it
        // no deauthentication sent to all could ever be checked.
        if self.pmf && keys.igtk.is_none() {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        }
        // The downgrade check: the elements the AP signs into message 3 must be
        // the ones its beacon showed before any key existed. Every supplicant
        // has them: none is built without the beacon's elements.
        if !self.ap_rsne.matches(kd.rsne) || !self.ap_rsnxe.matches(kd.rsnxe) {
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
}
