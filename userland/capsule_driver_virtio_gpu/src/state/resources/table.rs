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

use core::cell::Cell;

use super::resource::Resource;
use crate::protocol::MAX_RESOURCES;

pub struct ResourceTable {
    pub(super) next_id: Cell<u32>,
    pub(super) entries: [Cell<Resource>; MAX_RESOURCES],
}

impl Default for ResourceTable {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceTable {
    pub const fn new() -> Self {
        let entries = [const { Cell::new(Resource::NONE) }; MAX_RESOURCES];
        Self { next_id: Cell::new(1), entries }
    }
    pub fn insert(&self, r: Resource) -> Result<(), &'static str> {
        for entry in &self.entries {
            if !entry.get().in_use {
                entry.set(r);
                return Ok(());
            }
        }
        Err("virtio-gpu: resource table full")
    }
    pub fn lookup(&self, resource_id: u32) -> Option<Resource> {
        for entry in &self.entries {
            let r = entry.get();
            if r.in_use && r.resource_id == resource_id {
                return Some(r);
            }
        }
        None
    }
    pub fn update<F: FnOnce(&mut Resource)>(&self, resource_id: u32, f: F) -> bool {
        for entry in &self.entries {
            let mut r = entry.get();
            if r.in_use && r.resource_id == resource_id {
                f(&mut r);
                entry.set(r);
                return true;
            }
        }
        false
    }
}
