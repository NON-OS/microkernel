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

//! The words, or an imported account's key, handed to the service and zeroed.

use zeroize::Zeroize;

use super::*;

/* Not held open: the words go to the service, or an imported account's key. */
pub(super) fn from_words(state: &mut State, account: &str) {
    state.shield_ui.opened = false;
    /* Words not read yet are read again first; the key never stands in. */
    if state.words == WORDS_UNREAD {
        crate::wallet::accounts::words(state);
    }
    let (port, owner, wallet) = (state.keyring_port, state.owner_pid, state.wallet_id);
    match shield_words(port, owner, wallet) {
        Ok(words) => {
            if !super::super::job::start(state, OP_OPEN_WORDS, &[account, words.text()]) {
                slow(state);
            }
        }
        /* Only a wallet from a key opens from it: its nox1 is the key's.
         * A wallet with words opened from its key would get an address
         * its words could not restore. */
        Err(NO_KEYRING) if state.words == WORDS_NONE => open_key(state, account),
        Err(NO_KEYRING) if state.words == WORDS_UNREAD => {
            later(state);
            state.shield_ui.failure = Some(String::from(WORDS_UNREAD_YET));
        }
        Err(KEYRING_SLOW) => keyring_late(state),
        Err(_) => state.shield_ui.failure = Some(String::from(NO_WORDS)),
    }
}

fn open_key(state: &mut State, account: &str) {
    let mut key = [0u8; 32];
    match export_secret(state.keyring_port, state.owner_pid, state.wallet_id, &mut key) {
        Ok(()) => {}
        Err(KEYRING_SLOW) => return keyring_late(state),
        Err(_) => {
            state.shield_ui.failure = Some(String::from(NO_WORDS));
            return;
        }
    }
    let mut text = String::with_capacity(66);
    text.push_str("0x");
    for b in key.iter() {
        text.push(char::from(HEX[(b >> 4) as usize]));
        text.push(char::from(HEX[(b & 15) as usize]));
    }
    if !super::super::job::start(state, OP_OPEN_KEY, &[account, &text]) {
        slow(state);
    }
    text.zeroize();
    key.zeroize();
}

const HEX: &[u8; 16] = b"0123456789abcdef";
