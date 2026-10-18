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

//! Adding the next account from the phrase.

use super::account::derive;
use super::file;
use super::{ensure_root, save, switch, words};
use crate::wallet::state::State;

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
    if state.acct.list.len() >= file::MAX_ACCOUNTS as usize {
        return Added::Full;
    }
    /* Words not read back at boot are asked for again before a derive
     * that needs them. */
    if state.acct.words == crate::wallet::state::WORDS_UNREAD {
        words(state);
    }
    let index = state.acct.list.len() as u8;
    match derive(state, index) {
        Ok(account) => {
            state.acct.list.push(account);
            if switch(state, state.acct.list.len() - 1) && save(state) {
                Added::Opened
            } else {
                Added::OpenedUnsaved
            }
        }
        Err(-2) if state.acct.words == crate::wallet::state::WORDS_NONE => Added::NoWords,
        Err(-2) => Added::WordsUnread,
        Err(-28) => Added::Full,
        Err(_) => Added::Failed,
    }
}
