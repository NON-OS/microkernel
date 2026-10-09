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

use super::types::{KeyType, Store, StoreError};

impl Store {
    pub fn delete(&mut self, id: u32, caller_pid: u32) -> Result<(), StoreError> {
        let entry = self.entries.get(&id).ok_or(StoreError::NotFound)?;
        if entry.owner_pid != caller_pid {
            return Err(StoreError::AccessDenied);
        }
        let mut removed = match self.entries.remove(&id) {
            Some(entry) => entry,
            None => return Err(StoreError::NotFound),
        };
        super::wipe::secure_wipe(&mut removed.data);
        // An account key takes its shield seed with it.
        if removed.key_type == KeyType::Secp256k1Eth {
            let tag = id.to_le_bytes();
            let seeds: alloc::vec::Vec<u32> = self
                .entries
                .iter()
                .filter(|(_, e)| {
                    e.key_type == KeyType::ShieldSeed
                        && e.owner_pid == caller_pid
                        && e.data.get(..4) == Some(&tag[..])
                })
                .map(|(i, _)| *i)
                .collect();
            for seed in seeds {
                if let Some(mut gone) = self.entries.remove(&seed) {
                    super::wipe::secure_wipe(&mut gone.data);
                }
            }
        }
        Ok(())
    }
}
