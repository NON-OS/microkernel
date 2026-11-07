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

//! Open files, each owned by the client that opened it.
//!
//! A handle number means nothing to any client but its owner: a request
//! naming another client's handle is EBADF, as an unknown one is.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::errno::{Errno, EBADF, EMFILE, ENFILE};
use crate::limits::{HANDLES, HANDLES_PER_CLIENT};

#[derive(Clone, Copy, Debug)]
pub struct Handle {
    pub owner: u32,
    pub ino: u64,
    pub read: bool,
    pub write: bool,
    pub append: bool,
    /// Written through since it was opened or last synced: its close
    /// commits.
    pub wrote: bool,
}

#[derive(Default)]
pub struct Handles {
    map: BTreeMap<u64, Handle>,
    next: u64,
}

impl Handles {
    /// Whether `owner` may open one more, asked before an open creates.
    pub fn room(&self, owner: u32) -> Result<(), Errno> {
        if self.map.len() >= HANDLES {
            return Err(ENFILE);
        }
        if self.map.values().filter(|o| o.owner == owner).count() >= HANDLES_PER_CLIENT {
            return Err(EMFILE);
        }
        Ok(())
    }

    pub fn open(&mut self, h: Handle) -> Result<u64, Errno> {
        self.room(h.owner)?;
        self.next += 1;
        self.map.insert(self.next, h);
        Ok(self.next)
    }

    pub fn get(&mut self, owner: u32, id: u64) -> Result<&mut Handle, Errno> {
        self.map.get_mut(&id).filter(|h| h.owner == owner).ok_or(EBADF)
    }

    pub fn close(&mut self, owner: u32, id: u64) -> Result<Handle, Errno> {
        self.get(owner, id)?;
        self.map.remove(&id).ok_or(EBADF)
    }

    /// Every client holding a handle.
    pub fn owners(&self) -> Vec<u32> {
        let mut out: Vec<u32> = self.map.values().map(|h| h.owner).collect();
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Takes every handle `owner` holds.
    pub fn close_all(&mut self, owner: u32) -> Vec<Handle> {
        let ids: Vec<u64> =
            self.map.iter().filter(|(_, h)| h.owner == owner).map(|(i, _)| *i).collect();
        ids.iter().filter_map(|id| self.map.remove(id)).collect()
    }
}
