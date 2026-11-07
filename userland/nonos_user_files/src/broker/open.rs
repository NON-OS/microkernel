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

//! Opening and closing a file.

use super::{Broker, State};
use crate::errno::{Errno, EACCES, EEXIST, EINVAL, EISDIR, ENOENT, ENOTDIR};
use crate::handles::Handle;
use crate::host::Objects;
use crate::node::{Attr, Kind, Node};
use crate::path::parts;
use crate::tree::ROOT;
use crate::wire::flags::{APPEND, CREATE, DIRECTORY, EXCL, KNOWN, READ, TRUNC, WRITE};

impl<O: Objects> Broker<O> {
    /// A handle on the file or directory at `path`, made first with
    /// CREATE. A directory opens to read only.
    pub fn open(
        &mut self,
        owner: u32,
        path: &[u8],
        flags: u32,
        mode: u32,
        now: u64,
    ) -> Result<(u64, Attr), Errno> {
        if flags & !KNOWN != 0 || flags & (READ | WRITE) == 0 {
            return Err(EINVAL);
        }
        let parts = parts(path)?;
        let (objects, s) = self.ready(now)?;
        s.handles.room(owner)?;
        let ino = match s.tree.walk(&parts) {
            Ok(_) if flags & CREATE != 0 && flags & EXCL != 0 => return Err(EEXIST),
            Ok(ino) => ino,
            Err(ENOENT) if flags & CREATE != 0 => create(s, &parts, flags, mode, now)?,
            Err(e) => return Err(e),
        };
        let node = s.tree.get(ino)?;
        match node.kind {
            Kind::Dir if flags & (WRITE | CREATE) != 0 => return Err(EISDIR),
            Kind::File if flags & DIRECTORY != 0 => return Err(ENOTDIR),
            _ => {}
        }
        if node.kind == Kind::File && flags & (TRUNC | WRITE) == TRUNC | WRITE && node.size != 0 {
            super::io::resize(objects, s, ino, 0, u64::MAX, now)?;
        }
        let handle = Handle {
            owner,
            ino,
            read: flags & READ != 0,
            write: flags & WRITE != 0,
            append: flags & APPEND != 0,
            wrote: false,
        };
        let id = s.handles.open(handle)?;
        let node = s.tree.get_mut(ino)?;
        node.open += 1;
        Ok((id, node.attr(ino)))
    }

    /// Lets the handle go. A file written through it owes a commit; one
    /// nothing names any more is released.
    pub fn close(&mut self, owner: u32, id: u64, now: u64) -> Result<(), Errno> {
        let (_, s) = self.ready(now)?;
        let h = s.handles.close(owner, id)?;
        let node = s.tree.get_mut(h.ino)?;
        node.open = node.open.saturating_sub(1);
        if h.wrote {
            s.owe();
        }
        s.release(h.ino);
        Ok(())
    }
}

fn create(s: &mut State, parts: &[&[u8]], flags: u32, mode: u32, now: u64) -> Result<u64, Errno> {
    let (dir, name) = s.tree.parent(parts)?;
    // The root holds the person's folders and nothing else.
    if dir == ROOT {
        return Err(EACCES);
    }
    if flags & DIRECTORY != 0 {
        return Err(EINVAL);
    }
    let ino = s.tree.add(dir, name, Node::new(Kind::File, mode, dir, name, now))?;
    s.touch(dir, now);
    s.owe();
    Ok(ino)
}
