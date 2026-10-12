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

//! The two timers a link runs on: pairing's 30 s (Vol 3, Part H, 3.4)
//! and the ATT transaction's 30 s (Vol 3, Part F, 3.3.3). Either running
//! out ends the link, so a silent peer never holds it.

use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};

impl Session {
    pub(super) fn timers<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        now_ms: u64,
        env: &mut E,
    ) -> Result<(), HciError> {
        self.answered(host, now_ms, env)?;
        if let Some(step) = self.smp.as_mut().map(|smp| smp.poll(now_ms)) {
            self.smp_step(host, step, env)?;
        }
        if let Some(step) = self.hid.as_mut().map(|hid| hid.poll(now_ms)) {
            self.hid_step(step, env);
        }
        Ok(())
    }
}
