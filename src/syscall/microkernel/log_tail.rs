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

//! `MkLogTail`: the last of what the kernel wrote to its serial console.
//!
//! Most laptops have no serial port, so on them the console's lines went
//! nowhere, and a driver that failed on real hardware said why to nobody.
//! The kernel keeps the last of them in memory (`sys::serial::tail`) and
//! hands them to a caller holding AttestRead, the Terminal's `log`. They are
//! the lines a serial cable would show, nothing more: a Linux run's private
//! output never reaches the console, so it is not here either.

extern crate alloc;

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL};

/// Copies the latest bytes, up to `out_len`, oldest first, to `out_ptr`;
/// returns how many.
pub fn sys_log_tail(out_ptr: u64, out_len: u64) -> i64 {
    if out_ptr == 0 || out_len == 0 {
        return ERRNO_INVAL;
    }
    let want = (out_len as usize).min(crate::sys::serial::tail::KEPT);
    if crate::usercopy::validate_user_write(out_ptr, want).is_err() {
        return ERRNO_FAULT;
    }
    let mut copy = alloc::vec![0u8; want];
    let n = crate::sys::serial::tail::latest(&mut copy);
    match crate::usercopy::write_user_bytes(out_ptr, &copy[..n]) {
        Ok(()) => n as i64,
        Err(_) => ERRNO_FAULT,
    }
}
