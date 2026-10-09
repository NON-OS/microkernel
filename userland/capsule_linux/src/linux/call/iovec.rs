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

//! An iovec array, as readv, writev and the message calls take it, read
//! out of the guest and checked as Linux's import_iovec checks it. Pure,
//! over the guest's memory, so the host proofs hold every refusal.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::memory::Memory;

/// UIO_MAXIOV: Linux refuses a longer array, so a guest cannot have this
/// capsule walk an unbounded list.
pub const IOV_MAX: u64 = 1024;
/// One `struct iovec`: a pointer and a length, both eight bytes.
pub const IOVEC: usize = 16;

/// The `count` (base, len) pairs at `at`: EINVAL past UIO_MAXIOV, EFAULT
/// when the array cannot be read, and EINVAL for a length that is negative
/// as the ssize_t Linux reads it, so no length can wrap a total.
pub fn iovecs(mem: &impl Memory, at: u64, count: u64) -> Result<Vec<(u64, u64)>, i64> {
    match count {
        0 => return Ok(Vec::new()),
        n if n > IOV_MAX => return Err(errno::EINVAL),
        _ => {}
    }
    let raw = mem.read_at(at, count as usize * IOVEC).ok_or(errno::EFAULT)?;
    let word = |b: &[u8]| <[u8; 8]>::try_from(b).map_or(0, u64::from_le_bytes);
    let pieces: Vec<(u64, u64)> =
        raw.chunks_exact(IOVEC).map(|e| (word(&e[..8]), word(&e[8..]))).collect();
    if pieces.iter().any(|&(_, len)| (len as i64) < 0) {
        return Err(errno::EINVAL);
    }
    Ok(pieces)
}

/// readv's and writev's array, for a descriptor that is `open` or not:
/// EBADF for one that is not, before the array is looked at, even an empty
/// one, as Linux looks the descriptor up first.
pub fn vector(mem: &impl Memory, open: bool, at: u64, count: u64) -> Result<Vec<(u64, u64)>, i64> {
    if !open {
        return Err(errno::EBADF);
    }
    iovecs(mem, at, count)
}
