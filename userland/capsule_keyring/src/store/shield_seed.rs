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
//! An HD wallet's recovery words, kept for its shield.
//!
//! The shield runs the NOX Shield wallet core the phone apps run, which opens
//! its account from the words themselves, so the same words open the same
//! shield account and the same 0x account here and on a phone. The keyring
//! keeps the word indices beside the account key they made, under the same
//! owner, and gives them only to that owner.

use super::types::{KeyType, Store, StoreError};

/// The most words a phrase has.
pub const MAX_WORDS: usize = 24;
/// The eth key's id (4, little endian), the word count (1), the indices (2 each).
const DATA_LEN: usize = 4 + 1 + 2 * MAX_WORDS;

/// A phrase as indices into the BIP-39 list.
pub struct Words {
    pub count: u8,
    pub indices: [u16; MAX_WORDS],
}

impl Drop for Words {
    fn drop(&mut self) {
        for w in self.indices.iter_mut() {
            unsafe { core::ptr::write_volatile(w, 0) };
        }
    }
}

impl Store {
    /// Keep `indices` for the wallet whose account key is `eth_id`.
    pub fn put_shield_words(
        &mut self,
        eth_id: u32,
        indices: &[u16],
        owner_pid: u32,
        now: u64,
        expires_at: u64,
    ) -> Result<u32, StoreError> {
        if !matches!(indices.len(), 12 | 15 | 18 | 21 | 24) {
            return Err(StoreError::InvalidArgument);
        }
        // A wallet has one phrase: a second is refused rather than shadowing it.
        if self.find_shield(eth_id, owner_pid).is_some() {
            return Err(StoreError::InvalidArgument);
        }
        let mut data = [0u8; DATA_LEN];
        data[..4].copy_from_slice(&eth_id.to_le_bytes());
        data[4] = indices.len() as u8;
        for (i, w) in indices.iter().enumerate() {
            data[5 + 2 * i..7 + 2 * i].copy_from_slice(&w.to_le_bytes());
        }
        let out = self.store(KeyType::ShieldSeed, &data, owner_pid, now, expires_at);
        super::wipe::secure_wipe(&mut data);
        out
    }

    fn find_shield(&self, eth_id: u32, owner_pid: u32) -> Option<u32> {
        self.entries
            .iter()
            .find(|(_, e)| {
                e.key_type == KeyType::ShieldSeed
                    && e.owner_pid == owner_pid
                    && e.data.len() == DATA_LEN
                    && e.data[..4] == eth_id.to_le_bytes()
            })
            .map(|(id, _)| *id)
    }

    /// The words of the wallet `eth_id`, for its owner only. NotFound for a
    /// wallet that has none: one imported from a private key.
    pub fn shield_words(&mut self, eth_id: u32, caller_pid: u32) -> Result<Words, StoreError> {
        // The account key must exist, be the caller's and be usable now.
        let mut key = self.eth_secret(eth_id, caller_pid)?;
        super::wipe::secure_wipe(&mut key);
        let id = self.find_shield(eth_id, caller_pid).ok_or(StoreError::NotFound)?;
        let entry = self.entries.get_mut(&id).ok_or(StoreError::NotFound)?;
        if entry.locked {
            return Err(StoreError::Locked);
        }
        let mut words = Words { count: entry.data[4], indices: [0; MAX_WORDS] };
        for i in 0..(words.count as usize).min(MAX_WORDS) {
            words.indices[i] = u16::from_le_bytes([entry.data[5 + 2 * i], entry.data[6 + 2 * i]]);
        }
        entry.use_count = entry.use_count.saturating_add(1);
        Ok(words)
    }
}
