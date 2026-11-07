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

//! The calls that name a path: stat, mkdir, unlink, rmdir, truncate, setattr.

use super::Broker;
use crate::errno::{Errno, EACCES, EBUSY, EISDIR, ENOENT, ENOTDIR, ENOTEMPTY};
use crate::host::Objects;
use crate::node::{Attr, Kind, Node};
use crate::path::parts;
use crate::tree::ROOT;

use crate::wire::flags::{SET_MODE, SET_MTIME};

impl<O: Objects> Broker<O> {
    pub fn stat(&mut self, path: &[u8], now: u64) -> Result<Attr, Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let ino = s.tree.walk(&parts)?;
        Ok(s.tree.get(ino)?.attr(ino))
    }

    pub fn mkdir(&mut self, path: &[u8], mode: u32, now: u64) -> Result<Attr, Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let (dir, name) = s.tree.parent(&parts)?;
        if dir == ROOT {
            return Err(if s.tree.child(dir, name).is_some() {
                crate::errno::EEXIST
            } else {
                EACCES
            });
        }
        let ino = s.tree.add(dir, name, Node::new(Kind::Dir, mode, dir, name, now))?;
        s.touch(dir, now);
        s.owe();
        Ok(s.tree.get(ino)?.attr(ino))
    }

    /// Removes a file's name; its bytes stay while a handle holds it.
    pub fn unlink(&mut self, path: &[u8], now: u64) -> Result<(), Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let (dir, name) = s.tree.parent(&parts)?;
        let ino = s.tree.child(dir, name).ok_or(ENOENT)?;
        if s.tree.get(ino)?.kind == Kind::Dir {
            return Err(EISDIR);
        }
        s.tree.unlink(dir, name)?;
        s.touch(dir, now);
        s.release(ino);
        s.owe();
        Ok(())
    }

    pub fn rmdir(&mut self, path: &[u8], now: u64) -> Result<(), Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let ino = s.tree.walk(&parts)?;
        if s.tree.is_fixed(ino) {
            return Err(EBUSY);
        }
        let node = s.tree.get(ino)?;
        if node.kind != Kind::Dir {
            return Err(ENOTDIR);
        }
        if !node.children.is_empty() {
            return Err(ENOTEMPTY);
        }
        let (dir, name) = (node.parent, node.name.clone());
        s.tree.unlink(dir, &name)?;
        s.touch(dir, now);
        s.release(ino);
        s.owe();
        Ok(())
    }

    pub fn truncate(&mut self, path: &[u8], size: u64, now: u64) -> Result<(), Errno> {
        let parts = parts(path)?;
        let quota = self.quota;
        let (objects, s) = self.ready(now)?;
        let ino = s.tree.walk(&parts)?;
        super::io::resize(objects, s, ino, size, quota, now)
    }

    /// Sets the mode bits, the modification time, or both.
    pub fn setattr(
        &mut self,
        path: &[u8],
        mask: u32,
        mode: u32,
        mtime: u64,
        now: u64,
    ) -> Result<Attr, Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let ino = s.tree.walk(&parts)?;
        let node = s.tree.get_mut(ino)?;
        if mask & SET_MODE != 0 {
            node.mode = mode & 0o7777;
        }
        if mask & SET_MTIME != 0 {
            node.mtime = mtime;
        }
        node.ctime = now;
        let attr = node.attr(ino);
        s.owe();
        Ok(attr)
    }
}
