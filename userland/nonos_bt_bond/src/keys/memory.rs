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

//! Records kept in this capsule's own memory, which last for the boot and
//! no longer. It holds what the keyring would let one owner hold, sixteen
//! records, so a book that fits here fits there.

use super::types::{Keys, KeysError};
use crate::wipe::wipe;
use alloc::vec::Vec;

pub const MEMORY_RECORDS: usize = 16;

#[derive(Default)]
pub struct MemoryKeys {
    records: Vec<(u32, u8, Vec<u8>)>,
}

impl Keys for MemoryKeys {
    fn store(&mut self, kind: u8, data: &[u8]) -> Result<u32, KeysError> {
        if self.records.len() >= MEMORY_RECORDS {
            return Err(KeysError::Full);
        }
        // The lowest id not in use, never zero: at most sixteen are held,
        // so the search ends by seventeen.
        let free = (1..).find(|id| self.records.iter().all(|r| r.0 != *id));
        let id = free.ok_or(KeysError::Full)?;
        self.records.push((id, kind, data.to_vec()));
        Ok(id)
    }

    fn retrieve(&mut self, id: u32) -> Result<Vec<u8>, KeysError> {
        let found = self.records.iter().find(|r| r.0 == id);
        found.map(|r| r.2.clone()).ok_or(KeysError::NotFound)
    }

    fn delete(&mut self, id: u32) -> Result<(), KeysError> {
        let at = self.records.iter().position(|r| r.0 == id).ok_or(KeysError::NotFound)?;
        let mut gone = self.records.remove(at);
        wipe(&mut gone.2);
        Ok(())
    }

    fn list(&mut self, kind: u8) -> Result<Vec<u32>, KeysError> {
        Ok(self.records.iter().filter(|r| r.1 == kind).map(|r| r.0).collect())
    }
}

impl Drop for MemoryKeys {
    fn drop(&mut self) {
        for r in self.records.iter_mut() {
            wipe(&mut r.2);
        }
    }
}
