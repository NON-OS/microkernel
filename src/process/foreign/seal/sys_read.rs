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

//! `MkSealRead(seal_no, part, out_ptr, out_len)`: a sealed family, part by
//! part. The record and the measurement list are public; the witness is the
//! prover's alone.

use alloc::vec::Vec;

use super::table::SEALED;
use crate::syscall::microkernel::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_PERM,
};

const RECORD: u64 = 0;
const WITNESS: u64 = 1;
const MEASUREMENTS: u64 = 2;

/// Bytes written, or why not. The witness needs DeviceSecret, which signing
/// keeps to nonos.prove: the histogram and its salt never reach anyone else.
pub(crate) fn sys_seal_read(seal_no: u64, part: u64, out_ptr: u64, out_len: u64) -> i64 {
    if part == WITNESS && !crate::syscall::caps::current_caps_or_default().can_device_secret() {
        return ERRNO_PERM;
    }
    let bytes: Vec<u8> = {
        let sealed = SEALED.lock();
        let Some(s) = sealed.iter().find(|s| s.seal_no == seal_no) else {
            return ERRNO_NOENT;
        };
        match part {
            RECORD => s.record.to_vec(),
            WITNESS => s.witness(),
            MEASUREMENTS => s.measurements(),
            _ => return ERRNO_INVAL,
        }
    };
    if bytes.len() as u64 > out_len {
        return ERRNO_NOMEM;
    }
    match crate::usercopy::write_user_bytes(out_ptr, &bytes) {
        Ok(()) => bytes.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}
