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

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::crypto::{fresh_key, fresh_nonce, TAG_LEN};
use super::limits::{MAX_FILES, MAX_STORE_BYTES};
use super::types::{File, Store, StoreError};

impl Store {
    pub const fn new() -> Self {
        Self { files: BTreeMap::new() }
    }

    pub fn contains(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }

    pub fn ensure(&mut self, path: &str) -> Result<(), StoreError> {
        if self.files.contains_key(path) {
            return Ok(());
        }
        if self.files.len() >= MAX_FILES {
            return Err(StoreError::Full);
        }
        let key = fresh_key()?;
        let nonce = fresh_nonce()?;
        self.files.insert(String::from(path), File { key, nonce, ciphertext: Vec::new() });
        Ok(())
    }

    /// Whether `path` may hold `len` plaintext bytes alongside every other
    /// file as it stands. Checked before anything is opened or resized.
    pub(super) fn room_for(&self, path: &str, len: usize) -> Result<(), StoreError> {
        let others: usize = self
            .files
            .iter()
            .filter(|(name, _)| name.as_str() != path)
            .map(|(_, f)| f.ciphertext.len().saturating_sub(TAG_LEN))
            .sum();
        match others.checked_add(len) {
            Some(total) if total <= MAX_STORE_BYTES => Ok(()),
            _ => Err(StoreError::Full),
        }
    }

    /// The plaintext length `path` holds now, without opening it.
    pub(super) fn held(&self, path: &str) -> usize {
        self.files.get(path).map_or(0, |f| f.ciphertext.len().saturating_sub(TAG_LEN))
    }
}
