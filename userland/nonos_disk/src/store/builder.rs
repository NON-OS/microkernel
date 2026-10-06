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

//! Building the store: the full table first, then each payload on a sector
//! boundary with its entry filled in as it lands, the header last. The
//! payloads start past a table of `MAX_ENTRIES`, where vfs appends, so a
//! table that grows later never runs into one.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_disk_map::{digest16, streamed, valid_name, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES};
use nonos_disk_map::{MAX_TOTAL_BYTES, STREAMED_MAX_BYTES};
use nonos_disk_map::{NAME_LEN, SECTOR_SIZE, STORE_BASE_LBA, TOC_SPAN};

use super::error::StoreError;

pub struct StoreBuilder {
    pub(super) bytes: Vec<u8>,
    pub(super) names: Vec<String>,
    pub(super) payload: u64,
    /// Bytes of the entries vfs streams rather than loads, counted apart.
    pub(super) streamed: u64,
}

impl Default for StoreBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreBuilder {
    pub fn new() -> StoreBuilder {
        StoreBuilder { bytes: alloc::vec![0u8; TOC_SPAN], names: Vec::new(), payload: 0, streamed: 0 }
    }

    /// Whether `files` more files of `bytes` in all still fit what vfs loads.
    pub fn has_room(&self, files: usize, bytes: u64) -> bool {
        self.names.len() + files <= MAX_ENTRIES
            && self.payload.saturating_add(bytes) <= MAX_TOTAL_BYTES
    }

    /// Add `data` as `name`, with the digest vfs checks it against.
    pub fn add(&mut self, name: &str, data: &[u8]) -> Result<(), StoreError> {
        if !valid_name(name) {
            return Err(StoreError::BadName);
        }
        if self.names.iter().any(|n| n == name) {
            return Err(StoreError::Duplicate);
        }
        let len = data.len() as u64;
        let fits = if streamed(name) {
            self.names.len() < MAX_ENTRIES && self.streamed.saturating_add(len) <= STREAMED_MAX_BYTES
        } else {
            self.has_room(1, len)
        };
        if !fits {
            return Err(StoreError::Full);
        }
        let offset = STORE_BASE_LBA * SECTOR_SIZE as u64 + self.bytes.len() as u64;
        let at = HEADER_LEN + ENTRY_LEN * self.names.len();
        let e = &mut self.bytes[at..at + ENTRY_LEN];
        e[..name.len()].copy_from_slice(name.as_bytes());
        e[NAME_LEN..NAME_LEN + 8].copy_from_slice(&offset.to_le_bytes());
        e[NAME_LEN + 8..NAME_LEN + 16].copy_from_slice(&(data.len() as u64).to_le_bytes());
        e[NAME_LEN + 16..NAME_LEN + 32].copy_from_slice(&digest16(data));
        self.bytes.extend_from_slice(data);
        self.bytes.resize(self.bytes.len().div_ceil(SECTOR_SIZE) * SECTOR_SIZE, 0);
        self.names.push(String::from(name));
        if streamed(name) {
            self.streamed += len;
        } else {
            self.payload += len;
        }
        Ok(())
    }
}
