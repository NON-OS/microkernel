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

//! Pairing's steps applied: its PDUs queued on the SMP channel, its
//! question to the user answered, and its outcome acted on.

use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_smp::pairing::{Outcome, Step};
use nonos_bt_wire::l2cap::cid::SMP;

impl Session {
    pub(super) fn on_smp<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        pdu: &[u8],
        env: &mut E,
    ) -> Result<(), HciError> {
        // Once paired, an SMP PDU (a Security Request, say) asks for
        // nothing this link still needs.
        let now = env.now_ms();
        let Some(step) = self.smp.as_mut().map(|smp| smp.on_pdu(pdu, env, now)) else {
            return Ok(());
        };
        self.smp_step(host, step, env)
    }

    pub(super) fn smp_step<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        step: Step,
        env: &mut E,
    ) -> Result<(), HciError> {
        for pdu in &step.send {
            self.send(SMP, pdu);
        }
        if let Some(user) = step.user {
            return match self.ask(user, env) {
                Some(next) => self.smp_step(host, next, env),
                None => Ok(()),
            };
        }
        match step.outcome {
            Some(Outcome::Encrypt(ltk)) => self.encrypt(host, &ltk, env),
            Some(Outcome::Paired(bond)) => {
                self.paired(bond, env);
                Ok(())
            }
            Some(Outcome::Failed { by_peer, .. }) => {
                self.smp = None;
                let why = ["pairing failed", "pairing refused by the peer"];
                self.give_up(why[usize::from(by_peer)]);
                Ok(())
            }
            None => Ok(()),
        }
    }
}
