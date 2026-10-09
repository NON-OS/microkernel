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

//! The machine's data volume, by names of a slash and 1 to 63 [A-Za-z0-9._-]; errnos negative.

use crate::syscall::{
    call_raw, N_MK_DATA_FEED, N_MK_DATA_FEED_BEGIN, N_MK_DATA_IMPORT, N_MK_DATA_PASSPHRASE,
    N_MK_DATA_READ, N_MK_DATA_REMOVE, N_MK_DATA_STAT,
};

/// Import the disk plan's file as `name`, kept only if it is `bytes` long
/// and its SHA-256 is `sha256`. Needs StoreWrite and FileSystem. Its size.
pub fn mk_data_import(name: &[u8], sha256: &[u8; 32], bytes: u64) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_IMPORT, [p, n, sha256.as_ptr() as u64, bytes, 0, 0])
}

/// The size of `name`. Needs FileSystem.
pub fn mk_data_stat(name: &[u8]) -> i64 {
    call_raw(N_MK_DATA_STAT, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}

/// Read up to `buf.len()` bytes, at most 4 MiB, of `name` from `offset`.
/// Needs FileSystem. The bytes read, 0 at the end.
pub fn mk_data_read(name: &[u8], offset: u64, buf: &mut [u8]) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_READ, [p, n, offset, buf.as_mut_ptr() as u64, buf.len() as u64, 0])
}

/// `mk_data_read` into the guest `pid` the caller supervises, at `addr` there;
/// EFAULT when the guest has no page at `addr`, EPERM when `pid` is not its guest.
pub fn mk_data_read_peer(name: &[u8], offset: u64, pid: u32, addr: u64, len: u64) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_READ, [p, n, offset, addr, len, pid as u64])
}

/// Key the data volume with `passphrase`, 1 to 256 bytes (8 to `create` one over a ring never
/// written). Needs StoreWrite and FileSystem. 0; EACCES wrong, ENOENT not passphrase keyed,
/// EEXIST create over a volume, EBUSY one open, EAGAIN no disk chosen yet.
pub fn mk_data_volume_passphrase(create: bool, passphrase: &[u8]) -> i64 {
    let (p, n) = (passphrase.as_ptr() as u64, passphrase.len() as u64);
    call_raw(N_MK_DATA_PASSPHRASE, [create as u64, p, n, 0, 0, 0])
}

/// Begin feeding `name`, `bytes` long with SHA-256 `sha256`, or take up its
/// stream where it stopped; `probe` only says where it would start. Needs
/// StreamImport. The byte to feed from; EALREADY once imported and verified.
pub fn mk_data_feed_begin(name: &[u8], sha256: &[u8; 32], bytes: u64, probe: bool) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_FEED_BEGIN, [p, n, sha256.as_ptr() as u64, bytes, probe as u64, 0])
}

/// Seal `chunk`, at most 1 MiB, after what the caller's stream holds; how far it has come.
pub fn mk_data_feed(chunk: &[u8]) -> i64 {
    call_raw(N_MK_DATA_FEED, [chunk.as_ptr() as u64, chunk.len() as u64, 0, 0, 0, 0])
}

/// End the caller's stream: `keep` puts it down, mark saved, and says where;
/// else the file is linked only if whole with its SHA-256 (its size; EBADMSG).
pub fn mk_data_feed_end(keep: bool) -> i64 {
    call_raw(N_MK_DATA_FEED, [0, 0, if keep { 2 } else { 1 }, 0, 0, 0])
}

/// Take the imported `name` off the volume, with its record. Needs
/// StreamImport. 0; ENOENT nothing there, EBUSY a stream still coming to it.
pub fn mk_data_remove(name: &[u8]) -> i64 {
    call_raw(N_MK_DATA_REMOVE, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}
