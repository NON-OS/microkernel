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

//! `MkLocalSign`: a trailer for something this machine is installing.

use crate::security::local_build::{sign, LocalBuildError, TRAILER_LEN};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};

use super::local_image::{copy_in, MAX_ELF};

/// `MkLocalSign(elf_ptr, elf_len, caps, out_ptr, out_len)`. An `out_len` of
/// zero asks only for the trailer's length.
pub fn sys_local_sign(elf_ptr: u64, elf_len: u64, caps: u64, out_ptr: u64, out_len: u64) -> i64 {
    if out_len == 0 {
        return TRAILER_LEN as i64;
    }
    if out_len < TRAILER_LEN as u64 {
        return ERRNO_INVAL;
    }
    // The caller cannot prove a right for bytes it does not hold itself.
    let held = crate::capabilities::caps_to_bits(
        &crate::syscall::caps::current_caps_or_default().permissions,
    );
    if caps & !held != 0 {
        return ERRNO_PERM;
    }
    let elf = match copy_in(elf_ptr, elf_len, MAX_ELF) {
        Ok(bytes) => bytes,
        Err(e) => return e,
    };
    let trailer = match sign(&elf, caps) {
        Ok(t) => t,
        Err(e) => return errno(e),
    };
    match crate::usercopy::copy_to_user(out_ptr, &trailer) {
        Ok(()) => trailer.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}

fn errno(e: LocalBuildError) -> i64 {
    match e {
        LocalBuildError::ScarceCapability => ERRNO_PERM,
        LocalBuildError::NoIdentity => ERRNO_NODEV,
        LocalBuildError::ProofFailed | LocalBuildError::TrailerShape => ERRNO_INVAL,
    }
}
