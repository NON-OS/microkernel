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

//! Every store operation this personality makes for a guest.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_getpid;

use super::root::Key;

type Fail = &'static str;

/// The file at `at`, up to `max` bytes, in one allocation of its size. Grown
/// as it arrived, a 4 MB program passed through an 8 MiB buffer, which the
/// 16 MiB heap of a run could not give beside what it already held.
pub fn read(at: &Key, max: u32) -> Result<Vec<u8>, Fail> {
    let (size, _) = vfs::stat(mk_getpid(), at.as_bytes())?;
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
    let (size, _) = vfs::stat(mk_getpid(), at.as_bytes()).map_err(HashFault::Absent)?;
    if size > max {
        return Err(HashFault::Unread("larger than any program this installs"));
    }
    let mut stream = VfsStream::open(mk_getpid(), at.as_bytes()).map_err(HashFault::Unread)?;
    super::hash_windows::hash_windows(
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

pub fn write(at: &Key, data: &[u8]) -> Result<(), Fail> {
    at.writable()?;
    vfs::write_file(mk_getpid(), at.as_bytes(), data)
}

/*
 * The store keeps files and no directories: /linux/bin exists only as the
 * prefix of what is in it. A key that is no file but has keys below it is
 * answered as a directory, or `ls /bin` finds nothing to list.
 */
pub fn stat(at: &Key) -> Result<(u64, bool), Fail> {
    vfs::stat(mk_getpid(), at.as_bytes()).or_else(|e| implicit_dir(at).map(|_| (0, true)).ok_or(e))
}

pub fn stat_full(at: &Key) -> Result<(u64, bool, u64, bool), Fail> {
    vfs::stat_full(mk_getpid(), at.as_bytes())
        .or_else(|e| implicit_dir(at).map(|_| (0, true, 0, false)).ok_or(e))
}

fn implicit_dir(at: &Key) -> Option<()> {
    let below = list(at).ok()?;
    let prefix = at.as_bytes();
    below.iter().any(|k| k.as_bytes().get(prefix.len()) == Some(&b'/')).then_some(())
}

pub fn list(at: &Key) -> Result<Vec<String>, Fail> {
    vfs::list_paths(mk_getpid(), at.as_bytes())
}

pub fn open(at: &Key) -> Result<VfsStream, Fail> {
    VfsStream::open(mk_getpid(), at.as_bytes())
}
