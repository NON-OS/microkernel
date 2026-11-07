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

//! rename, with Linux's RENAME_NOREPLACE and RENAME_EXCHANGE.

use super::{commit, Broker, State};
use crate::errno::{Errno, EACCES, EBUSY, EEXIST, EINVAL, EISDIR, ENOENT, ENOTDIR, ENOTEMPTY};
use crate::host::Objects;
use crate::node::Kind;
use crate::path::parts;
use crate::tree::ROOT;

use crate::wire::flags::{RENAME_EXCHANGE as EXCHANGE, RENAME_NOREPLACE as NOREPLACE};

impl<O: Objects> Broker<O> {
    /// Moves `from` to `to`, replacing what `to` named, and commits before
    /// it answers: a save renamed over a document is on the volume, or the
    /// old document still is.
    pub fn rename(&mut self, from: &[u8], to: &[u8], flags: u32, now: u64) -> Result<(), Errno> {
        if flags & !(NOREPLACE | EXCHANGE) != 0 || flags == NOREPLACE | EXCHANGE {
            return Err(EINVAL);
        }
        let (fp, tp) = (parts(from)?, parts(to)?);
        let (objects, s) = self.ready(now)?;
        // Both directories first, then the name: Linux's order of refusals.
        let (fd, fname) = s.tree.parent(&fp)?;
        let (td, tname) = s.tree.parent(&tp)?;
        let ino = s.tree.child(fd, fname).ok_or(ENOENT)?;
        if s.tree.is_fixed(ino) {
            return Err(EBUSY);
        }
        if td == ROOT {
            return Err(if s.tree.child(td, tname).is_some() { EBUSY } else { EACCES });
        }
        let kind = s.tree.get(ino)?.kind;
        if kind == Kind::Dir && s.tree.within(td, ino) {
            return Err(EINVAL);
        }
        let there = s.tree.child(td, tname);
        // A name moved onto a directory above it: Linux's trap, ENOTEMPTY.
        if there.is_some_and(|other| other != ino && s.tree.within(ino, other)) {
            return Err(ENOTEMPTY);
        }
        if flags & EXCHANGE != 0 {
            let other = there.ok_or(ENOENT)?;
            exchange(s, (fd, fname, ino), (td, tname, other))?;
        } else if there != Some(ino) {
            if let Some(other) = there {
                if flags & NOREPLACE != 0 {
                    return Err(EEXIST);
                }
                replaceable(s, kind, other)?;
                s.tree.unlink(td, tname)?;
                s.release(other);
            }
            s.tree.get_mut(fd)?.children.remove(fname);
            s.tree.relink(ino, td, tname)?;
        }
        s.touch(fd, now);
        s.touch(td, now);
        s.tree.get_mut(ino)?.ctime = now;
        commit(objects, s)
    }
}

/// Whether a `kind` may take the place of `other`, as Linux decides it.
fn replaceable(s: &State, kind: Kind, other: u64) -> Result<(), Errno> {
    let node = s.tree.get(other)?;
    match (kind, node.kind) {
        (Kind::File, Kind::Dir) => Err(EISDIR),
        (Kind::Dir, Kind::File) => Err(ENOTDIR),
        (Kind::Dir, Kind::Dir) if !node.children.is_empty() => Err(ENOTEMPTY),
        _ => Ok(()),
    }
}

fn exchange(s: &mut State, a: (u64, &[u8], u64), b: (u64, &[u8], u64)) -> Result<(), Errno> {
    let ((ad, an, ai), (bd, bn, bi)) = (a, b);
    if ai == bi {
        return Ok(());
    }
    if s.tree.is_fixed(bi) {
        return Err(EBUSY);
    }
    if s.tree.get(bi)?.kind == Kind::Dir && s.tree.within(ad, bi) {
        return Err(EINVAL);
    }
    s.tree.get_mut(ad)?.children.remove(an);
    s.tree.get_mut(bd)?.children.remove(bn);
    s.tree.relink(bi, ad, an)?;
    s.tree.relink(ai, bd, bn)
}
