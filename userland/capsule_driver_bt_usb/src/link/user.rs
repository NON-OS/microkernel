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

//! The user's part in pairing. Only a passkey shown here can be answered:
//! this capsule has no way yet to ask the user for digits or a yes, so the
//! other two are declined at once and the peer ends pairing cleanly
//! (Vol 3, Part H, 3.5.5) instead of waiting out its timer.

use super::env::Env;
use super::session::Session;
use nonos_bt_smp::pairing::{Step, User};

impl Session {
    /// The next step, `None` when there is none until the peer answers.
    pub(super) fn ask<E: Env>(&mut self, user: User, env: &mut E) -> Option<Step> {
        let now = env.now_ms();
        let smp = self.smp.as_mut()?;
        let step = match user {
            User::ShowPasskey(n) => {
                env.show_passkey(n);
                return None;
            }
            User::EnterPasskey => smp.passkey(None, env, now),
            User::ConfirmNumber(_) => smp.confirm(false, now),
        };
        // A refusal here is a call out of turn; ending the link is the safe
        // answer to a state machine that no longer agrees with the peer.
        if step.is_err() {
            self.give_up("pairing state lost");
        }
        step.ok()
    }
}
