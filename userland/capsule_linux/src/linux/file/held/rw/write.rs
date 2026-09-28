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

/* Writing a file at an offset, into the family's copy. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{cache, desc, resolve, store};

/*
 * Write `bytes` at `at`, or at the end for a descriptor opened O_APPEND;
 * the count taken and where the write ended.
 */
pub fn write_at(guest: &mut Guest, fd: u64, at: u64, bytes: &[u8]) -> Result<(usize, u64), i64> {
    let entry = guest.fds.get(fd as usize).filter(|f| f.is_open()).ok_or(errno::EBADF)?;
    if entry.kind == Kind::Dir {
        return Err(errno::EISDIR);
    }
    if entry.kind != Kind::File || !entry.writable {
        return Err(errno::EBADF);
    }
    let path = entry.path.clone();
    let exists = store::stat(&resolve::key(&path)).is_ok();
    cache::hold(&path, exists)?;
    let at = if desc::appends(entry) { cache::size(&path).unwrap_or(0) } else { at };
    let n = cache::write(&path, at, bytes)?;
    let end = at + n as u64;
    if let Some(e) = guest.fds.get_mut(fd as usize) {
        e.size = cache::size(&path).unwrap_or(end);
    }
    Ok((n, end))
}
