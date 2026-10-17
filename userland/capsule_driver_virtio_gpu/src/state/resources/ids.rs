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
//! Ids and the entries behind them (G-73). A fresh id is one no entry
//! holds, so an id a client chose, the primary's or the blank's is never
//! handed out twice; an entry goes when the host let go of its resource.

use super::resource::Resource;
use super::table::ResourceTable;
use crate::protocol::MAX_RESOURCES;

impl ResourceTable {
    /// The next id after the last one given that no entry holds. The table
    /// holds at most MAX_RESOURCES, so this ends within that many tries.
    pub fn alloc_id(&self) -> u32 {
        loop {
            let id = self.next_id.get();
            self.next_id.set(id.wrapping_add(1).max(1));
            if self.lookup(id).is_none() {
                return id;
            }
        }
    }

    pub fn remove(&self, resource_id: u32) {
        for entry in &self.entries {
            if entry.get().in_use && entry.get().resource_id == resource_id {
                entry.set(Resource::default());
            }
        }
    }

    /// The ids of the entries `keep` picks, taken before any is changed.
    pub fn ids_where(&self, keep: impl Fn(&Resource) -> bool) -> ([u32; MAX_RESOURCES], usize) {
        let (mut ids, mut n) = ([0u32; MAX_RESOURCES], 0);
        for entry in &self.entries {
            let r = entry.get();
            if r.in_use && keep(&r) {
                ids[n] = r.resource_id;
                n += 1;
            }
        }
        (ids, n)
    }
}
