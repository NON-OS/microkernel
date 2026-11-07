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

//! Changed chunks in RAM, oldest written out first when the count is full.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::Files;
use crate::errno::{Errno, ENOMEM};
use crate::host::Objects;
use crate::limits::DIRTY_CHUNKS;
use crate::store::read_full;

#[derive(Default)]
pub struct Dirty {
    /// (file, chunk index) to the chunk's bytes and when it was last used.
    bufs: BTreeMap<(u64, u64), (Vec<u8>, u64)>,
    clock: u64,
}

impl Dirty {
    pub fn get(&self, ino: u64, index: u64) -> Option<&[u8]> {
        self.bufs.get(&(ino, index)).map(|(b, _)| b.as_slice())
    }

    /// Lets go of a file's changed chunks unwritten: it was released, or
    /// truncated below them.
    pub fn forget(&mut self, ino: u64, from: u64) {
        self.bufs.retain(|(i, index), _| *i != ino || *index < from);
    }

    fn keys_of(&self, ino: Option<u64>) -> Vec<(u64, u64)> {
        self.bufs.keys().filter(|(i, _)| ino.is_none_or(|f| f == *i)).copied().collect()
    }
}

impl<O: Objects> Files<'_, O> {
    /// Chunk `index` of `ino`, changed in RAM: taken from the volume the
    /// first time, after writing the oldest out when the count is full.
    pub fn chunk_mut(&mut self, ino: u64, index: u64) -> Result<&mut Vec<u8>, Errno> {
        self.dirty.clock += 1;
        let now = self.dirty.clock;
        if !self.dirty.bufs.contains_key(&(ino, index)) {
            if self.dirty.bufs.len() >= DIRTY_CHUNKS {
                let oldest = self.dirty.bufs.iter().min_by_key(|(_, (_, t))| *t).map(|(k, _)| *k);
                if let Some(key) = oldest {
                    self.flush(key)?;
                }
            }
            let id = self.tree.get(ino)?.chunks.get(index as usize).copied().unwrap_or(0);
            let bytes = self.fetch(id)?;
            self.dirty.bufs.insert((ino, index), (bytes, now));
        }
        let (bytes, used) = self.dirty.bufs.get_mut(&(ino, index)).ok_or(crate::errno::EIO)?;
        *used = now;
        Ok(bytes)
    }

    fn fetch(&mut self, id: u64) -> Result<Vec<u8>, Errno> {
        if id == 0 {
            return Ok(Vec::new());
        }
        let size = self.objects.size(id)? as usize;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size).map_err(|_| ENOMEM)?;
        bytes.resize(size, 0);
        let got = read_full(self.objects, id, 0, &mut bytes)?;
        bytes.truncate(got);
        Ok(bytes)
    }

    /// Writes one changed chunk as a new object and points the file at it;
    /// the object it replaces waits for the next commit to be removed.
    fn flush(&mut self, key: (u64, u64)) -> Result<(), Errno> {
        let Some((bytes, _)) = self.dirty.bufs.get(&key) else {
            return Ok(());
        };
        // A chunk of zeros is a hole: nothing to write.
        let id = match bytes.iter().all(|b| *b == 0) {
            true => 0,
            false => self.store.append(self.objects, bytes)?,
        };
        let node = self.tree.get_mut(key.0)?;
        let slot = node.chunks.get_mut(key.1 as usize).ok_or(crate::errno::EIO)?;
        let old = core::mem::replace(slot, id);
        if old != 0 {
            self.store.pending.push(old);
        }
        self.dirty.bufs.remove(&key);
        Ok(())
    }

    /// Writes out the changed chunks of `ino`, or of every file for None.
    pub fn flush_all(&mut self, ino: Option<u64>) -> Result<(), Errno> {
        for key in self.dirty.keys_of(ino) {
            self.flush(key)?;
        }
        Ok(())
    }
}
