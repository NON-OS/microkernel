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

/* Metadata made rather than read: from its parts, or for a built-in
 * program. */

use super::super::super::super::synth::S_IFREG;
use super::super::super::super::{mounts, times};
use super::super::super::statbuf::inode;
use super::super::super::statbuf::Meta;

pub(in super::super) fn at(path: &[u8], mode: u32, size: u64, mtime_ms: u64) -> Meta {
    let dev = mounts::dev(mounts::of(path).0);
    let (atime_ms, mtime_ms) = match times::of(path) {
        Some(t) if t.written_ms >= mtime_ms => (t.atime_ms, t.mtime_ms),
        _ => (mtime_ms, mtime_ms),
    };
    Meta { mode, size, ino: inode(path), nlink: 1, rdev: 0, dev, mtime_ms, atime_ms }
}

/// One of the built-in BusyBox's programs in a bin directory the tree has no
/// file for: execve runs it (exec_resolve), so it is there to a shell's PATH
/// search and to make's, which look before they run. Read-only, executable,
/// the built-in's size.
pub(super) fn built_in(full: &[u8]) -> Option<Meta> {
    crate::linux::built_in::serves(full)?;
    Some(at(full, S_IFREG | 0o555, crate::linux::built_in::size(), 0))
}
