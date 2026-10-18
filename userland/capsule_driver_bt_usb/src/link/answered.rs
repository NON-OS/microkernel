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

//! What the user said, acted on between two turns of the link: the answer
//! pairing waits for, and a bond forgotten while its link runs.

use super::desk::{Answer, Asked};
use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

/// Why a link ends whose bond the user forgot.
pub const FORGOTTEN: &str = "its bond was forgotten";
/// Why a link ends when the radio is turned off; the bond is kept (E-72).
pub const OFF: &str = "the radio was turned off";

impl Session {
    pub(super) fn answered<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        now_ms: u64,
        env: &mut E,
    ) -> Result<(), HciError> {
        if env.hang_up() {
            self.give_up(FORGOTTEN);
        }
        let Some(asked) = self.waiting else { return Ok(()) };
        let Some(answer) = env.answer() else { return Ok(()) };
        self.waiting = None;
        let Some(smp) = self.smp.as_mut() else { return Ok(()) };
        // A wrong kind of answer declines: the pairing it was meant for
        // cannot use it, and the peer ends pairing cleanly (Vol 3, Part H,
        // 3.5.5) instead of waiting out its timer.
        let step = match (asked, answer) {
            (Asked::Passkey, Answer::Passkey(v)) => smp.passkey(Some(v), env, now_ms),
            (Asked::Passkey, _) => smp.passkey(None, env, now_ms),
            (Asked::Number(_), Answer::Same(same)) => smp.confirm(same, now_ms),
            (Asked::Number(_), _) => smp.confirm(false, now_ms),
        };
        match step {
            Ok(step) => self.smp_step(host, step, env),
            Err(_) => {
                self.give_up("pairing state lost");
                Ok(())
            }
        }
    }
}
