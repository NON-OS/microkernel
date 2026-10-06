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

//! `MkBootSlots(out_ptr, out_len)`: the two boot slots of the device proof's
//! witness, the bootloader's and this kernel's, each with its context digest,
//! path and root, as the record in `security::boot::slots::record`. Which
//! slots a machine booted narrows down which release it runs, so it goes only
//! to the capsule that holds DeviceSecret, the one that proves.

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_PERM};
use crate::security::boot::slots::{boot_slots_record, RECORD_LEN};

/// The record's length on success; ENOENT when the kernel's check did not
/// admit the loader or either image carries no trailer of its kind.
pub(in crate::syscall::microkernel) fn sys_boot_slots(out_ptr: u64, out_len: u64) -> i64 {
    if !super::gate::device_secret_caller() {
        return ERRNO_PERM;
    }
    if out_len < RECORD_LEN as u64 {
        return ERRNO_NOMEM;
    }
    let Some(record) = boot_slots_record() else {
        return ERRNO_NOENT;
    };
    if crate::usercopy::write_user_bytes(out_ptr, &record).is_err() {
        return ERRNO_FAULT;
    }
    RECORD_LEN as i64
}
