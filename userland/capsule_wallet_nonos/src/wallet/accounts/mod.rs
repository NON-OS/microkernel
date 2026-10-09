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

/*
 * More accounts from one phrase. Account 0 is the one the phrase was made
 * or restored with; the keyring derives account i (m/44'/60'/0'/0/i) from
 * the words it keeps beside it, so the same words open the same accounts
 * here, on a phone and in any Ethereum wallet. The open account is the one
 * every screen, signature and shield request uses.
 */

pub mod file;

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
    state.accounts = Vec::from([Account { index: 0, id: state.wallet_id, address: state.address }]);
    state.account_open = 0;
    state.accounts_owed = None;
}

pub fn root_id(state: &State) -> u32 {
    state.accounts.first().map_or(state.wallet_id, |a| a.id)
}

pub fn ensure_root(state: &mut State) {
    if state.accounts.is_empty() && state.address_ready {
        reset_root(state);
    }
}

pub enum Added {
    Opened,
    /* Opened, but the store would not keep the list for the next boot. */
    OpenedUnsaved,
    /* Imported from a key: there are no words to derive from. */
    NoWords,
    /* The words beside account 0 have not been read back from the disk. */
    WordsUnread,
    Full,
    Failed,
}

/* Derive the next account and open it. */
pub fn add(state: &mut State) -> Added {
    ensure_root(state);
    if state.accounts.len() >= file::MAX_ACCOUNTS as usize {
        return Added::Full;
    }
    /* Words not read back at boot are asked for again before a derive
     * that needs them. */
    if state.words == crate::wallet::state::WORDS_UNREAD {
        words(state);
    }
    let index = state.accounts.len() as u8;
    match derive(state, index) {
        Ok(account) => {
            state.accounts.push(account);
            if switch(state, state.accounts.len() - 1) && save(state) {
                Added::Opened
            } else {
                Added::OpenedUnsaved
            }
        }
        Err(-2) if state.words == crate::wallet::state::WORDS_NONE => Added::NoWords,
        Err(-2) => Added::WordsUnread,
        Err(-28) => Added::Full,
        Err(_) => Added::Failed,
    }
}

fn derive(state: &State, index: u8) -> Result<Account, i32> {
    let now = mk_time_millis().max(0) as u64;
    let (port, owner) = (state.keyring_port, state.owner_pid);
    let id = derive_account(port, owner, root_id(state), index as u32, now, now + LIFETIME_MS)?;
    let address = wallet_address(port, owner, id)?;
    Ok(Account { index, id, address })
}

/* Open the account at `at` in the list: every screen reads it from now on.
 * False when the store would not keep it as the open one for the next boot. */
pub fn switch(state: &mut State, at: usize) -> bool {
    let Some(account) = state.accounts.get(at).copied() else { return true };
    if at as u8 == state.account_open && state.wallet_id == account.id {
        return true;
    }
    /* Nothing of the last account's shield stays: its store, its history
     * and the spend it followed belong to that account. */
    crate::wallet::shield::open::forget_account(state);
    state.wallet_id = account.id;
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

/* Open account 0's sealed words into the keyring, or say why not. A wallet
 * whose words file is missing is taken for one from a key only when it is
 * known to be one, or is older than that record. */
pub fn words(state: &mut State) {
    use crate::wallet::state::{WORDS_HELD, WORDS_NONE, WORDS_UNREAD};
    use crate::wallet::vault::Words;
    let root = root_id(state);
    /* A wallet recorded as from a key has no words, whatever a words file
     * left by an earlier wallet holds: it is never opened beside this key. */
    if crate::wallet::vault::recall_kind() == Some(true) {
        state.words = WORDS_NONE;
        return;
    }
    state.words =
        match crate::wallet::vault::recall_words(state.keyring_port, state.owner_pid, root) {
            Words::Opened => WORDS_HELD,
            Words::Absent => match crate::wallet::vault::recall_kind() {
                Some(false) => WORDS_UNREAD,
                _ => WORDS_NONE,
            },
            Words::Unread => WORDS_UNREAD,
        };
}

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
