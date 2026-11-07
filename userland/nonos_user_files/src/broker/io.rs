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

//! Reading, writing, sizing and syncing through a handle.

use alloc::vec::Vec;

use super::{commit, Broker};
use crate::chunks::{read, truncate, write};
use crate::errno::{Errno, EBADF, EDQUOT, EINVAL, EISDIR};
use crate::host::Objects;
use crate::limits::IO_MAX;
use crate::node::{Attr, Kind};

impl<O: Objects> Broker<O> {
    pub fn read(
        &mut self,
        owner: u32,
        id: u64,
        offset: u64,
        len: usize,
        now: u64,
    ) -> Result<Vec<u8>, Errno> {
        let (objects, s) = self.ready(now)?;
        let h = *s.handles.get(owner, id)?;
        if !h.read {
            return Err(EBADF);
        }
        if s.tree.get(h.ino)?.kind == Kind::Dir {
            return Err(EISDIR);
        }
        read(&mut s.files(objects), h.ino, offset, len.min(IO_MAX))
    }

    /// Writes at `offset`, or at the end for a handle opened to append.
    /// Returns what landed and the file's size after.
    pub fn write(
        &mut self,
        owner: u32,
        id: u64,
        offset: u64,
        data: &[u8],
        now: u64,
    ) -> Result<(usize, u64), Errno> {
        let quota = self.quota;
        let (objects, s) = self.ready(now)?;
        let h = s.handles.get(owner, id)?;
        if !h.write {
            return Err(EBADF);
        }
        h.wrote = true;
        let ino = h.ino;
        let size = s.tree.get(ino)?.size;
        let at = if h.append { size } else { offset };
        if at.checked_add(data.len() as u64).is_none_or(|end| end > crate::limits::MAX_FILE) {
            return Err(crate::errno::EFBIG);
        }
        let grows = at.saturating_add(data.len() as u64).saturating_sub(size);
        if s.used.saturating_add(grows) > quota {
            return Err(EDQUOT);
        }
        let n = write(&mut s.files(objects), ino, at, data, now)?;
        let after = s.tree.get(ino)?.size;
        s.used = s.used + after - size;
        Ok((n, after))
    }

    pub fn fstat(&mut self, owner: u32, id: u64, now: u64) -> Result<Attr, Errno> {
        let (_, s) = self.ready(now)?;
        let ino = s.handles.get(owner, id)?.ino;
        Ok(s.tree.get(ino)?.attr(ino))
    }

    /// A handle not opened to write is EINVAL, as Linux's ftruncate says.
    pub fn ftruncate(&mut self, owner: u32, id: u64, size: u64, now: u64) -> Result<(), Errno> {
        let quota = self.quota;
        let (objects, s) = self.ready(now)?;
        let h = s.handles.get(owner, id)?;
        if !h.write {
            return Err(EINVAL);
        }
        h.wrote = true;
        let ino = h.ino;
        resize(objects, s, ino, size, quota, now)
    }

    /// Commits now: every changed chunk of every file, and the tree.
    pub fn fsync(&mut self, owner: u32, id: u64, now: u64) -> Result<(), Errno> {
        let (objects, s) = self.ready(now)?;
        s.handles.get(owner, id)?.wrote = false;
        commit(objects, s)
    }
}

/// Sets a file's size, held to the quota, and owes a commit.
pub(super) fn resize<O: Objects>(
    objects: &mut O,
    s: &mut super::State,
    ino: u64,
    size: u64,
    quota: u64,
    now: u64,
) -> Result<(), Errno> {
    let old = s.tree.get(ino)?.size;
    if s.tree.get(ino)?.kind == Kind::Dir {
        return Err(EISDIR);
    }
    if size > crate::limits::MAX_FILE {
        return Err(crate::errno::EFBIG);
    }
    if size > old && s.used.saturating_add(size - old) > quota {
        return Err(EDQUOT);
    }
    truncate(&mut s.files(objects), ino, size, now)?;
    s.used = s.used - old + size;
    s.owe();
    Ok(())
}
