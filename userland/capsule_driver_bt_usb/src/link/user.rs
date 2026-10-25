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

//! The user's part in pairing. A passkey to show is logged and given to
//! NBTC; digits to type and numbers to confirm wait for the user's answer,
//! which comes through NBTC from Settings (`answered.rs`).

use super::desk::Asked;
use super::env::Env;
use super::session::Session;
use nonos_bt_smp::pairing::User;

impl Session {
    /// What pairing asks of the user, passed on. The step that follows
    /// comes from the peer, or from the user's answer (`answered.rs`).
    pub(super) fn ask<E: Env>(&mut self, user: User, env: &mut E) {
        match user {
            User::ShowPasskey(n) => env.show_passkey(n),
            User::EnterPasskey => self.wait_on(Asked::Passkey, env),
            User::ConfirmNumber(n) => self.wait_on(Asked::Number(n), env),
        }
    }

    /// Pairing waits on the user; the SMP timer bounds the wait (Vol 3,
    /// Part H, 3.4), and `answered` takes the answer.
    fn wait_on<E: Env>(&mut self, asked: Asked, env: &mut E) {
        self.waiting = Some(asked);
        env.asked(asked);
    }
}
