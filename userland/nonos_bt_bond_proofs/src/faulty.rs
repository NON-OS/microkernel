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

//! A `Keys` over `MemoryKeys` that refuses the operations a test names,
//! as a keyring that is full, gone or confused would.

use nonos_bt_bond::{Keys, KeysError, MemoryKeys};

#[derive(Default)]
pub struct Faulty {
    pub inner: MemoryKeys,
    pub store: Option<KeysError>,
    pub delete: Option<KeysError>,
    /// Refuses the next delete only, so a rollback after it goes through.
    pub delete_next: Option<KeysError>,
    pub list: Option<KeysError>,
    pub stores: usize,
}

impl Keys for Faulty {
    fn store(&mut self, kind: u8, data: &[u8]) -> Result<u32, KeysError> {
        self.stores += 1;
        self.store.map_or_else(|| self.inner.store(kind, data), Err)
    }

    fn retrieve(&mut self, id: u32) -> Result<Vec<u8>, KeysError> {
        self.inner.retrieve(id)
    }

    fn delete(&mut self, id: u32) -> Result<(), KeysError> {
        let refusal = self.delete_next.take().or(self.delete);
        refusal.map_or_else(|| self.inner.delete(id), Err)
    }

    fn list(&mut self, kind: u8) -> Result<Vec<u32>, KeysError> {
        self.list.map_or_else(|| self.inner.list(kind), Err)
    }
}
