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

//! The first checks of a tick: boot, the idle lock, owed accounts, locked.

use super::{mk_uptime_ms, Wallet};

// How often accounts the list on disk names, and that did not come back at
// boot, are asked for again.
const ACCOUNTS_RETRY_MS: i64 = 5_000;

impl Wallet {
    /// The checks that come first on a tick. Some when the tick ends here.
    pub(super) fn tick_gate(&mut self) -> Option<bool> {
        if !self.ready {
            self.ready = true;
            return Some(self.hydrate_again());
        }
        /* A vault or keyring that did not answer at boot is asked again,
         * waiting longer each time, until it settles. */
        if self.hydrate_again() {
            return Some(true);
        }
        if !self.state.address_ready {
            return Some(false);
        }
        /* Left alone: the window locks by itself (`event::idle_lock`). The
         * clock starts at the first tick with an account, not at boot. */
        let now = mk_uptime_ms();
        if self.last_active == 0 {
            self.last_active = now;
        }
        let sending = self.state.action.is_some();
        if super::event::locking::idle_lock::due(now, self.last_active, self.state.locked, sending) {
            let _ = super::event::lock(&mut self.state);
            self.state.status = b"locked after five minutes without a press";
            return Some(true);
        }
        /* Accounts the list on disk names that did not come back at boot
         * are asked for again every few seconds until they do. */
        if self.state.accounts_owed.is_some() {
            let now = mk_uptime_ms();
            if now >= self.state.accounts_retry_at {
                self.state.accounts_retry_at = now + ACCOUNTS_RETRY_MS;
                super::accounts::resume(&mut self.state);
                return Some(true);
            }
        }
        /* Locked: what is going out finishes, and nothing new begins. */
        if self.state.locked && self.state.action.is_none() {
            return Some(false);
        }
        None
    }
}
