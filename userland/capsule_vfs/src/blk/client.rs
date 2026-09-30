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

//! Reads of the package store, through the kernel.
//!
//! The kernel's block layer picks the disk NONOS is kept on, NVMe, SATA or
//! virtio-blk, and the store's writes already went there. Its reads went to
//! the virtio-blk driver by name, so on any other disk the store could not be
//! loaded. Reading through the kernel as well keeps both halves on one disk.

use nonos_libc::mk_store_read;

use super::error::BlkError;
use super::wire::{MAX_READ_BYTES, SECTOR_SIZE, STORE_END_LBA};

const EPERM: i64 = -1;
const ENODEV: i64 = -19;
const ENOSYS: i64 = -38;
const ETIMEDOUT: i64 = -110;

/// How far the store may reach, in sectors: to the disk plan, below which the
/// kernel keeps the store's window. A smaller disk refuses the read past its
/// end by itself.
pub fn capacity() -> Result<u64, BlkError> {
    Ok(STORE_END_LBA)
}

pub fn read_blocks(lba: u64, out: &mut [u8]) -> Result<(), BlkError> {
    if out.is_empty() || out.len() % SECTOR_SIZE != 0 || out.len() > MAX_READ_BYTES {
        return Err(BlkError::Inval);
    }
    let n = mk_store_read(lba, out.as_mut_ptr(), out.len());
    match n {
        n if n == out.len() as i64 => Ok(()),
        /*
         * No NONOS disk behind any driver, a kernel without the call, or a
         * vfs without the store's authority: there is no store to read.
         */
        EPERM | ENODEV | ENOSYS => Err(BlkError::NoService),
        ETIMEDOUT => Err(BlkError::Transport(n)),
        n if n < 0 => Err(BlkError::Status(n as i32)),
        n => Err(BlkError::ShortReply(n as usize)),
    }
}
