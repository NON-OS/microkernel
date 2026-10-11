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

//! One vfs_pool request each, under this process's pid.

use alloc::vec;
use alloc::vec::Vec;

use super::frame::{handle, named, simple, Answer};
use super::vfs::{call, HDR_LEN, OP_CLOSE, OP_MKDIR, OP_OPEN, OP_READ, OP_STORE_PERSIST};
use super::vfs::{OP_UNLINK, OP_WRITE};

pub(super) fn open(path: &[u8], flags: u32) -> Answer<u32> {
    let mut rx = [0u8; 64];
    let n = call(OP_OPEN, &named(path, &flags.to_le_bytes()), &mut rx)?;
    let fd = rx.get(HDR_LEN + 4..HDR_LEN + 8).filter(|_| n >= HDR_LEN + 8).ok_or(None)?;
    Ok(u32::from_le_bytes([fd[0], fd[1], fd[2], fd[3]]))
}

/// Exactly `len` bytes, or silence: a short file is not a state file.
pub(super) fn read(fd: u32, len: usize) -> Answer<Vec<u8>> {
    let mut rx = vec![0u8; HDR_LEN + 8 + len];
    let n = call(OP_READ, &handle(fd, &(len as u32).to_le_bytes()), &mut rx)?;
    let data = rx.get(HDR_LEN + 4..n).filter(|d| d.len() == len).ok_or(None)?;
    Ok(data.to_vec())
}

pub(super) fn write(fd: u32, data: &[u8]) -> Answer<()> {
    simple(OP_WRITE, &handle(fd, data))
}

pub(super) fn close(fd: u32) {
    let _ = simple(OP_CLOSE, &handle(fd, &[]));
}

pub(super) fn mkdir(path: &[u8]) -> Answer<()> {
    simple(OP_MKDIR, &named(path, &[]))
}

pub(super) fn unlink(path: &[u8]) -> Answer<()> {
    simple(OP_UNLINK, &named(path, &[]))
}

pub(super) fn persist(path: &[u8]) -> Answer<()> {
    simple(OP_STORE_PERSIST, &named(path, &[]))
}
