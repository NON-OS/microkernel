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

//! What the run does with each step: PDUs go to the responder, and the
//! users and the controller answer what the step asks.

use super::run::Run;
use nonos_bt_smp::pairing::{Outcome, Step, User};

impl Run {
    pub(super) fn take(&mut self, step: Step) {
        self.sent.extend(step.send.iter().cloned());
        self.to_peer.extend(step.send);
        if let Some(user) = step.user {
            self.users.push(user);
            self.answer(user);
        }
        if let Some(outcome) = step.outcome {
            self.outcomes.push(outcome);
            if let Outcome::Encrypt(_) = outcome {
                let next = self.init.encrypted(self.now).unwrap();
                self.to_init.extend(self.peer.keys());
                self.take(next);
            }
        }
    }

    fn answer(&mut self, user: User) {
        let next = match user {
            // The responder's user types what ours is shown.
            User::ShowPasskey(k) => {
                self.peer.passkey = k;
                return;
            }
            User::EnterPasskey => {
                let typed = self.typed.unwrap_or(Some(self.peer.passkey));
                self.init.passkey(typed, &mut self.rng, self.now)
            }
            User::ConfirmNumber(v) => {
                let same = self.agree.unwrap_or(v == self.peer.number());
                self.init.confirm(same, self.now)
            }
        };
        self.take(next.unwrap());
    }

    /// The initiator's PDUs with this code.
    pub fn sent_with(&self, code: u8) -> usize {
        self.sent.iter().filter(|p| p[0] == code).count()
    }
}
