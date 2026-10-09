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

use alloc::collections::BTreeMap;
use alloc::string::String;

pub const MAX_HANDLES: usize = 1024;
/// The most handles one owner holds open, so no one caller can take every
/// handle from the rest, the kernel's /ram files among them.
pub const PER_OWNER: usize = MAX_HANDLES / 2;
/// The owner the kernel's own requests carry: every process's /ram file is
/// opened through it.
const KERNEL: u32 = 0;

struct Handle {
    path: String,
    owner_pid: u32,
}

pub struct HandleTable {
    next_id: u64,
    table: BTreeMap<u64, Handle>,
}

impl HandleTable {
    pub const fn new() -> Self {
        Self { next_id: 1, table: BTreeMap::new() }
    }

    pub fn insert(&mut self, path: String, owner_pid: u32) -> Option<u64> {
        if self.is_full() || self.held_by(owner_pid) >= PER_OWNER {
            return None;
        }
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.table.insert(id, Handle { path, owner_pid });
        Some(id)
    }

    pub fn path_for(&self, id: u64, sender_pid: u32) -> Result<&str, HandleError> {
        let h = self.table.get(&id).ok_or(HandleError::NotFound)?;
        if sender_pid != 0 && h.owner_pid != sender_pid {
            return Err(HandleError::Denied);
        }
        Ok(h.path.as_str())
    }

    pub fn remove(&mut self, id: u64, sender_pid: u32) -> Result<(), HandleError> {
        match self.table.get(&id) {
            None => Err(HandleError::NotFound),
            Some(h) if sender_pid != 0 && h.owner_pid != sender_pid => Err(HandleError::Denied),
            Some(_) => {
                self.table.remove(&id);
                Ok(())
            }
        }
    }
}

impl HandleTable {
    /// Whether no handle can be opened for anyone.
    pub fn is_full(&self) -> bool {
        self.table.len() >= MAX_HANDLES
    }

    /// How many handles `owner_pid` holds open.
    pub fn held_by(&self, owner_pid: u32) -> usize {
        self.table.values().filter(|h| h.owner_pid == owner_pid).count()
    }

    /// Close every handle whose owner `alive` says has ended, and say how many
    /// were closed. Only the owner (or the kernel) closes a handle, so a caller
    /// that ended without closing held its handles for good. The kernel's own
    /// are never taken: mk_pid_alive does not name pid 0 as alive.
    pub fn close_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let before = self.table.len();
        self.table.retain(|_, h| h.owner_pid == KERNEL || alive(h.owner_pid));
        before.saturating_sub(self.table.len())
    }
}

pub enum HandleError {
    NotFound,
    Denied,
}
