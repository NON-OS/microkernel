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

//! `MkEnroll(op, kind, in_ptr, in_len, out_ptr, out_len)`: the TPM's half of
//! registering this device, for the capsule that proves. Five operations
//! (`codec`), each through `machine_key::run`. The answer's length, or ENOMEM
//! with nothing written when `out_len` is shorter. Needs DeviceSecret: the EK
//! and the AK identify this machine for as long as its TPM lives.

use alloc::vec::Vec;

use super::answer::answer;
use super::codec::INPUT_MAX;
use super::errors::errno;
use super::request::request;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

pub(in crate::syscall::microkernel) fn sys_enroll(
    op: u64,
    kind: u64,
    in_ptr: u64,
    in_len: u64,
    out_ptr: u64,
    out_len: u64,
) -> i64 {
    if !super::super::gate::device_secret_caller() {
        return ERRNO_PERM;
    }
    if in_len > INPUT_MAX as u64 {
        return ERRNO_INVAL;
    }
    let input = match in_len {
        0 => Vec::new(),
        n => match crate::usercopy::read_user_bytes(in_ptr, n as usize) {
            Ok(v) => v,
            Err(_) => return ERRNO_FAULT,
        },
    };
    let Some(req) = request(op, kind, &input) else {
        return ERRNO_INVAL;
    };
    let answer = match answer(req) {
        Ok(a) => a,
        Err(e) => return errno(e),
    };
    let bytes = answer.bytes();
    if bytes.len() as u64 > out_len {
        return ERRNO_NOMEM;
    }
    match crate::usercopy::write_user_bytes(out_ptr, bytes) {
        Ok(()) => bytes.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}
