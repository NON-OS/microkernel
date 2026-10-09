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

//! `MkDataRead` into the caller's own buffer, through a bounce buffer on
//! the kernel heap taken without panicking: a heap that cannot spare it
//! refuses the call with ENOMEM. The heap wipes it as it frees it
//! (HEAP_ZERO_ON_FREE), so no plaintext outlives the call there.

use alloc::vec::Vec;

use super::super::errnos::{ERRNO_FAULT, ERRNO_NOMEM};
use super::errno::errno;

/// `buf_len` is at most MAX_READ, and `buf` was checked writable for it.
pub(super) fn read_to_caller(name: &[u8], offset: u64, buf: u64, buf_len: u64) -> i64 {
    let mut bounce = Vec::new();
    if bounce.try_reserve_exact(buf_len as usize).is_err() {
        crate::log::warn!("[DATA] read refused: no {} bytes of heap to bounce it", buf_len);
        return ERRNO_NOMEM;
    }
    /*
     * Zeroed a serve unit at a time: the whole call runs with interrupts
     * masked, and 4 MiB of stores in one stretch is long enough under
     * emulation to hold up another cpu's TLB shootdown. The volume read
     * serves once per sector and the copy out once per unit.
     */
    while bounce.len() < buf_len as usize {
        crate::smp::serve_shootdowns();
        let step = (buf_len as usize - bounce.len()).min(crate::smp::SERVE_UNIT);
        bounce.resize(bounce.len() + step, 0u8);
    }
    let n = match crate::fs::blockfs_volume::read_at(name, offset, &mut bounce) {
        Ok(n) => n,
        Err(e) => return errno(e),
    };
    if crate::usercopy::copy_to_user(buf, &bounce[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}
