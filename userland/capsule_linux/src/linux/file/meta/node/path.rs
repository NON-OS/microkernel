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

/*
 * The metadata for a path, from the store, the family's copies and
 * the trees the personality makes.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::synth::{self, S_IFDIR, S_IFLNK, S_IFREG};
use super::super::super::{cache, dev, models, modes, mounts, resolve, store, times, walk};
use super::super::statbuf::inode;
use super::super::statbuf::Meta;
use super::device::device;
use super::fd::now;
use super::made::made;

pub const S_IFIFO: u32 = 0o010000;
pub const S_IFSOCK: u32 = 0o140000;

pub(super) fn at(path: &[u8], mode: u32, size: u64, mtime_ms: u64) -> Meta {
    let dev = mounts::dev(mounts::of(path).0);
    let (atime_ms, mtime_ms) = match times::of(path) {
        Some(t) if t.written_ms >= mtime_ms => (t.atime_ms, t.mtime_ms),
        _ => (mtime_ms, mtime_ms),
    };
    Meta { mode, size, ino: inode(path), nlink: 1, rdev: 0, dev, mtime_ms, atime_ms }
}

/* The path's metadata; its last component followed when `follow` is set. */
pub fn of(guest: &Guest, named: alloc::vec::Vec<u8>, follow: bool) -> Result<Meta, i64> {
    let full = walk::follow(guest, named, follow);
    if !follow {
        if let Some(to) = guest.links.target(&full) {
            return Ok(at(&full, S_IFLNK | 0o777, to.len() as u64, now()));
        }
    }
    if let Some(m) = dev::device_of(&full).and_then(|d| device(&full, d)) {
        return Ok(m);
    }
    if let Some(node) = synth::node(&full) {
        return made(&full, node?);
    }
    if let Some(model) = models::stat(&full) {
        return model.map(|(mode, size)| at(&full, mode, size, now()));
    }
    if let (Some(size), Some(t)) = (cache::size(&full), cache::mtime(&full)) {
        let mode = modes::of(&full).unwrap_or(modes::FILE);
        return Ok(at(&full, S_IFREG | mode, size, t));
    }
    let Ok((size, is_dir, mtime, writable)) = store::stat_full(&resolve::key(&full)) else {
        return built_in(&full).ok_or(errno::ENOENT);
    };
    let kind = if is_dir { S_IFDIR } else { S_IFREG };
    let default = match (writable, is_dir) {
        (false, _) => modes::SHARED,
        (true, true) => modes::DIR,
        (true, false) => modes::FILE,
    };
    Ok(at(&full, kind | modes::of(&full).unwrap_or(default), size, mtime))
}

/// One of the built-in BusyBox's programs in a bin directory the tree has no
/// file for: execve runs it (exec_resolve), so it is there to a shell's PATH
/// search and to make's, which look before they run. Read-only, executable,
/// the built-in's size.
fn built_in(full: &[u8]) -> Option<Meta> {
    crate::linux::built_in::serves(full)?;
    Some(at(full, S_IFREG | 0o555, crate::linux::built_in::size(), 0))
}
