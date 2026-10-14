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

//! Bringing the accounts back after the vault opened at boot.

use super::account::derive;
use super::{reset_root, switch, words};
use crate::wallet::state::State;

/* After the vault opened account 0 at boot: the words beside it, then the
 * further accounts that were in use, and the one that was open. */
pub fn restore(state: &mut State) {
    reset_root(state);
    words(state);
    state.accounts_owed = crate::wallet::vault::recall_accounts();
    resume(state);
    /* Start the shield now, so the private address is ready when asked. */
    crate::wallet::shield::open::ensure(state);
}

/* Derive the accounts the list on disk names and this window does not hold
 * yet. They wait for the recovery words when those are not read back, and
 * the tick asks again (`app`), so none is dropped without a word. */
pub fn resume(state: &mut State) {
    use crate::wallet::state::WORDS_UNREAD;
    let Some((count, open)) = state.accounts_owed else { return };
    if state.words == WORDS_UNREAD {
        words(state);
    }
    while state.accounts.len() < count as usize {
        match derive(state, state.accounts.len() as u8) {
            Ok(account) => state.accounts.push(account),
            Err(-2) if state.words == crate::wallet::state::WORDS_NONE => {
                state.accounts_owed = None;
                state.status = b"the account list names further accounts, but this wallet has no recovery words";
                return;
            }
            Err(-28) => {
                state.accounts_owed = None;
                state.status = b"the keyring is full, so not every account in use came back";
                return;
            }
            Err(_) => {
                state.status = b"wallet restored; further accounts wait for the keyring and the recovery words";
                return;
            }
        }
    }
    state.accounts_owed = None;
    /* The account that was open opens again, unless the holder already
     * opened another, or work is under way for this one. */
    let busy = crate::wallet::act::running(state) || state.shield_ui.waiting.is_some();
    if state.account_open == 0 && !busy && (open as usize) < state.accounts.len() && open != 0 {
        switch(state, open as usize);
        crate::wallet::shield::open::ensure(state);
    }
}
