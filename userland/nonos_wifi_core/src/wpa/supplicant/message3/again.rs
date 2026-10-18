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

//! A retransmitted message 3 after the keys are in: it is answered with
//! message 4 again but nothing is reinstalled, so a replay cannot reset a
//! nonce.

use super::*;

impl Supplicant {
    pub(in crate::wpa::supplicant) fn on_message3_again(
        &mut self,
        frame: &[u8],
        key: &EapolKey<'_>,
    ) -> StepOutput {
        // A repeat of the message 3 already taken: the same ANonce, never a
        // new PTK once connected (a new one is a rekey's, `rekey.rs`).
        if key.nonce != self.anonce || !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        StepOutput::reply(self.message4(&key.replay_counter))
    }
}
