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

//! The image argument both local-attestation calls take.

use alloc::vec::Vec;

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM};

mod digest;

use digest::DigestError;

/// The BLAKE3 digest of the caller's image, hashed where it lies through a
/// bounce buffer: an image costs the kernel heap one chunk, not its size.
pub(super) fn digest_in(ptr: u64, len: u64) -> Result<[u8; 32], i64> {
    let read = |at: u64, chunk: &mut [u8]| -> Result<(), ()> {
        let addr = ptr.checked_add(at).ok_or(())?;
        crate::usercopy::copy_from_user(addr, chunk).map_err(|_| ())
    };
    match digest::in_chunks(len, crate::smp::SERVE_UNIT, read, crate::smp::serve_shootdowns) {
        Ok(digest) => Ok(digest),
        Err(DigestError::Length) => Err(ERRNO_INVAL),
        Err(DigestError::NoMemory) => Err(ERRNO_NOMEM),
        Err(DigestError::Unreadable) => Err(ERRNO_FAULT),
    }
}

pub(super) fn copy_in(ptr: u64, len: u64, cap: usize) -> Result<Vec<u8>, i64> {
    if len == 0 || len as usize > cap {
        return Err(ERRNO_INVAL);
    }
    let len = len as usize;
    /*
     * A length the caller chose, so the range is checked before anything is
     * allocated for it, and the buffer is taken fallibly: one the heap cannot
     * hold is ENOMEM, where the infallible allocation this was halted the
     * machine.
     */
    if crate::usercopy::validate_user_read(ptr, len).is_err() {
        return Err(ERRNO_FAULT);
    }
    let Ok(mut out) = crate::usercopy::take_buffer(len) else {
        return Err(ERRNO_NOMEM);
    };
    out.resize(len, 0);
    match crate::usercopy::copy_from_user(ptr, &mut out) {
        Ok(()) => Ok(out),
        Err(_) => Err(ERRNO_FAULT),
    }
}

/// A caller may not mint or claim authority it does not itself hold.
pub(super) fn within_own_authority(caps: u64) -> bool {
    let held = crate::capabilities::caps_to_bits(
        &crate::syscall::caps::current_caps_or_default().permissions,
    );
    caps & !held == 0
}
