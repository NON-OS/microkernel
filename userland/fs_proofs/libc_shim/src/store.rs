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

//! `MkStoreRead` and `MkStoreWrite` as the kernel answers them, in the
//! order the kernel checks: the request's shape, the store's window, then
//! the disk. A read or a write with no disk selected is ENODEV, as the
//! kernel's one table for both (store_errno.rs) answers.

use crate::disk::{DISK, SECTOR};

const STORE_BASE_LBA: u64 = 256;
const PLAN_LBA: u64 = 245_760;
const MAX_READ: usize = 32 * 1024;
const MAX_WRITE: usize = 8192;

const EPERM: i64 = -1;
const ENODEV: i64 = -19;
const EINVAL: i64 = -22;

pub fn mk_store_read(lba: u64, buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 || len > MAX_READ || len % SECTOR != 0 {
        return EINVAL;
    }
    if lba < STORE_BASE_LBA || lba.saturating_add((len / SECTOR) as u64) > PLAN_LBA {
        return EINVAL;
    }
    DISK.with(|d| {
        let d = d.borrow();
        if !d.present {
            return ENODEV;
        }
        if let Some(errno) = d.read_errno {
            return errno;
        }
        /*
         * SAFETY: the caller hands `len` writable bytes at `buf`, as the
         * kernel call requires.
         */
        let out = unsafe { std::slice::from_raw_parts_mut(buf, len) };
        for (i, chunk) in out.chunks_mut(SECTOR).enumerate() {
            chunk.copy_from_slice(&d.sector(lba + i as u64));
        }
        len as i64
    })
}

pub fn mk_store_write(lba: u64, buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 || lba < STORE_BASE_LBA {
        return EINVAL;
    }
    if len > MAX_WRITE || len % SECTOR != 0 {
        return EINVAL;
    }
    if lba.saturating_add((len / SECTOR) as u64) > PLAN_LBA {
        return EPERM;
    }
    DISK.with(|d| {
        let mut d = d.borrow_mut();
        if !d.present || !d.writable {
            return ENODEV;
        }
        /*
         * SAFETY: the caller hands `len` readable bytes at `buf`, as the
         * kernel call requires.
         */
        let data = unsafe { std::slice::from_raw_parts(buf, len) };
        let took = if d.short_writes && len > SECTOR { len - SECTOR } else { len };
        for (i, chunk) in data[..took].chunks(SECTOR).enumerate() {
            d.land(lba + i as u64, chunk);
        }
        took as i64
    })
}
