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

//! The image and trailer arguments both local-attestation calls take.

use alloc::vec::Vec;

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM};

/// A Linux program with its interpreter is comfortably inside this.
pub(super) const MAX_ELF: usize = 64 << 20;

/*
 * Each copy runs with interrupts off, so a large image crosses in pieces
 * rather than holding the processor for the whole of it.
 */
const CHUNK: usize = 1 << 20;

/// Bring `len` bytes at `ptr` into the kernel, refusing more than `cap`. The
/// buffer is reserved fallibly: a request the heap cannot meet is an errno,
/// not an abort.
pub(super) fn copy_in(ptr: u64, len: u64, cap: usize) -> Result<Vec<u8>, i64> {
    let len = usize::try_from(len).map_err(|_| ERRNO_INVAL)?;
    if len == 0 || len > cap {
        return Err(ERRNO_INVAL);
    }
    let mut out = Vec::new();
    out.try_reserve_exact(len).map_err(|_| ERRNO_NOMEM)?;
    out.resize(len, 0);
    for (i, piece) in out.chunks_mut(CHUNK).enumerate() {
        let at = ptr.checked_add((i * CHUNK) as u64).ok_or(ERRNO_FAULT)?;
        crate::usercopy::copy_from_user(at, piece).map_err(|_| ERRNO_FAULT)?;
    }
    Ok(out)
}
