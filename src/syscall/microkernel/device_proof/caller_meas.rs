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

//! `MkCallerMeas(pid, out_ptr, out_len)`: the measurement and authority byte
//! the spawn gate recorded for `pid`, 33 bytes, so nonos.prove can name the
//! service a DELIVER came from without taking the caller's word for it. The
//! pid is the sender the kernel wrote on the message. Gated as
//! MkDeviceSecret: the DeviceSecret bit at Vendor authority.

use crate::security::attest_registry::meas_of;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM, ERRNO_SRCH};

const OUT_LEN: usize = 33;

/// 0 with measurement[32] | authority written; ESRCH when `pid` has no
/// registry entry (it never passed a spawn gate, or it exited).
pub(in crate::syscall::microkernel) fn sys_caller_meas(
    pid: u64,
    out_ptr: u64,
    out_len: u64,
) -> i64 {
    if !super::gate::device_secret_caller() {
        return ERRNO_PERM;
    }
    if out_len != OUT_LEN as u64 {
        return ERRNO_INVAL;
    }
    let Ok(pid) = u32::try_from(pid) else {
        return ERRNO_SRCH;
    };
    if crate::usercopy::validate_user_write(out_ptr, OUT_LEN).is_err() {
        return ERRNO_FAULT;
    }
    let Some((meas, authority)) = meas_of(pid) else {
        return ERRNO_SRCH;
    };
    let mut out = [0u8; OUT_LEN];
    out[..32].copy_from_slice(&meas);
    out[32] = authority;
    match crate::usercopy::write_user_bytes(out_ptr, &out) {
        Ok(()) => 0,
        Err(_) => ERRNO_FAULT,
    }
}
