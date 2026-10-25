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
 * The recovery words of this wallet, for the shield service and nobody
 * else: the keyring answers only the wallet that owns the account, with
 * the word indices, and they are spelled out here and zeroed on drop.
 */

use alloc::string::String;

use nonos_hd::ENGLISH_WORDLIST;
use zeroize::Zeroize;

use super::call::keyring_call;
use super::constants::{HDR_LEN, OP_SHIELD_MATERIAL};

const MAX_WORDS: usize = 24;

#[path = "shield_words_text.rs"]
mod text;
pub use text::Words;

/* Err(-2) means the account has no words: it was imported from a key. */
pub fn shield_words(port: u32, owner_pid: u32, wallet_id: u32) -> Result<Words, i32> {
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&owner_pid.to_le_bytes());
    payload[4..].copy_from_slice(&wallet_id.to_le_bytes());
    let mut rx = keyring_call(port, OP_SHIELD_MATERIAL, &payload, 1 + 2 * MAX_WORDS)?;
    let body = rx.get(HDR_LEN..).unwrap_or(&[]);
    let count = body.first().copied().unwrap_or(0) as usize;
    let fits = matches!(count, 12 | 15 | 18 | 21 | 24) && body.len() > 2 * count;
    /* The longest word is eight letters: the text never grows past this, so
     * no copy of it is left behind in a reallocation. */
    let mut text = String::with_capacity(count * 9);
    let mut ok = fits;
    if fits {
        for i in 0..count {
            let at = 1 + 2 * i;
            let index = u16::from_le_bytes([body[at], body[at + 1]]) as usize;
            match ENGLISH_WORDLIST.get(index) {
                Some(word) => {
                    if i > 0 {
                        text.push(' ');
                    }
                    text.push_str(word);
                }
                None => ok = false,
            }
        }
    }
    let words = Words(text);
    rx.zeroize();
    if ok {
        Ok(words)
    } else {
        Err(-22)
    }
}
