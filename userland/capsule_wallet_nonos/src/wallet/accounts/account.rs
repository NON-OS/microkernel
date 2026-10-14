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

//! An account in the list, account 0 and deriving the next one.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use crate::wallet::ipc::{derive_account, wallet_address};
use crate::wallet::state::State;

/* The keyring's lifetime for an account opened in this session: a year, as
 * the vault's own. */
const LIFETIME_MS: u64 = 31_536_000_000;

#[derive(Clone, Copy)]
pub struct Account {
    pub index: u8,
    pub id: u32,
    pub address: [u8; 20],
}

/* A new phrase or key became account 0: forget the old list. */
pub fn reset_root(state: &mut State) {
    state.accounts =
        Vec::from([Account { index: 0, id: state.vault.wallet_id, address: state.address }]);
    state.account_open = 0;
    state.accounts_owed = None;
}

pub fn root_id(state: &State) -> u32 {
    state.accounts.first().map_or(state.vault.wallet_id, |a| a.id)
}

pub fn ensure_root(state: &mut State) {
    if state.accounts.is_empty() && state.address_ready {
        reset_root(state);
    }
}

pub(super) fn derive(state: &State, index: u8) -> Result<Account, i32> {
    let now = mk_time_millis().max(0) as u64;
    let (port, owner) = (state.vault.keyring_port, state.vault.owner_pid);
    let id = derive_account(port, owner, root_id(state), index as u32, now, now + LIFETIME_MS)?;
    let address = wallet_address(port, owner, id)?;
    Ok(Account { index, id, address })
}
