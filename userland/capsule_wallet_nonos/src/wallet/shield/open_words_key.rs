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

//! An imported account's key, handed to the service as hex and zeroed.

use zeroize::Zeroize;

use super::super::*;

pub(super) fn open_key(state: &mut State, account: &str) {
    let mut key = [0u8; 32];
    match export_secret(
        state.vault.keyring_port,
        state.vault.owner_pid,
        state.vault.wallet_id,
        &mut key,
    ) {
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
    if !crate::wallet::shield::job::start(state, OP_OPEN_KEY, &[account, &text]) {
        slow(state);
    }
    text.zeroize();
    key.zeroize();
}

const HEX: &[u8; 16] = b"0123456789abcdef";
