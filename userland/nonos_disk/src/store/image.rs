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

//! A finished store, as the bytes that go to the disk from `STORE_BASE_LBA`.

use alloc::rc::Rc;
use alloc::vec::Vec;

use nonos_disk_map::{STORE_MAGIC, STORE_VERSION};

use super::builder::StoreBuilder;

/// Shared rather than copied: the write and the read-back hold the same
/// bytes.
#[derive(Clone)]
pub struct StoreImage {
    pub(super) bytes: Rc<Vec<u8>>,
    pub files: usize,
    pub payload_bytes: u64,
}

impl StoreImage {
    /// A store with no files: vfs loads it as present and empty, and
    /// appends to it the first time something is kept.
    pub fn empty() -> StoreImage {
        StoreBuilder::new().finish()
    }

    /// The whole container: header, the full table, every payload.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn shared(&self) -> Rc<Vec<u8>> {
        self.bytes.clone()
    }
}

impl StoreBuilder {
    /// The header last, with the count of what was added.
    pub fn finish(mut self) -> StoreImage {
        let count = self.names.len();
        self.bytes[0..8].copy_from_slice(&STORE_MAGIC);
        self.bytes[8..12].copy_from_slice(&STORE_VERSION.to_le_bytes());
        self.bytes[12..16].copy_from_slice(&(count as u32).to_le_bytes());
        StoreImage { bytes: Rc::new(self.bytes), files: count, payload_bytes: self.payload }
    }
}
