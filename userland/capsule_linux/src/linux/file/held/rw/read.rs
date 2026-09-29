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

/* Reading a file at an offset: a made file, the family's copy, or the store. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{cache, desc, models, resolve, store, synth_ops};

/* The most one call moves. */
pub const MAX_IO: usize = 1 << 20;

pub fn read_at(guest: &mut Guest, fd: u64, at: u64, len: usize) -> Result<Vec<u8>, i64> {
    let entry = guest.fds.get_mut(fd as usize).filter(|f| f.is_open()).ok_or(errno::EBADF)?;
    match entry.kind {
        Kind::File => {}
        Kind::Dir => return Err(errno::EISDIR),
        _ => return Err(errno::EINVAL),
    }
    if !desc::reads(entry) {
        return Err(errno::EBADF);
    }
    let len = len.min(MAX_IO);
    if let Some(model) = models::read(&entry.path, at, len) {
        return model;
    }
    if let Some(made) = synth_ops::read(&entry.path, at, len) {
        return made;
    }
    if let Some(held) = cache::read(&entry.path, at, len) {
        return Ok(held);
    }
    if entry.stream.is_none() {
        entry.stream = Some(store::open(&resolve::key(&entry.path)).map_err(|_| errno::EIO)?);
    }
    let size = store::stat(&resolve::key(&entry.path)).map(|(s, _)| s).unwrap_or(entry.size);
    if len == 0 || at >= size {
        return Ok(Vec::new());
    }
    let want = (len as u64).min(size - at) as u32;
    let stream = entry.stream.as_mut().ok_or(errno::EBADF)?;
    stream.read_window(at, want).map_err(|_| errno::EIO)
}
