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

//! A new wallet drawn by the keyring, its words written straight into the
//! Zeroizing backup buffer.

use crate::wallet::ipc::generate_wallet_hd;
use crate::wallet::state::State;

// Generation asks the keyring for a fresh HD wallet: real entropy becomes a
// BIP39 phrase, the phrase becomes the m/44'/60'/0'/0/0 account. The response
// carries the words exactly once for the backup screen. A draw can stall
// briefly while the entropy source reseeds, so retry a few times before
// reporting the real failure.
const ATTEMPTS: u32 = 3;

/// The new wallet's id and word count, or the keyring's last refusal.
pub(super) fn draw(state: &mut State) -> Result<(u32, u8), i32> {
    let mut last_err = 0i32;
    for _ in 0..ATTEMPTS {
        let words = &mut state.custody.backup_words;
        match generate_wallet_hd(state.keyring_port, state.owner_pid, words) {
            Ok(drawn) => return Ok(drawn),
            Err(e) => last_err = e,
        }
        for _ in 0..300 {
            nonos_libc::mk_yield();
        }
    }
    Err(last_err)
}
