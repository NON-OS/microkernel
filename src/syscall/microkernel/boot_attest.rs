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

//! `MkBootAttest(out_ptr, out_len)`: the kernel's own verdict on the
//! bootloader that started it, measured, self-reported, refused or without
//! evidence, with the loader's measurement and the root and epoch it was
//! admitted under. A versioned byte record like `MkAttestPolicy`'s, refused
//! into a buffer shorter than the record. Nothing in it identifies the
//! machine, so it needs no capability beyond a valid token.

use super::errnos::{ERRNO_FAULT, ERRNO_NOMEM};
use crate::security::boot::loader_check::boot_attest_record;
use crate::security::boot::loader_check::record::RECORD_LEN;

/// The record's length on success.
pub(super) fn sys_boot_attest(out_ptr: u64, out_len: u64) -> i64 {
    if out_len < RECORD_LEN as u64 {
        return ERRNO_NOMEM;
    }
    let record = boot_attest_record();
    if crate::usercopy::write_user_bytes(out_ptr, &record).is_err() {
        return ERRNO_FAULT;
    }
    RECORD_LEN as i64
}
