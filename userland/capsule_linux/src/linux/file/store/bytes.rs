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

//! A stored file's bytes read, and a program's hashed a window at a time.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_getpid;

use super::super::root::Key;
use super::packed::{packed, packed_hash};

pub(super) type Fail = &'static str;

/// The file at `at`, up to `max` bytes, in one allocation of its size. Grown
/// as it arrived, a 4 MB program passed through an 8 MiB buffer, which the
/// 16 MiB heap of a run could not give beside what it already held.
pub fn read(at: &Key, max: u32) -> Result<Vec<u8>, Fail> {
    let (size, _) = match vfs::stat(mk_getpid(), at.as_bytes()) {
        Ok(found) => found,
        Err(e) => return packed(at).and_then(|v| super::pack::read(v, max)).unwrap_or(Err(e)),
    };
    let len = u32::try_from(size).unwrap_or(u32::MAX).min(max);
    VfsStream::open(mk_getpid(), at.as_bytes())?.read_window(0, len)
}

/// The BLAKE3 of the file at `at`, read a window at a time.
///
/// Checking a program against its pin needs its hash, not its bytes, and a
/// program held whole to hash it is one allocation of its full size: the
/// chat program is up to 14 MB unstripped, in an installer whose heap is
/// 16 MiB. That allocation failing read as "its program is not in the
/// store", and the store said "in no index", on a machine that had it.
/// `Absent` when the store has no file there, `Unread` with the store's
/// reason when it has one that could not be read.
pub fn hash(at: &Key, max: u64) -> Result<[u8; 32], HashFault> {
    const WINDOW: u32 = 1 << 20;
    let (size, _) = match vfs::stat(mk_getpid(), at.as_bytes()) {
        Ok(found) => found,
        Err(e) => return packed_hash(at, max).unwrap_or(Err(HashFault::Absent(e))),
    };
    if size > max {
        return Err(HashFault::Unread("larger than any program this installs"));
    }
    let mut stream = VfsStream::open(mk_getpid(), at.as_bytes()).map_err(HashFault::Unread)?;
    super::super::hash_windows::hash_windows(
        size,
        WINDOW,
        HashFault::Unread("the store ended the file early"),
        |offset, len| stream.read_window(offset, len).map_err(HashFault::Unread),
    )
}

/// Why a file could not be hashed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFault {
    Absent(Fail),
    Unread(Fail),
}
