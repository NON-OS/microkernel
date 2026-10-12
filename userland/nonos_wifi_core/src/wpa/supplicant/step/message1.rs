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

//! Message 1 carries the ANonce; the PTK is derived from it here.

use super::*;

impl Supplicant {
    // Message 1 -> derive the PTK, answer with message 2 (SNonce and the
    // station's RSNE, MIC keyed by the fresh KCK). A repeated message 1 before
    // message 3 is answered the same way, with the ANonce it carries: the AP
    // repeats it when message 2 was lost, and its replay counter advanced, so
    // the earlier message 2 would be discarded.
    pub(super) fn on_message1(&mut self, key: &EapolKey<'_>) -> StepOutput {
        self.anonce = key.nonce;
        self.ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, &self.anonce, &self.snonce);
        match self.message2(&key.replay_counter) {
            Some(frame) => {
                self.state = State::PtkDerived;
                StepOutput::reply(Some(frame))
            }
            None => {
                self.fail(super::super::state::Failure::Internal);
                StepOutput::none()
            }
        }
    }
}
