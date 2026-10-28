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

//! The line an installer leaves on why its job failed: which package and
//! which check. Only the process running that listing's job may leave it,
//! and it goes nowhere but init's table (linux_jobs/detail.rs): not to
//! serial, not to disk, not to the log.

use crate::syscall::caps::current_caps_or_default;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use crate::userspace::init::{install_detail, install_tell, INSTALL_DETAIL_MAX as MAX};

/// The failed job's line into the caller's buffer: its length, 0 for none.
pub(super) const READ: u64 = 1;
/// The installer leaves its line; it names no listing, its pid is bound.
pub(super) const TELL: u64 = 2;

pub(super) fn read(listing: &str, buf_ptr: u64, buf_len: u64) -> i64 {
    let mut line = [0u8; MAX];
    let n = install_detail(listing, &mut line);
    if n == 0 {
        return 0;
    }
    if buf_len < n as u64 {
        return ERRNO_INVAL;
    }
    match crate::usercopy::copy_to_user(buf_ptr, &line[..n]) {
        Ok(()) => n as i64,
        Err(_) => ERRNO_FAULT,
    }
}

/// Printable ASCII only: anything else becomes '?', so the line cannot
/// carry control bytes to a terminal or a card.
pub(super) fn tell(listing_len: u64, buf_ptr: u64, buf_len: u64) -> i64 {
    if !current_caps_or_default().can_local_sign() {
        return ERRNO_PERM;
    }
    if listing_len != 0 || buf_len == 0 || buf_len > MAX as u64 {
        return ERRNO_INVAL;
    }
    let mut line = [0u8; MAX];
    let n = buf_len as usize;
    if crate::usercopy::copy_from_user(buf_ptr, &mut line[..n]).is_err() {
        return ERRNO_FAULT;
    }
    line.iter_mut().filter(|b| !(b' '..=b'~').contains(*b)).for_each(|b| *b = b'?');
    let Some(pid) = crate::process::current_pid() else { return ERRNO_PERM };
    match install_tell(pid, &line[..n]) {
        true => 0,
        false => ERRNO_PERM,
    }
}
