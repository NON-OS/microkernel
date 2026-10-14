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

//! Reading the keyring and the vault again until boot settles.

use super::{hydrate, mk_uptime_ms, Wallet};

// The first wait between two reads of a vault or keyring that did not
// answer at boot, doubled each time up to the last.
const HYDRATE_FIRST_MS: i64 = 1_000;
const HYDRATE_MAX_MS: i64 = 15_000;

impl Wallet {
    /// Whether boot has not settled: the keyring not found yet, or no wallet
    /// and the vault not read to an answer. A vault that answered, opened
    /// or not, is never read again in this window.
    fn unsettled(&self) -> bool {
        let s = &self.state;
        s.keyring_port == 0 || s.owner_pid == 0 || (s.wallet_id == 0 && !s.vault_restore_tried)
    }

    /// Read the keyring and the vault again once the wait is over. True when
    /// it ran.
    pub(super) fn hydrate_again(&mut self) -> bool {
        let now = mk_uptime_ms();
        if !self.unsettled() || now < self.hydrate_at {
            return false;
        }
        hydrate(&mut self.state);
        let wait = (HYDRATE_FIRST_MS << self.hydrate_tries.min(4)).min(HYDRATE_MAX_MS);
        self.hydrate_tries = self.hydrate_tries.saturating_add(1);
        self.hydrate_at = now + wait;
        true
    }

    /// Whether network work is under way, a refresh or a press's.
    pub(super) fn stepping(&self) -> bool {
        self.state.net.job.is_some() || self.state.action.is_some()
    }
}
