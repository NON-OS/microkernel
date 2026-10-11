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

//! The two shapes of a vfs_pool body: a path, or an open descriptor, each
//! after this process's pid.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::vfs::call;

pub(super) type Answer<T> = Result<T, Option<i32>>;

pub(super) fn named(path: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(5 + path.len() + tail.len());
    body.extend_from_slice(&mk_getpid().to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.extend_from_slice(tail);
    body
}

pub(super) fn handle(fd: u32, tail: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(8 + tail.len());
    body.extend_from_slice(&mk_getpid().to_le_bytes());
    body.extend_from_slice(&fd.to_le_bytes());
    body.extend_from_slice(tail);
    body
}

pub(super) fn simple(op: u16, body: &[u8]) -> Answer<()> {
    call(op, body, &mut [0u8; 64]).map(|_| ())
}
