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

//! Reading and writing a broker file through its descriptor.

use alloc::vec;
use alloc::vec::Vec;

use nonos_user_files::limits::IO_MAX;

use crate::linux::guest::{Fd, Guest};

use super::super::desc;
use super::{held, link};

/// The broker handle behind `fd`, when it is a broker file.
pub fn owns_fd(fd: &Fd) -> Option<u64> {
    desc::of(fd).and_then(held::handle)
}

/// Whether descriptor `fd` of `guest` is a broker file, for mmap.
pub fn is_user_file(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| owns_fd(f).is_some())
}

/// Up to `len` bytes of the file behind `fd` from `at`, short only at its
/// end; None when `fd` is not a broker file.
pub fn read(fd: &Fd, at: u64, len: usize) -> Option<Result<Vec<u8>, i64>> {
    let handle = owns_fd(fd)?;
    let mut out = vec![0u8; len];
    let mut done = 0;
    while done < len {
        let take = (len - done).min(IO_MAX);
        let got = link::with(|c| c.read(handle, at + done as u64, &mut out[done..done + take]));
        match got {
            Ok(0) => break,
            Ok(n) => done += n,
            Err(e) if done == 0 => return Some(Err(e as i64)),
            Err(_) => break,
        }
    }
    out.truncate(done);
    Some(Ok(out))
}

/// Writes `bytes` at `at`, or at the end for a descriptor opened O_APPEND,
/// which the broker's handle knows; the count taken and where it ended.
/// The descriptor's size follows the file's, for a later SEEK_END.
pub fn write(
    guest: &mut Guest,
    fd: u64,
    at: u64,
    bytes: &[u8],
) -> Option<Result<(usize, u64), i64>> {
    let entry = guest.fds.get_mut(fd as usize)?;
    let handle = owns_fd(entry)?;
    let append = desc::appends(entry);
    let (mut done, mut end) = (0usize, at);
    while done < bytes.len() || bytes.is_empty() {
        let piece = &bytes[done..(done + IO_MAX).min(bytes.len())];
        match link::with(|c| c.write(handle, at + done as u64, piece)) {
            Ok((n, size)) => {
                entry.size = size;
                done += n;
                end = if append { size } else { at + done as u64 };
                if n < piece.len() || bytes.is_empty() {
                    break;
                }
            }
            Err(e) if done == 0 => return Some(Err(e as i64)),
            Err(_) => break,
        }
    }
    Some(Ok((done, end)))
}
