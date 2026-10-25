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
 * Records kept for a service name, not a pid (L33). A Bluetooth bond has to
 * outlive a restart of the driver that made it, and a pid never comes back.
 * The process serving the name now (`server/bond_gate.rs` asks the
 * registry) adopts them, and every other check stays by pid, unchanged.
 */

use super::types::{KeyType, Store, StoreError};

impl Store {
    /// Store a kind kept by name for `owner_pid`, which serves the holder
    /// name numbered `holder`. Its earlier records are adopted first, so the
    /// per-owner share counts them.
    pub fn store_named(
        &mut self,
        holder: u8,
        key_type: KeyType,
        data: &[u8],
        owner_pid: u32,
        now: u64,
        expires_at: u64,
    ) -> Result<u32, StoreError> {
        if !key_type.held_by_name() {
            return Err(StoreError::InvalidArgument);
        }
        self.adopt(holder, owner_pid);
        self.insert(key_type, data, (owner_pid, Some(holder)), now, expires_at)
    }

    /// Whether a record kept by name sits with a pid other than `pid`; only
    /// then is the registry asked whom the name belongs to.
    pub fn awaits_holder(&self, pid: u32) -> bool {
        self.entries.values().any(|e| e.holder.is_some() && e.owner_pid != pid)
    }

    /// Give `pid`, which serves holder name `holder` now, every record kept
    /// for that name, and say how many changed hands.
    pub fn adopt(&mut self, holder: u8, pid: u32) -> usize {
        let mut moved = 0;
        for entry in self.entries.values_mut().filter(|e| e.holder == Some(holder)) {
            if entry.owner_pid != pid {
                entry.owner_pid = pid;
                moved += 1;
            }
        }
        moved
    }
}
