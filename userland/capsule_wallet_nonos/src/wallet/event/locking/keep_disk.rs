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

//! The records of a new wallet, on this machine's disk.

use super::keep_plan::Records;
use crate::wallet::state::State;
use crate::wallet::vault::Unsealed;
use crate::wallet::vault::{forget_vault, forget_words, remember, remember_kind, remember_words};

/// The records of the wallet `state` holds, on this machine's disk.
pub(super) struct Disk<'a> {
    pub(super) state: &'a State,
}

impl Records for Disk<'_> {
    fn clear_vault(&mut self) -> Result<(), Unsealed> {
        forget_vault()
    }

    fn kind(&mut self, from_key: bool) -> Result<(), Unsealed> {
        remember_kind(from_key)
    }

    fn words(&mut self) -> Result<(), Unsealed> {
        remember_words(self.state.keyring_port, self.state.owner_pid, self.state.wallet_id)
    }

    fn clear_words(&mut self) -> Result<(), Unsealed> {
        forget_words()
    }

    fn accounts(&mut self) -> Result<(), Unsealed> {
        let s = self.state;
        let count = s.accounts.len().max(1) as u8;
        crate::wallet::vault::remember_accounts(count, s.account_open.min(count - 1))
    }

    fn vault(&mut self) -> Result<(), Unsealed> {
        remember(self.state.keyring_port, self.state.owner_pid, self.state.wallet_id)
    }
}
