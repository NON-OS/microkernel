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

//! Opening an account and keeping the list across reboots.

use crate::wallet::state::State;

/* Open the account at `at` in the list: every screen reads it from now on.
 * False when the store would not keep it as the open one for the next boot. */
pub fn switch(state: &mut State, at: usize) -> bool {
    let Some(account) = state.accounts.get(at).copied() else { return true };
    if at as u8 == state.account_open && state.vault.wallet_id == account.id {
        return true;
    }
    /* Nothing of the last account's shield stays: its store, its history
     * and the spend it followed belong to that account. */
    crate::wallet::shield::open::forget_account(state);
    state.vault.wallet_id = account.id;
    state.address = account.address;
    state.account_open = at as u8;
    crate::wallet::state::forget_live(state);
    save(state)
}

/* Keep the list across reboots, beside the vault. False when the store
 * would not take it. */
pub fn save(state: &State) -> bool {
    let count = state.accounts.len().max(1) as u8;
    crate::wallet::vault::remember_accounts(count, state.account_open.min(count - 1)).is_ok()
}
