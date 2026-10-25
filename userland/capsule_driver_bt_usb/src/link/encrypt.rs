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

//! Encrypting the link with the key pairing derived. Once it is on,
//! pairing goes on to the keys the peer distributes (Vol 3, Part H, 3.6.1).

use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

impl Session {
    pub(super) fn encrypt<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        ltk: &[u8; 16],
        env: &mut E,
    ) -> Result<(), HciError> {
        // Pairing's last PDU goes out before the link changes keys.
        self.flush(host)?;
        match host.encrypt(self.handle, ltk) {
            Ok(()) => {}
            Err(HciError::Transport(e)) => return Err(HciError::Transport(e)),
            // Refused, dropped or unanswered: the HID service may not be
            // read on this link. A drop's Disconnection Complete stays
            // queued for the caller (`Host::encrypt`).
            Err(_) => {
                self.give_up("encryption failed");
                return Ok(());
            }
        }
        let now = env.now_ms();
        match self.smp.as_mut().map(|smp| smp.encrypted(now)) {
            Some(Ok(step)) => self.smp_step(host, step, env),
            _ => {
                self.give_up("pairing state lost");
                Ok(())
            }
        }
    }
}
