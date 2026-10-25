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

use alloc::vec::Vec;

use super::types::{KeyType, Store};

impl Store {
    /// The ids of `key_type` that `caller_pid` owns, lowest first. A
    /// restarted driver finds its records again by this (L33).
    pub fn ids_owned_by(&self, caller_pid: u32, key_type: KeyType) -> Vec<u32> {
        let mine = self.entries.iter().filter(|(_, e)| e.owner_pid == caller_pid);
        mine.filter(|(_, e)| e.key_type == key_type).map(|(id, _)| *id).collect()
    }
}
