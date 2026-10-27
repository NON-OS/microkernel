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

//! The package store's sectors, on the disk the kernel keeps NONOS on.

use crate::syscall::{
    call_raw, N_MK_DATA_HANDOVER, N_MK_KEEPING, N_MK_STORE_READ, N_MK_STORE_WRITE,
};

/// Read `len` bytes (whole sectors, at most 32 KiB) from `lba` into `buf`.
/// Returns the bytes read, or a negative errno.
pub fn mk_store_read(lba: u64, buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    call_raw(N_MK_STORE_READ, [lba, buf as u64, len as u64, 0, 0, 0])
}

/// Write `len` bytes (whole sectors) from `buf` at `lba`.
/// Returns the bytes written, or a negative errno.
pub fn mk_store_write(lba: u64, buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    call_raw(N_MK_STORE_WRITE, [lba, buf as u64, len as u64, 0, 0, 0])
}

/// Whether this boot keeps what is written: false until the person chose a
/// mode that keeps, and false on a kernel that does not know the call.
pub fn mk_keeping() -> bool {
    call_raw(N_MK_KEEPING, [0; 6]) == 1
}

/// Prepare this disk's data volume for an update: its key sealed once more,
/// for one boot, so the next release's bootloader opens it. Needs StoreWrite
/// and FileSystem. 0 once written; ENOTSUP this boot keeps nothing, EAGAIN
/// not open yet, EALREADY a passphrase keys it, ENOENT made for one kernel,
/// EACCES this boot not approved, EIO the disk did not take it.
pub fn mk_data_handover() -> i64 {
    call_raw(N_MK_DATA_HANDOVER, [0; 6])
}
