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

use alloc::vec;
use alloc::vec::Vec;

use super::crypto::{fresh_nonce, open, seal, TAG_LEN};
use super::limits::MAX_FILE_BYTES;
use super::types::{Store, StoreError};

impl Store {
    pub fn truncate(&mut self, path: &str, length: usize) -> Result<(), StoreError> {
        if !self.files.contains_key(path) {
            return Err(StoreError::NotFound);
        }
        if length > MAX_FILE_BYTES {
            return Err(StoreError::TooLarge);
        }
        if length > self.held(path) {
            self.room_for(path, length)?;
        }
        let f = self.files.get_mut(path).ok_or(StoreError::NotFound)?;
        let mut plain: Vec<u8> = if f.ciphertext.is_empty() {
            Vec::new()
        } else {
            let plain_len =
                f.ciphertext.len().checked_sub(TAG_LEN).ok_or(StoreError::CryptoFailure)?;
            let mut buf = vec![0u8; plain_len];
            let n = open(&f.key, &f.nonce, &f.ciphertext, &mut buf)?;
            buf.truncate(n);
            buf
        };
        plain.resize(length, 0);
        f.nonce = fresh_nonce()?;
        if plain.is_empty() {
            f.ciphertext = Vec::new();
            return Ok(());
        }
        let mut cipher = vec![0u8; plain.len() + TAG_LEN];
        let n = seal(&f.key, &f.nonce, &plain, &mut cipher)?;
        cipher.truncate(n);
        f.ciphertext = cipher;
        Ok(())
    }
}
