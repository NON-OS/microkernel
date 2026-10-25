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
    pub fn store(
        &mut self,
        key_type: KeyType,
        data: &[u8],
        owner_pid: u32,
        now: u64,
        expires_at: u64,
    ) -> Result<u32, StoreError> {
        // Only through `store_named`, or any capsule could leave records the
        // reaper never takes.
        if key_type.held_by_name() {
            return Err(StoreError::AccessDenied);
        }
        self.insert(key_type, data, (owner_pid, None), now, expires_at)
    }
}
