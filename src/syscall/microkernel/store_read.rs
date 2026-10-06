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

//! `MkStoreRead(lba, user_ptr, len)`: sectors of the package store.
//!
//! Store writes go through the block layer to the NONOS disk; reads went to
//! the virtio-blk driver by name, which on an NVMe or SATA machine has no
//! device. Reading here keeps both on one disk, in `MkStoreWrite`'s window.

use alloc::vec::Vec;

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

const STORE_BASE_LBA: u64 = 256;
const SECTOR: usize = 512;
/// Sixty-four sectors, what every block driver takes in one request.
const MAX_LEN: u64 = 32 * 1024;

pub fn sys_store_read(lba: u64, user_ptr: u64, len: u64) -> i64 {
    if !crate::syscall::caps::current_caps_or_default().can_store_write() {
        return ERRNO_PERM;
    }
    if user_ptr == 0 || len == 0 || len > MAX_LEN || len % SECTOR as u64 != 0 {
        return ERRNO_INVAL;
    }
    let len = len as usize;
    if lba < STORE_BASE_LBA
        || lba.saturating_add((len / SECTOR) as u64) > crate::fs::blockfs_volume::PLAN_LBA
    {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(user_ptr, len).is_err() {
        return ERRNO_FAULT;
    }
    let mut buf = Vec::new();
    if buf.try_reserve_exact(len).is_err() {
        return ERRNO_NOMEM;
    }
    buf.resize(len, 0u8);
    if super::store_copy::read(lba, &mut buf) {
        if crate::usercopy::copy_to_user(user_ptr, &buf).is_err() {
            return ERRNO_FAULT;
        }
        return len as i64;
    }
    if let Err(e) = crate::hardware::block_device::read(lba, &mut buf) {
        crate::log::warn!("[STORE-RD] lba {} refused: {:?}", lba, e);
        return super::store_errno::store_errno(e);
    }
    if crate::usercopy::copy_to_user(user_ptr, &buf).is_err() {
        return ERRNO_FAULT;
    }
    len as i64
}
