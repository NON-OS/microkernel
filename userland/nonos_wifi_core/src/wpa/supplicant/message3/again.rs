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

//! A repeated message 3: answered again, and nothing reinstalled.

use super::super::state::Supplicant;
use super::super::step::StepOutput;
use crate::eapol::parse::EapolKey;

impl Supplicant {
    pub(in super::super) fn on_message3_again(
        &mut self,
        frame: &[u8],
        key: &EapolKey<'_>,
    ) -> StepOutput {
        if !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        StepOutput::reply(self.message4(&key.replay_counter))
    }
}
