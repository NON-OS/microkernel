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

//! The machine's data volume, by name. A name is a slash and 1 to 63 of
//! [A-Za-z0-9._-]. Each call returns a count or a negative errno.

use crate::syscall::{call_raw, N_MK_DATA_IMPORT, N_MK_DATA_READ, N_MK_DATA_STAT};

/// Import the disk plan's file as `name`, kept only if its SHA-256 is
/// `sha256`. Needs StoreWrite and FileSystem. Returns its size.
pub fn mk_data_import(name: &[u8], sha256: &[u8; 32]) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_IMPORT, [p, n, sha256.as_ptr() as u64, 0, 0, 0])
}

/// The size of `name`. Needs FileSystem.
pub fn mk_data_stat(name: &[u8]) -> i64 {
    call_raw(N_MK_DATA_STAT, [name.as_ptr() as u64, name.len() as u64, 0, 0, 0, 0])
}

/// Read up to `buf.len()` bytes of `name` from `offset`, at most 1 MiB.
/// Needs FileSystem. Returns the bytes read, 0 at the end.
pub fn mk_data_read(name: &[u8], offset: u64, buf: &mut [u8]) -> i64 {
    let (p, n) = (name.as_ptr() as u64, name.len() as u64);
    call_raw(N_MK_DATA_READ, [p, n, offset, buf.as_mut_ptr() as u64, buf.len() as u64, 0])
}
